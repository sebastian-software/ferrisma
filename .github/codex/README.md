# Codex review pilot

Add a repository Actions secret named `OPENAI_API_KEY` and allow GitHub Actions
to create and approve pull requests in the repository's Actions settings. The
organization policy must permit the approval setting. A maintainer can then
create and apply the `codex-review` label to a ready pull request targeting the
default branch. Removing the label stops reviews on subsequent pushes.

GitHub-hosted Actions run GPT-6.2 Sol on the proposed diff. A blocking Sol
review requests changes. A clean Sol review sends the same diff to GPT-6.2
Astra. On a new commit, Sol runs first and Astra rechecks its open findings.
Only a successfully published Astra approval sets `Astra Review (pilot)` to
success for the reviewed commit. OpenAI usage is billed to the API project.

The pilot status is optional. Leave it optional while using a PR label to
opt in. A workflow error or a diff over 400,000 bytes needs manual attention.
The workflow reads the proposed Git diff from a trusted base checkout and
does not run the PR's code or tests.
