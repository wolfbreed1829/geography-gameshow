"""Emit real, directly addressable HTML pages from Trunk's completed app shell.

Called by the post_build hook. All entry pages share the same hashed Rust/WASM
and stylesheet bundles; GitHub Pages needs no catch-all redirect or rewrite.
"""
from html import escape
from pathlib import Path
import os
import re

output = Path(os.environ["TRUNK_STAGING_DIR"])
shell = (output / "index.html").read_text(encoding="utf-8")
pages = {
    "programs": ("Programs", "Explore Geography, Anatomy, and Art programs with Neal Nichols Jr."),
    "geography": ("Geography", "Maps drawn entirely from memory. Discover Neal Nichols Jr.'s original live Geography Gameshow."),
    "anatomy": ("Anatomy", "Explore a new live-drawing concept about the human body: bones, muscles, and organs."),
    "art": ("Art", "Discover Neal Nichols Jr.'s proposed art show, exploring drawing, perspective, texture, and creativity."),
    "in-action": ("The Show in Action", "See the Geography Gameshow in the classroom and watch the original show overview."),
    "meet-neal": ("Meet Neal", "Meet Neal Nichols Jr., artist, traveler, educator, and creator of The Geography Gameshow."),
    "challenge": ("The Curiosity Challenge", "Try three-question Geography, Anatomy, and Art challenges inspired by the spirit of the show."),
    "planning": ("Plan a Visit", "Plan a worldwide school visit: scheduling, room setup, student participation, and school media."),
    "booking": ("School Inquiries", "An international school inquiry form for Geography, Anatomy, and Art programs with Neal Nichols Jr."),
}
for slug, (title, description) in pages.items():
    title = escape(f"{title} | The Geography Gameshow")
    document = re.sub(r"<title>.*?</title>", f"<title>{title}</title>", shell)
    document = re.sub(r'<meta name="description" content="[^"]*"\s*/?>', f'<meta name="description" content="{escape(description, quote=True)}">', document)
    document = re.sub(r'<meta property="og:title" content="[^"]*"\s*/?>', f'<meta property="og:title" content="{title}">', document)
    document = re.sub(r'<meta property="og:description" content="[^"]*"\s*/?>', f'<meta property="og:description" content="{escape(description, quote=True)}">', document)
    (output / f"{slug}.html").write_text(document, encoding="utf-8")
print(f"Built {len(pages) + 1} standalone page entries.")
