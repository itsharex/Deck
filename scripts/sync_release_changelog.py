#!/usr/bin/env python3
"""
同步 GitHub Release 到仓库根目录 CHANGELOG.md。
- incremental：从 GITHUB_EVENT_PATH 读取 release 事件，仅处理当前 tag（不调用 Releases 列表 API）。
- full：分页拉取全部 Release；已存在的 tag 保留原文，仅补全缺失条目，并按发布时间倒序排列。
"""
from __future__ import annotations

import json
import os
import re
import sys
import urllib.request
from dataclasses import dataclass
from datetime import datetime
from typing import Any

CHANGELOG_FILENAME = "CHANGELOG.md"
MARKER_AUTO = "<!-- release-changelog-bot:auto -->"
TAG_PREFIX = "<!-- release-changelog-bot:tag:"
GITHUB_API = "https://api.github.com"


@dataclass
class ReleaseView:
    tag: str
    title: str
    body: str
    published_at: str
    assets: list[tuple[str, str]]


def _http_get_links(resp: Any) -> dict[str, str]:
    raw = resp.headers.get("Link", "")
    links: dict[str, str] = {}
    for part in raw.split(","):
        m = re.search(r'<([^>]+)>;\s*rel="(\w+)"', part.strip())
        if m:
            links[m.group(2)] = m.group(1)
    return links


def fetch_all_releases(repo: str, token: str) -> list[dict[str, Any]]:
    out: list[dict[str, Any]] = []
    url = f"{GITHUB_API}/repos/{repo}/releases?per_page=100"
    while url:
        req = urllib.request.Request(
            url,
            headers={
                "Accept": "application/vnd.github+json",
                "Authorization": f"Bearer {token}",
                "X-GitHub-Api-Version": "2022-11-28",
                "User-Agent": "release-changelog-bot",
            },
        )
        with urllib.request.urlopen(req, timeout=120) as resp:
            page = json.loads(resp.read().decode("utf-8"))
            links = _http_get_links(resp)
            out.extend(page)
            url = links.get("next")
    return out


def release_from_api_obj(obj: dict[str, Any]) -> ReleaseView | None:
    if obj.get("draft"):
        return None
    tag = obj.get("tag_name") or ""
    if not tag:
        return None
    assets: list[tuple[str, str]] = []
    for a in obj.get("assets") or []:
        name = a.get("name") or ""
        u = a.get("browser_download_url") or ""
        if name and u:
            assets.append((name, u))
    return ReleaseView(
        tag=tag,
        title=(obj.get("name") or "").strip() or tag,
        body=(obj.get("body") or "").strip(),
        published_at=obj.get("published_at") or "",
        assets=assets,
    )


def release_from_event_payload(payload: dict[str, Any]) -> ReleaseView | None:
    rel = payload.get("release") or {}
    if not isinstance(rel, dict):
        return None
    return release_from_api_obj(rel)


def format_block(r: ReleaseView) -> str:
    lines: list[str] = [
        f"{TAG_PREFIX}{r.tag} -->",
        f"## {r.tag} — {r.title}",
        "",
        f"- **Tag:** `{r.tag}`",
        f"- **Published:** {r.published_at or '_unknown_'}",
        "",
        "### Release notes",
        "",
        r.body if r.body else "_（无正文）_",
        "",
        "### Assets",
        "",
    ]
    if r.assets:
        for name, url in r.assets:
            lines.append(f"- [`{name}`]({url})")
    else:
        lines.append("_（无附件）_")
    lines.append("")
    return "\n".join(lines)


def default_preamble() -> str:
    return "\n".join(
        [
            "# GitHub Releases Changelog",
            "",
            "本文件由 [release-changelog-bot](.github/workflows/release-changelog-bot.yml) 根据 GitHub Release 自动生成与增量更新；**请勿手动修改各版本条目**（可修改本说明文字）。",
            "",
            MARKER_AUTO,
            "",
        ]
    )


def parse_blocks_from_body(body: str) -> dict[str, str]:
    blocks: dict[str, str] = {}
    pattern = re.compile(
        re.escape(TAG_PREFIX) + r"(.+?) -->\r?\n(.*?)(?=" + re.escape(TAG_PREFIX) + r"|\Z)",
        re.DOTALL,
    )
    for m in pattern.finditer(body.strip() + "\n"):
        tag = m.group(1).strip()
        blocks[tag] = m.group(0).rstrip() + "\n"
    return blocks


