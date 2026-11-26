# Local Codex review pilot

Run this watcher on an always-on macOS or Linux host with Python 3.9 or later,
OpenSSL, and a current Codex CLI. Sign in with ChatGPT using `codex login`
under the OS account that runs the watcher. Sol and Astra use that account's
Codex allowance. The watcher stops at startup if it can't confirm ChatGPT login.

Create a GitHub App, turn off its webhook, and grant repository access to
Ferrisma. It needs **Contents** read, **Pull requests** write, and **Commit
statuses** write. Keep its private key on the watcher host with permissions
restricted to the watcher account. The app signs GitHub requests separately
from the Codex CLI, so it can review a pull request opened by the person whose
Codex allowance it uses.

```sh
python3 -B .github/codex/watch.py --once \
  --repo sebastian-software/ferrisma \
  --app-id YOUR_GITHUB_APP_CLIENT_ID \
  --private-key /path/to/github-app.pem
```

Remove `--once` for a scan every minute when idle, or keep it and schedule the
command with the host's scheduler. One process scans all `--repo` arguments.
Reviews run serially, so a long review can delay the next scan. The watcher
remembers completed commits in `~/.local/state/ferrisma-codex-review/` and
recovers published reviews from GitHub after a restart.

Create and apply the `codex-review` label to a ready PR targeting the default
branch. Sol checks each new head commit. When Sol clears it, Astra checks the
same diff and rechecks any open Astra findings. A blocker from either model
requests changes. An Astra approval sets `Astra Review (pilot)` to success for
that commit. Leave the status optional while piloting on selected PRs.

Codex receives the text diff and `AGENTS.md` from the base commit. The CLI runs
with shell execution and web search disabled. GitHub credentials stay outside
the Codex process. A diff over 400,000 bytes needs a human review. Hosted code
and API billing settings are unnecessary for the watcher.
