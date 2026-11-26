#!/usr/bin/env python3
"""Writes the sections of the manual, from the xmake-docs repository, as JSON
for `xmake_analyzer stubs --manual`, which puts them in the stubs.

Usage:
    python tools/gen_doc_links.py [OUT]   (default: build/manual.json)
    xmake_analyzer stubs --manual build/manual.json

From the English API pages (docs/api/**.md):
- every `## name` heading becomes a link: the page and the anchor VitePress
  makes of the heading, e.g. `set_license` to xpack-interfaces;
- the `#### Function Prototype` of a heading becomes its signature, e.g.
  `set_default(default: boolean)`;
- the rows of its `#### Parameter Description` table document the parameters.
"""

import json
import sys
from collections.abc import Iterator
import re
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "build" / "manual.json"
REPO = "xmake-io/xmake-docs"
BRANCH = "master"


def fetch(url: str) -> str:
    request = urllib.request.Request(url, headers={"User-Agent": "xmake-analyzer-docs"})
    with urllib.request.urlopen(request, timeout=60) as response:
        return response.read().decode("utf-8")


def slug(heading: str) -> str:
    """The anchor VitePress makes of a heading: `set_license` -> `set-license`."""
    s = heading.strip().lower()
    s = re.sub(r"[\s`~!@#$%^&*()+=\[\]{}|\\;:'\",.<>/?_-]+", "-", s)
    return s.strip("-")


def normalize_prototype(code: str) -> str | None:
    """`set_default(default: <boolean>)` -> `set_default(default: boolean)`;
    a table of options is shortened to its keys: `{defines, languages}`."""
    # The first call of the block, up to its balanced closing parenthesis.
    start = code.find("(")
    if start < 0:
        return None
    depth = 0
    end = None
    for i in range(start, len(code)):
        if code[i] in "({":
            depth += 1
        elif code[i] in ")}":
            depth -= 1
            if depth == 0 and code[i] == ")":
                end = i + 1
                break
    if end is None:
        return None
    proto = code[:end]

    def keys_only(match: re.Match[str]) -> str:
        body = match.group(0)[1:-1]
        keys = re.findall(r"([$\w]+)\s*=", body)
        return "{" + ", ".join(keys) + "}" if keys else "{}"

    # Innermost tables first: `{force = {$flags}}` -> `{force = table}`
    # -> `{force}`.
    proto = re.sub(r"\{[^{}=]*\}", "table", proto)
    while True:
        shorter = re.sub(r"\{[^{}]*=[^{}]*\}", keys_only, proto, flags=re.S)
        if shorter == proto:
            break
        proto = shorter
    proto = re.sub(r"<([^<>]*)>", r"\1", proto)
    proto = re.sub(r"\s+", " ", proto).replace("( ", "(").replace(" )", ")")
    return proto.strip()


def sections(text: str) -> Iterator[tuple[str, str]]:
    """`(name, body)` of every `## name` section."""
    parts = re.split(r"^##\s+", text, flags=re.M)
    for part in parts[1:]:
        heading, _, body = part.partition("\n")
        match = re.fullmatch(r"`?([A-Za-z_][\w.:]*)`?\s*", heading)
        if match and not match.group(1)[0].isupper():
            yield match.group(1), body


def prototype_of(body: str) -> str | None:
    match = re.search(r"####\s+Function Prototype.*?```lua\n(.*?)```", body, flags=re.S)
    return normalize_prototype(match.group(1)) if match else None


def parameters_of(body: str) -> list[tuple[str, str]]:
    match = re.search(r"####\s+Parameter Description\s*\n(.*?)(?:\n#|\Z)", body, flags=re.S)
    if not match:
        return []
    rows: list[tuple[str, str]] = []
    for line in match.group(1).splitlines():
        if not line.strip().startswith("|"):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cells) >= 2 and cells[0] not in ("Parameter", "") and not set(cells[0]) <= set("-: "):
            rows.append((cells[0].strip("`"), cells[1]))
    return rows


def main() -> None:
    tree = json.loads(fetch(f"https://api.github.com/repos/{REPO}/git/trees/{BRANCH}?recursive=1"))
    pages = sorted(
        e["path"] for e in tree["tree"]
        if e["path"].startswith("docs/api/") and e["path"].endswith(".md") and "/zh/" not in e["path"]
    )
    links: set[tuple[str, str, str]] = set()
    prototypes: set[tuple[str, str, str]] = set()
    params: set[tuple[str, str, int, str, str]] = set()
    for path in pages:
        text = fetch(f"https://raw.githubusercontent.com/{REPO}/{BRANCH}/{path}")
        page = path[len("docs/"):-len(".md")]  # api/description/project-target
        # A page of one builtin (`ipairs`) or one module (`core.base.json`):
        # the page itself, without anchor.
        for prefix in ("api/scripts/builtin-modules/", "api/scripts/extension-modules/"):
            if page.startswith(prefix):
                name = page[len(prefix):].replace("/", ".")
                if re.fullmatch(r"[a-z_][\w.]*", name):
                    links.add((name, page, ""))
        for name, body in sections(text):
            links.add((name, page, slug(name)))
            prototype = prototype_of(body)
            if prototype:
                prototypes.add((name, page, prototype))
            for index, (param, description) in enumerate(parameters_of(body)):
                params.add((name, page, index, param, description))

    out = Path(sys.argv[1]) if len(sys.argv) > 1 else OUT
    out.parent.mkdir(parents=True, exist_ok=True)
    manual = {
        # `(name, page, anchor)`: the `## name` headings of the API pages.
        "links": [list(e) for e in sorted(links)],
        # `(name, page, prototype)`: the `Function Prototype` of the sections.
        "prototypes": [list(e) for e in sorted(prototypes)],
        # `(name, page, parameter, description)`, in the order of the parameters.
        "params": [[n, p, a, d] for n, p, _, a, d in sorted(params)],
    }
    out.write_text(json.dumps(manual, ensure_ascii=False, indent=1), encoding="utf-8", newline="\n")
    print(f"wrote {out}: {len(links)} links, {len(prototypes)} prototypes, {len(params)} parameters from {len(pages)} pages")

if __name__ == "__main__":
    main()