def parse_existing(path: str) -> tuple[str, dict[str, str]]:
    if not os.path.isfile(path):
        return default_preamble(), {}
    raw = open(path, encoding="utf-8").read()
    if MARKER_AUTO in raw:
        head, rest = raw.split(MARKER_AUTO, 1)
        preamble = head + MARKER_AUTO + "\n\n"
        rest = rest.lstrip("\n")
        blocks = parse_blocks_from_body(rest)
        return preamble, blocks
    preamble = default_preamble()
    blocks = parse_blocks_from_body(raw)
    return preamble, blocks


def published_at_from_block(block: str) -> str:
    m = re.search(r"^\-\s\*\*Published:\*\*\s*(.+)$", block, re.MULTILINE)
    if not m:
        return ""
    s = m.group(1).strip()
    if s == "_unknown_":
        return ""
    return s


def parse_iso(dt: str) -> datetime:
    if not dt:
        return datetime.min.replace(tzinfo=None)
    try:
        d = datetime.fromisoformat(dt.replace("Z", "+00:00"))
        return d.replace(tzinfo=None)
    except ValueError:
        return datetime.min.replace(tzinfo=None)


def sort_blocks(blocks: dict[str, str]) -> list[str]:
    tags = list(blocks.keys())
    tags.sort(
        key=lambda t: parse_iso(published_at_from_block(blocks[t])),
        reverse=True,
    )
    return [blocks[t] for t in tags]


def write_changelog(path: str, preamble: str, ordered_blocks: list[str]) -> None:
    body = preamble.rstrip() + "\n\n"
    body += "\n".join(b.rstrip() + "\n" for b in ordered_blocks if b.strip())
    if not body.endswith("\n"):
        body += "\n"
    open(path, "w", encoding="utf-8").write(body)


def run_incremental(repo_root: str) -> bool:
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    if not event_path or not os.path.isfile(event_path):
        print("incremental 模式需要 GITHUB_EVENT_PATH", file=sys.stderr)
        sys.exit(1)
    payload = json.load(open(event_path, encoding="utf-8"))
    rel = release_from_event_payload(payload)
    if rel is None:
        print("跳过：草稿/无 tag", file=sys.stderr)
        return False
    path = os.path.join(repo_root, CHANGELOG_FILENAME)
    preamble, blocks = parse_existing(path)
    if rel.tag in blocks:
        print(f"已存在 {rel.tag}，跳过（去重）")
        return False
    blocks[rel.tag] = format_block(rel)
    write_changelog(path, preamble, sort_blocks(blocks))
    print(f"已追加 Release: {rel.tag}")
    return True


def run_full(repo: str, repo_root: str, token: str) -> bool:
    path = os.path.join(repo_root, CHANGELOG_FILENAME)
    preamble, blocks = parse_existing(path)
    api_objs = fetch_all_releases(repo, token)
    views = [v for o in api_objs if (v := release_from_api_obj(o)) is not None]
    changed = False
    for v in views:
        if v.tag not in blocks:
            blocks[v.tag] = format_block(v)
            changed = True
    if not changed:
        print("全量扫描：无缺失 Release，文件未修改")
        return False
    write_changelog(path, preamble, sort_blocks(blocks))
    print(f"全量扫描完成，当前共 {len(blocks)} 条 Release 记录")
    return True


def main() -> None:
    mode = os.environ.get("SYNC_MODE", "").strip().lower()
    token = os.environ.get("GITHUB_TOKEN", "").strip()
    repo = os.environ.get("GITHUB_REPOSITORY", "").strip()
    repo_root = os.environ.get("GITHUB_WORKSPACE", os.getcwd()).strip()

    if not token or not repo:
        print("需要环境变量 GITHUB_TOKEN 与 GITHUB_REPOSITORY", file=sys.stderr)
        sys.exit(1)
    if mode not in ("incremental", "full"):
        print("SYNC_MODE 须为 incremental 或 full", file=sys.stderr)
        sys.exit(1)

    if mode == "incremental":
        changed = run_incremental(repo_root)
    else:
        changed = run_full(repo, repo_root, token)

    flag = os.path.join(repo_root, ".release-changelog-changed")
    if changed:
        open(flag, "w", encoding="utf-8").write("1")
    elif os.path.isfile(flag):
        os.remove(flag)


if __name__ == "__main__":
    main()
