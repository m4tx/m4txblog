#!/usr/bin/env python3
"""Imports the approved comments from the old WordPress database.

The comments are mapped to the new posts using `legacy_redirects.toml`
(see `generate_legacy_redirects.py`) and inserted into the blog's SQLite
database. The original WordPress comment IDs are kept, so that old links to
`#comment-123` keep working after the redirect.

To minimize the amount of stored personal data, only the data that was
already public is imported: the author's name, website, the date, and the
content. E-mail and IP addresses are not imported.

The script can be run multiple times (e.g. once more at the moment of the
migration, to include the latest comments); already imported comments are
updated. It refuses to overwrite comments that were not imported from
WordPress.

The blog's migrations need to be applied first (e.g. by starting the server
once).

Usage (requires `pymysql`):

    WP_DB_PASSWORD=... ./scripts/import_wp_comments.py path/to/db.sqlite3

Other connection settings can be overridden with the `WP_DB_HOST`,
`WP_DB_PORT`, `WP_DB_USER`, and `WP_DB_NAME` environment variables.
"""

import os
import re
import sqlite3
import sys
import tomllib
from datetime import timezone
from html.parser import HTMLParser
from pathlib import Path

import pymysql

REPO_ROOT = Path(__file__).resolve().parent.parent
LEGACY_REDIRECTS = REPO_ROOT / "legacy_redirects.toml"
TABLE = "m4txblog__comment"

# (site code, table prefix, ID offset). The English site's comment IDs would
# collide with the Polish ones, so they're moved to a separate range (but
# there are no approved comments there anyway).
SITES = [("pl", "wp_", 0), ("en", "wp_3_", 10_000_000)]

# The user ID of the blog's author in WordPress
SITE_OWNER_USER_ID = 1


def connect():
    return pymysql.connect(
        host=os.environ.get("WP_DB_HOST", "127.0.0.1"),
        port=int(os.environ.get("WP_DB_PORT", "3306")),
        user=os.environ.get("WP_DB_USER", "m4txblog"),
        password=os.environ["WP_DB_PASSWORD"],
        database=os.environ.get("WP_DB_NAME", "m4txblog"),
        charset="utf8mb4",
    )


class Node:
    def __init__(self, tag, attrs=None):
        self.tag = tag
        self.attrs = dict(attrs or {})
        self.children = []


class TreeBuilder(HTMLParser):
    VOID_TAGS = {"br", "img"}
    # Anything else (e.g. `<stdio.h>` in a code snippet) is kept as text
    KNOWN_TAGS = VOID_TAGS | {
        "a", "b", "blockquote", "cite", "code", "em", "i", "li", "ol", "p", "pre",
        "small", "strong", "tt", "ul",
    }

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.root = Node(None)
        self.stack = [self.root]

    def handle_starttag(self, tag, attrs):
        if tag not in self.KNOWN_TAGS:
            self.handle_data(self.get_starttag_text())
            return
        node = Node(tag, attrs)
        self.stack[-1].children.append(node)
        if tag not in self.VOID_TAGS:
            self.stack.append(node)

    def handle_endtag(self, tag):
        if tag not in self.KNOWN_TAGS:
            self.handle_data(f"</{tag}>")
            return
        # Be lenient with unclosed tags
        for i in range(len(self.stack) - 1, 0, -1):
            if self.stack[i].tag == tag:
                del self.stack[i:]
                break

    def handle_data(self, data):
        self.stack[-1].children.append(data)


def escape_markdown(text):
    # WordPress displayed indented text as-is, but it would become a code
    # block in Markdown
    text = re.sub(r"(?m)^[ \t]+", "", text)
    text = re.sub(r"([\\`*_\[\]<])", r"\\\1", text)
    return re.sub(r"(?m)^(\s*)([#>])", r"\1\\\2", text)


def plain_text(node):
    if isinstance(node, str):
        return node
    return "".join(plain_text(child) for child in node.children)


def render(node, in_list=None):
    if isinstance(node, str):
        return escape_markdown(node)

    inner = lambda: "".join(render(child, in_list) for child in node.children)
    tag = node.tag
    if tag in ("strong", "b"):
        content = inner().strip()
        return f"**{content}**" if content else ""
    if tag in ("em", "i", "cite"):
        content = inner().strip()
        return f"*{content}*" if content else ""
    if tag in ("code", "tt"):
        code = plain_text(node)
        if "\n" in code.strip():
            return f"\n\n```\n{code.strip(chr(10))}\n```\n\n"
        fence = "``" if "`" in code else "`"
        return f"{fence}{code}{fence}"
    if tag == "pre":
        return f"\n\n```\n{plain_text(node).strip(chr(10))}\n```\n\n"
    if tag == "a":
        text = inner().strip()
        href = node.attrs.get("href", "")
        if not href:
            return text
        return f"[{text or href}]({href})"
    if tag == "img":
        src = node.attrs.get("src", "")
        if "/smilies/" in src:
            # WordPress replaced emoticons with images; restore the text, so
            # that it's converted to an emoji below
            return f" {node.attrs.get('alt', '').strip()} "
        alt = node.attrs.get("alt", "") or "image"
        return f"[{escape_markdown(alt)}]({src})" if src else ""
    if tag == "br":
        return "\n"
    if tag == "p":
        return f"\n\n{inner().strip()}\n\n"
    if tag == "blockquote":
        content = re.sub(r"\n\s*\n(\s*\n)+", "\n\n", inner().strip())
        quoted = "\n".join(f"> {line}".rstrip() for line in content.split("\n"))
        return f"\n\n{quoted}\n\n"
    if tag in ("ul", "ol"):
        items = [child for child in node.children if isinstance(child, Node) and child.tag == "li"]
        lines = []
        for index, item in enumerate(items, start=1):
            marker = f"{index}." if tag == "ol" else "-"
            content = render(item, tag).strip().replace("\n", "\n   ")
            lines.append(f"{marker} {content}")
        return "\n\n" + "\n".join(lines) + "\n\n"
    if tag == "li":
        return inner()
    # Unknown tags (e.g. <small>) are just unwrapped
    return inner()


