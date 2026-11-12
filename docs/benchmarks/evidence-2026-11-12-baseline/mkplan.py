import json
import sys

R = "/Users/sebastian/Workspace/ferrixel/.claude/worktrees/agent-a5b9537dab7d91c10"
S = "/private/tmp/claude-501/-Users-sebastian-Workspace-ferrixel--claude-worktrees-validate-issues-labels-v1-7aadd3/7d1a4515-4dbd-40f9-9bc7-0df0d4838873/scratchpad"
B = f"{R}/target/release/examples/jxl_decode_compare"


def jxl(inp, out, reps=7, n=1):
    plan = {"reps": reps, "warmups": 1, "candidates": [
        {"id": f"{m}-{r}", "cmd": [B, inp, m, r, str(n)]}
        for m in ("own", "oxide") for r in ("full", "crop")]}
    json.dump(plan, open(out, "w"), indent=1)


if __name__ == "__main__":
    jxl(sys.argv[1], sys.argv[2], int(sys.argv[3]) if len(sys.argv) > 3 else 7)
