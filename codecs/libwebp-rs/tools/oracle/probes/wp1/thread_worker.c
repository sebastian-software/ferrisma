// WP1 probe: runs the worker of src/utils/thread_utils.h through one named
// scenario and prints the state after each step. Every step prints, in this
// order: `LABEL.ret` when the call returns a value, `LABEL.status`,
// `LABEL.had_error` and `LABEL.counter`, where counter is the value that the
// hook adds to. The scenarios are chosen so that the printed states do not
// depend on thread timing; tests/wp1_thread.rs runs the same steps on the Rust
// worker and compares the traces.
//
// Usage: thread_worker SCENARIO

#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "probe_support.h"
#include "src/utils/thread_utils.h"

static int AddHook(void* data1, void* data2) {
  *(int*)data1 += *(int*)data2;
  return 1;
}

static int FailHook(void* data1, void* data2) {
  *(int*)data1 += *(int*)data2;
  return 0;
}

// Sleeps first, so that a worker that was launched is still working when the
// next call runs.
static int SlowAddHook(void* data1, void* data2) {
  usleep(20000);
  *(int*)data1 += *(int*)data2;
  return 1;
}

static const WebPWorkerInterface* g_interface;
static int counter;
static int value;

static void SetJob(WebPWorker* worker, WebPWorkerHook hook) {
  worker->hook = hook;
  worker->data1 = &counter;
  worker->data2 = &value;
}

static void Trace(const char* label, const WebPWorker* worker, int has_ret, int ret) {
  if (has_ret) printf("%s.ret=%d\n", label, ret);
  printf("%s.status=%d\n", label, (int)worker->status);
  printf("%s.had_error=%d\n", label, worker->had_error);
  printf("%s.counter=%d\n", label, counter);
}

static void Sync(WebPWorker* worker, const char* label) {
  Trace(label, worker, 1, g_interface->Sync(worker));
}

static void Reset(WebPWorker* worker, const char* label) {
  Trace(label, worker, 1, g_interface->Reset(worker));
}

static void Plain(WebPWorker* worker, const char* label) {
  Trace(label, worker, 0, 0);
}

static void ScenarioSyncIdle(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  Sync(&worker, "sync");
  g_interface->End(&worker);
}

static void ScenarioLaunchWithoutReset(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  g_interface->Launch(&worker);
  Plain(&worker, "launch");
  Sync(&worker, "sync");
  g_interface->End(&worker);
}

static void ScenarioResetLaunchSync(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  Reset(&worker, "reset");
  g_interface->Launch(&worker);
  Sync(&worker, "sync");
  g_interface->End(&worker);
  Plain(&worker, "end");
}

static void ScenarioSyncAfterFailingHook(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, FailHook);
  Reset(&worker, "reset1");
  g_interface->Launch(&worker);
  Sync(&worker, "sync1");
  Reset(&worker, "reset2");
  Sync(&worker, "sync2");
  g_interface->End(&worker);
  Plain(&worker, "end");
}

static void ScenarioResetInEachState(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, SlowAddHook);
  // NOT_OK: the thread starts.
  Reset(&worker, "reset_idle");
  // OK: nothing starts again.
  Reset(&worker, "reset_ok");
  // WORK: Reset syncs the launched job first. The hook succeeds, so the result
  // does not depend on whether the thread has already finished.
  g_interface->Launch(&worker);
  Reset(&worker, "reset_work");
  g_interface->End(&worker);
  Plain(&worker, "end");
}

static void ScenarioEndNeverLaunched(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  g_interface->End(&worker);
  Plain(&worker, "end_idle");
  Reset(&worker, "reset");
  g_interface->End(&worker);
  Plain(&worker, "end");
}

static void ScenarioExecuteFailing(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, FailHook);
  g_interface->Execute(&worker);
  Plain(&worker, "execute");
  Sync(&worker, "sync");
  g_interface->End(&worker);
}

static void ScenarioExecuteSucceeding(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  g_interface->Execute(&worker);
  Plain(&worker, "execute");
  g_interface->End(&worker);
}

static void ScenarioLaunchNullHook(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  Reset(&worker, "reset");
  g_interface->Launch(&worker);
  Sync(&worker, "sync");
  g_interface->End(&worker);
}

static void ScenarioSyncWithoutLaunch(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  Reset(&worker, "reset");
  Sync(&worker, "sync");
  g_interface->End(&worker);
}

static void ScenarioTwoLaunches(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, AddHook);
  Reset(&worker, "reset");
  g_interface->Launch(&worker);
  Sync(&worker, "sync1");
  g_interface->Launch(&worker);
  Sync(&worker, "sync2");
  g_interface->End(&worker);
  Plain(&worker, "end");
}

static void ScenarioFailingThenReset(void) {
  WebPWorker worker;
  g_interface->Init(&worker);
  SetJob(&worker, FailHook);
  Reset(&worker, "reset1");
  g_interface->Launch(&worker);
  Sync(&worker, "sync1");
  // The error flag stays set until the next Reset, so a good job still fails.
  SetJob(&worker, AddHook);
  g_interface->Launch(&worker);
  Sync(&worker, "sync2");
  Reset(&worker, "reset2");
  g_interface->Launch(&worker);
  Sync(&worker, "sync3");
  g_interface->End(&worker);
  Plain(&worker, "end");
}

int main(int argc, char* argv[]) {
  const char* scenario;
  ProbeCheckArgs(argc, 1, 1, "thread_worker SCENARIO");
  scenario = argv[1];
  g_interface = WebPGetWorkerInterface();
  counter = 0;
  value = 5;
  if (strcmp(scenario, "sync_idle") == 0) {
    ScenarioSyncIdle();
  } else if (strcmp(scenario, "launch_without_reset") == 0) {
    ScenarioLaunchWithoutReset();
  } else if (strcmp(scenario, "reset_launch_sync") == 0) {
    ScenarioResetLaunchSync();
  } else if (strcmp(scenario, "sync_after_failing_hook") == 0) {
    ScenarioSyncAfterFailingHook();
  } else if (strcmp(scenario, "reset_in_each_state") == 0) {
    ScenarioResetInEachState();
  } else if (strcmp(scenario, "end_never_launched") == 0) {
    ScenarioEndNeverLaunched();
  } else if (strcmp(scenario, "execute_failing") == 0) {
    ScenarioExecuteFailing();
  } else if (strcmp(scenario, "execute_succeeding") == 0) {
    ScenarioExecuteSucceeding();
  } else if (strcmp(scenario, "launch_null_hook") == 0) {
    ScenarioLaunchNullHook();
  } else if (strcmp(scenario, "sync_without_launch") == 0) {
    ScenarioSyncWithoutLaunch();
  } else if (strcmp(scenario, "two_launches") == 0) {
    ScenarioTwoLaunches();
  } else if (strcmp(scenario, "failing_then_reset") == 0) {
    ScenarioFailingThenReset();
  } else {
    fprintf(stderr, "thread_worker: unknown scenario %s\n", scenario);
    return 2;
  }
  return 0;
}