def html_to_markdown(content):
    content = content.replace("\r", "")
    builder = TreeBuilder()
    builder.feed(content)
    builder.close()
    markdown = render(builder.root)

    # The same emoticon replacements as in the post importer
    markdown = re.sub(r"(\s):\)\*\*", r"\1😄**", markdown)
    markdown = re.sub(r"(\s|^):\)(\s|$)", r"\1😄\2", markdown)
    markdown = re.sub(r"(\s|^);\)(\s|$)", r"\1😉\2", markdown)
    markdown = re.sub(r"(\s|^):P(\s|$)", r"\1😜\2", markdown)
    markdown = re.sub(r"(\s):P\*\*", r"\1😜**", markdown)

    # Remove the extra spaces added around the emoticon images
    markdown = re.sub(r" +([😄😉😜]) +", r" \1 ", markdown)
    markdown = re.sub(r" ([😄😉😜]) ([.,!?])", r" \1\2", markdown)

    markdown = re.sub(r"[ \t]+\n", "\n", markdown)
    markdown = re.sub(r"\n{3,}", "\n\n", markdown)
    return markdown.strip()


def main():
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} path/to/db.sqlite3", file=sys.stderr)
        sys.exit(2)

    legacy = tomllib.loads(LEGACY_REDIRECTS.read_text())
    posts = {(post["site"], post["wp_id"]): post for post in legacy["post"]}

    cursor = connect().cursor()
    comments = []
    for site, prefix, id_offset in SITES:
        cursor.execute(
            "SELECT c.comment_ID, c.comment_parent, c.comment_post_ID, c.comment_author, "
            "c.comment_author_url, c.comment_date_gmt, c.comment_content, c.user_id "
            f"FROM {prefix}comments c JOIN {prefix}posts p ON p.ID = c.comment_post_ID "
            "WHERE c.comment_approved = '1' AND c.comment_type IN ('comment', '') "
            "AND p.post_type = 'post' AND p.post_status = 'publish' "
            "ORDER BY c.comment_ID"
        )
        rows = cursor.fetchall()
        imported_ids = {row[0] for row in rows}
        for wp_id, parent, post_id, author, url, date_gmt, content, user_id in rows:
            post = posts.get((site, post_id))
            if post is None:
                sys.exit(f"error: no post mapping for {site} post {post_id}")
            url = url.strip()
            if not url.startswith(("http://", "https://")):
                url = None
            comments.append(
                {
                    "id": wp_id + id_offset,
                    "post_permalink": post["permalink"],
                    "post_language": post["language"],
                    "parent": parent + id_offset if parent in imported_ids else None,
                    "author_name": author.strip()[:100] or "Anonymous",
                    "author_url": url[:500] if url else None,
                    "content": html_to_markdown(content),
                    "is_site_owner": int(user_id == SITE_OWNER_USER_ID),
                    "created_at": date_gmt.replace(tzinfo=timezone.utc).isoformat(),
                }
            )

    db = sqlite3.connect(sys.argv[1])
    with db:
        existing = {
            row[0]: row[1:]
            for row in db.execute(
                f"SELECT id, post_permalink, author_name, created_at FROM {TABLE}"
            )
        }
        for comment in comments:
            current = existing.get(comment["id"])
            expected = (comment["post_permalink"], comment["author_name"], comment["created_at"])
            if current is not None and current != expected:
                sys.exit(
                    f"error: comment {comment['id']} already exists and wasn't imported "
                    f"from WordPress: {current}"
                )

        # Insert parents first, so that the foreign keys are always valid
        for comment in sorted(comments, key=lambda comment: comment["id"]):
            db.execute(
                f"INSERT INTO {TABLE} (id, post_permalink, post_language, parent, author_name, "
                "author_email, author_url, content, status, is_site_owner, created_at) "
                "VALUES (:id, :post_permalink, :post_language, :parent, :author_name, NULL, "
                ":author_url, :content, 'approved', :is_site_owner, :created_at) "
                "ON CONFLICT (id) DO UPDATE SET post_permalink = excluded.post_permalink, "
                "post_language = excluded.post_language, parent = excluded.parent, "
                "author_name = excluded.author_name, author_url = excluded.author_url, "
                "content = excluded.content, is_site_owner = excluded.is_site_owner, "
                "created_at = excluded.created_at",
                comment,
            )

    print(f"imported {len(comments)} comments", file=sys.stderr)


if __name__ == "__main__":
    main()
