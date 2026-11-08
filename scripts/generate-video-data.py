"""Generate the small Rust video catalog from the verified YouTube manifest."""
import argparse
import json
from pathlib import Path
import re

root = Path(__file__).resolve().parent.parent
parser = argparse.ArgumentParser()
parser.add_argument("--check", action="store_true")
args = parser.parse_args()
videos = json.loads((root / "docs/PROGRAM-VIDEOS.json").read_text(encoding="utf-8"))
assert len({video["id"] for video in videos}) == len(videos), "Duplicate video ID"
for video in videos:
    assert re.fullmatch(r"[A-Za-z0-9_-]{11}", video["id"]), video["id"]
    assert video["program"] in {"geography", "art"}, video["program"]
    assert video["source"] == f'https://www.youtube.com/watch?v={video["id"]}'

lines = ["// Generated from docs/PROGRAM-VIDEOS.json by scripts/generate-video-data.py."]
for program in ("geography", "art"):
    lines.append(f"const {program.upper()}_VIDEOS: &[Video] = &[")
    for video in videos:
        if video["program"] != program:
            continue
        lines.append("    Video {")
        for field in ("id", "title", "description", "channel", "duration"):
            lines.append(f"        {field}: {json.dumps(video[field], ensure_ascii=False)},")
        lines.append("    },")
    lines.append("];")
result = "\n".join(lines) + "\n"
target = root / "src/video_data.rs"
if args.check:
    assert target.read_text(encoding="utf-8") == result, "Run scripts/generate-video-data.py"
    print(f"Verified {len(videos)} video entries.")
else:
    target.write_text(result, encoding="utf-8")
    print(f"Generated {len(videos)} video entries.")
