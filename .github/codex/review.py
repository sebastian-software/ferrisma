"""Run a restricted, subscription-authenticated Codex review."""

import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory


ROOT = Path(__file__).resolve().parent
MODELS = {"sol": "gpt-6.2-sol", "astra": "gpt-6.2-astra"}
CHATGPT_ONLY = 'forced_login_method="chatgpt"'
ALLOWED_ENV = {"HOME", "USER", "LOGNAME", "PATH", "TMPDIR", "LANG", "CODEX_HOME"}


def codex_env():
    return {name: value for name, value in os.environ.items() if name in ALLOWED_ENV}


def require_chatgpt_login():
    result = subprocess.run(
        ["codex", "-c", CHATGPT_ONLY, "login", "status"],
        env=codex_env(), capture_output=True, text=True, timeout=20, check=False,
    )
    if result.returncode or "logged in using chatgpt" not in (result.stdout + result.stderr).lower():
        raise RuntimeError("Sign in with ChatGPT using `codex login` on the watcher host")


def run_model(stage, diff, guidance, previous=""):
    prompt = (ROOT / "review-prompt.md").read_text(encoding="utf-8")
    prompt += "\n" + json.dumps({"base_guidance": guidance, "diff": diff, "prior_findings": previous})
    with TemporaryDirectory(prefix="codex-pr-review-") as workspace:
        workspace = Path(workspace).resolve()
        result = workspace / "result.json"
        args = [
            "codex", "exec", "--ignore-user-config", "--ignore-rules", "--ephemeral",
            "--skip-git-repo-check", "--strict-config", "--sandbox", "read-only",
            "--disable", "shell_tool", "--disable", "view_image", "--disable", "image_generation",
            "--disable", "hooks", "--disable", "plugins", "--disable", "remote_plugin",
            "-c", CHATGPT_ONLY, "-c", 'approval_policy="never"', "-c", 'web_search="disabled"',
            "--model", MODELS[stage], "--output-schema", str(ROOT / "review-schema.json"),
            "--output-last-message", str(result), "-",
        ]
        process = subprocess.run(
            args, input=prompt, cwd=workspace, env=codex_env(), text=True,
            capture_output=True, timeout=600, check=False,
        )
        if process.returncode:
            raise RuntimeError(f"{stage.title()} Codex run failed. Check Codex login, model access and allowance.")
        return load_findings(result)


def load_findings(path):
    result = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(result, dict) or result.get("complete") is not True:
        raise ValueError("Codex did not return a complete review")
    findings = result.get("findings")
    if not isinstance(findings, list) or len(findings) > 25:
        raise ValueError("Codex returned an invalid findings list")
    for finding in findings:
        if (
            not isinstance(finding, dict)
            or not isinstance(finding.get("path"), str)
            or not finding["path"].strip()
            or type(finding.get("line")) is not int
            or finding["line"] < 0
            or not isinstance(finding.get("body"), str)
            or not 0 < len(finding["body"].strip()) <= 1_500
        ):
            raise ValueError("Codex returned an invalid finding")
    return findings


def review_payload(stage, base, head, findings):
    model = {"sol": "GPT-6.2 Sol", "astra": "GPT-6.2 Astra"}[stage]
    marker = f"<!-- ferrisma-codex-review:{stage}:{base} -->"
    if not findings:
        if stage != "astra":
            raise ValueError("Only Astra can approve")
        return {
            "commit_id": head, "event": "APPROVE",
            "body": f"{marker}\n{model} completed the review and found no blockers.",
        }

    lines = [marker, f"{model} found {len(findings)} blocking issue(s).", ""]
    comments = []
    for finding in findings:
        path, line = finding["path"], finding["line"]
        body = finding["body"].strip().replace("@", "@\u200b")
        location = f"{path}:{line}" if line else path
        location = " ".join(location.split()).replace("`", "'").replace("@", "@\u200b")
        lines.append(f"- `{location}` {body}")
        if line:
            comments.append({"path": path, "line": line, "side": "RIGHT", "body": body})
    return {
        "commit_id": head, "event": "REQUEST_CHANGES",
        "body": "\n".join(lines), "comments": comments,
    }
