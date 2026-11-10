//! WP1 differential test: the worker of `src/utils/thread_utils.c` (`Init`,
//! `Reset`, `Sync`, `Launch`, `Execute` and `End`, with the `NOT_OK`, `OK` and
//! `WORK` states and the error flag). The probe
//! `tools/oracle/probes/wp1/thread_worker.c` runs twelve named scenarios in C;
//! this file runs the same steps on the Rust worker and compares the traces.
//!
//! The Rust worker owns its job while it works, so the counter that the hook
//! adds to is read back from the job when the worker is not working, and the
//! job keeps the counter between runs. The C probe shares one counter variable,
//! and each trace prints the same fields in the same order.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. `Reset` while the worker is `WORK` and the hook fails. In C the status is
//!    read without the lock, so the result depends on timing. The success
//!    variant is compared; the failure variant is not.

#[path = "support/oracle.rs"]
mod oracle;

use std::time::Duration;

use oracle::{Fields, Oracle};
use webp::utils::thread_utils::{WebPWorker, WebPWorkerHook, WebPWorkerStatus};

/// Adds the value to the counter, returns success.
fn add_hook(counter: &mut i32, value: &mut i32) -> bool {
    *counter += *value;
    true
}

/// Adds the value to the counter, returns failure.
fn fail_hook(counter: &mut i32, value: &mut i32) -> bool {
    *counter += *value;
    false
}

/// Sleeps first, so that a launched worker is still working for the next call.
fn slow_add_hook(counter: &mut i32, value: &mut i32) -> bool {
    std::thread::sleep(Duration::from_millis(20));
    *counter += *value;
    true
}

/// The state of one scenario: the worker, the counter and the printed fields.
struct Run {
    worker: WebPWorker<i32, i32>,
    counter: i32,
    fields: Fields,
}

impl Run {
    fn new() -> Self {
        Self {
            worker: WebPWorker::init(),
            counter: 0,
            fields: Fields::new(),
        }
    }

    /// Sets the hook with the shared counter and the value 5, as the probe does.
    fn set_job(&mut self, hook: WebPWorkerHook<i32, i32>) {
        self.worker.set_job(hook, self.counter, 5);
    }

    /// Records the counter, which the job holds while the worker is idle.
    fn read_counter(&mut self) {
        if let Some((counter, _)) = self.worker.job_data() {
            self.counter = *counter;
        }
    }

    fn trace(&mut self, label: &str, ret: Option<bool>) {
        self.read_counter();
        if let Some(ret) = ret {
            self.fields =
                std::mem::take(&mut self.fields).with(&format!("{label}.ret"), i32::from(ret));
        }
        let status = self.worker.status().0;
        let had_error = i32::from(self.worker.had_error());
        self.fields = std::mem::take(&mut self.fields)
            .with(&format!("{label}.status"), status)
            .with(&format!("{label}.had_error"), had_error)
            .with(&format!("{label}.counter"), self.counter);
    }

    fn reset(&mut self, label: &str) {
        let ret = self.worker.reset();
        self.trace(label, Some(ret));
    }

    fn sync(&mut self, label: &str) {
        let ret = self.worker.sync();
        self.trace(label, Some(ret));
    }

    fn plain(&mut self, label: &str) {
        self.trace(label, None);
    }
}

fn run_scenario(name: &str) -> Fields {
    let mut run = Run::new();
    match name {
        "sync_idle" => {
            run.set_job(add_hook);
            run.sync("sync");
            run.worker.end();
        }
        "launch_without_reset" => {
            run.set_job(add_hook);
            run.worker.launch();
            run.plain("launch");
            run.sync("sync");
            run.worker.end();
        }
        "reset_launch_sync" => {
            run.set_job(add_hook);
            run.reset("reset");
            run.worker.launch();
            run.sync("sync");
            run.worker.end();
            run.plain("end");
        }
        "sync_after_failing_hook" => {
            run.set_job(fail_hook);
            run.reset("reset1");
            run.worker.launch();
            run.sync("sync1");
            run.reset("reset2");
            run.sync("sync2");
            run.worker.end();
            run.plain("end");
        }
        "reset_in_each_state" => {
            run.set_job(slow_add_hook);
            run.reset("reset_idle");
            run.reset("reset_ok");
            run.worker.launch();
            run.reset("reset_work");
            run.worker.end();
            run.plain("end");
        }
        "end_never_launched" => {
            run.set_job(add_hook);
            run.worker.end();
            run.plain("end_idle");
            run.reset("reset");
            run.worker.end();
            run.plain("end");
        }
        "execute_failing" => {
            run.set_job(fail_hook);
            run.worker.execute();
            run.plain("execute");
            run.sync("sync");
            run.worker.end();
        }
        "execute_succeeding" => {
            run.set_job(add_hook);
            run.worker.execute();
            run.plain("execute");
            run.worker.end();
        }
        "launch_null_hook" => {
            run.reset("reset");
            run.worker.launch();
            run.sync("sync");
            run.worker.end();
        }
        "sync_without_launch" => {
            run.set_job(add_hook);
            run.reset("reset");
            run.sync("sync");
            run.worker.end();
        }
        "two_launches" => {
            run.set_job(add_hook);
            run.reset("reset");
            run.worker.launch();
            run.sync("sync1");
            run.worker.launch();
            run.sync("sync2");
            run.worker.end();
            run.plain("end");
        }
        "failing_then_reset" => {
            run.set_job(fail_hook);
            run.reset("reset1");
            run.worker.launch();
            run.sync("sync1");
            // The error flag stays set until the next reset, so a good job still fails.
            run.set_job(add_hook);
            run.worker.launch();
            run.sync("sync2");
            run.reset("reset2");
            run.worker.launch();
            run.sync("sync3");
            run.worker.end();
            run.plain("end");
        }
        other => panic!("no Rust scenario named {other}"),
    }
    run.fields
}

const SCENARIOS: [&str; 12] = [
    "sync_idle",
    "launch_without_reset",
    "reset_launch_sync",
    "sync_after_failing_hook",
    "reset_in_each_state",
    "end_never_launched",
    "execute_failing",
    "execute_succeeding",
    "launch_null_hook",
    "sync_without_launch",
    "two_launches",
    "failing_then_reset",
];

#[test]
fn worker_traces_match_the_c_worker_in_every_scenario() {
    let Some(oracle) = Oracle::locate("worker_traces_match_the_c_worker_in_every_scenario") else {
        return;
    };
    let probe = oracle.probe("wp1", "thread_worker");
    for scenario in SCENARIOS {
        let c_trace = probe.run(&[scenario], &[]).success().fields();
        assert_eq!(c_trace, run_scenario(scenario), "scenario {scenario}");
    }
}

/// The states that the Rust worker reports, without the oracle: a fresh worker
/// is `NOT_OK`, a reset worker is `OK`, and a launched worker reports `OK` again
/// after `sync`.
#[test]
fn worker_status_follows_the_c_states() {
    let mut worker: WebPWorker<i32, i32> = WebPWorker::init();
    assert_eq!(worker.status(), WebPWorkerStatus::NOT_OK);
    assert!(worker.reset());
    assert_eq!(worker.status(), WebPWorkerStatus::OK);
    worker.set_job(add_hook, 0, 2);
    worker.launch();
    assert!(worker.sync());
    assert_eq!(worker.status(), WebPWorkerStatus::OK);
    assert_eq!(worker.job_data().map(|(counter, _)| *counter), Some(2));
    worker.end();
    assert_eq!(worker.status(), WebPWorkerStatus::NOT_OK);
}
