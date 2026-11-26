"""GitHub API access as a dedicated review app."""

import base64
from datetime import datetime, timezone
import json
import re
import subprocess
import time
from urllib.error import HTTPError
from urllib.request import HTTPRedirectHandler, Request, build_opener


class GitHubError(RuntimeError):
    def has_status(self, status):
        return f"HTTP {status}" in str(self)


class NoRedirect(HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        return None


API_ROOT = "https://api.github.com/"
OPENER = build_opener(NoRedirect)
NEXT_PAGE = re.compile(r'<([^>]+)>;\s*rel="next"')


def api(token, path, *, payload=None, paginate=False, raw=False):
    url = API_ROOT + path
    data = json.dumps(payload).encode() if payload is not None else None
    headers = {
        "Accept": "application/vnd.github.diff" if raw else "application/vnd.github+json",
        "Authorization": f"Bearer {token}",
        "Content-Type": "application/json",
        "User-Agent": "ferrisma-codex-review",
        "X-GitHub-Api-Version": "2026-03-10",
    }
    pages = []
    while url:
        if not url.startswith(API_ROOT):
            raise GitHubError("GitHub returned an unexpected pagination URL")
        try:
            with OPENER.open(Request(url, data=data, headers=headers), timeout=45) as response:
                body = response.read()
                link = NEXT_PAGE.search(response.headers.get("Link", ""))
        except HTTPError as error:
            try:
                message = json.load(error).get("message", error.reason)
            except ValueError:
                message = error.reason
            raise GitHubError(f"{path}: {message} (HTTP {error.code})") from None
        if raw:
            return body.decode("utf-8", errors="replace")
        result = json.loads(body)
        if not paginate:
            return result
        pages.extend(result)
        url = link.group(1) if link else None
    return pages


def base64url(data):
    return base64.urlsafe_b64encode(data).rstrip(b"=")


class GitHubApp:
    def __init__(self, app_id, private_key):
        self.app_id = app_id
        self.private_key = private_key
        self.installations = {}
        self.tokens = {}
        self.login = api(self.jwt(), "app")["slug"] + "[bot]"

    def jwt(self):
        now = int(time.time())
        header = base64url(b'{"alg":"RS256","typ":"JWT"}')
        claims = base64url(json.dumps({"iat": now - 60, "exp": now + 480, "iss": self.app_id}).encode())
        message = header + b"." + claims
        signature = subprocess.run(
            ["openssl", "dgst", "-sha256", "-sign", str(self.private_key)],
            input=message, capture_output=True, timeout=10, check=True,
        ).stdout
        return (message + b"." + base64url(signature)).decode()

    def token(self, repo):
        if repo not in self.installations:
            self.installations[repo] = api(self.jwt(), f"repos/{repo}/installation")["id"]
        installation = self.installations[repo]
        token, expires = self.tokens.get(installation, (None, 0))
        if expires - time.time() < 60:
            grant = api(self.jwt(), f"app/installations/{installation}/access_tokens", payload={})
            token = grant["token"]
            expires = datetime.fromisoformat(grant["expires_at"].replace("Z", "+00:00"))
            self.tokens[installation] = token, expires.astimezone(timezone.utc).timestamp()
        return token

    def request(self, repo, path, **kwargs):
        return api(self.token(repo), f"repos/{repo}/{path}", **kwargs)
