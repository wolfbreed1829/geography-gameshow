"""Encode only the visually approved photographs, leaving originals untouched.

Run with Python + Pillow. Full images retain every oriented RGB pixel; only the
small navigation thumbnails are resized. No metadata, crops, or retouching are
included. The reviewed allowlist is in docs/PROGRAM-PHOTOS.json.
"""
import argparse
import hashlib
import json
from pathlib import Path

from PIL import Image, ImageOps

parser = argparse.ArgumentParser()
parser.add_argument("--source", type=Path, default=Path(r"C:\Users\mpatr\Downloads\asset1"))
args = parser.parse_args()
root = Path(__file__).resolve().parent.parent
approved = json.loads((root / "docs/PROGRAM-PHOTOS.json").read_text(encoding="utf-8"))
destination = root / "public/photos"
destination.mkdir(exist_ok=True)

for photo in approved:
    source = args.source / photo["source"]
    assert hashlib.sha256(source.read_bytes()).hexdigest() == photo["sha256"], f"Source changed; review again: {source}"
    with Image.open(source) as raw:
        original = ImageOps.exif_transpose(raw).convert("RGB")
    assert list(original.size) == photo["size"]
    output = destination / f'{photo["file"]}.webp'
    original.save(output, "WEBP", lossless=True, method=6)
    with Image.open(output) as encoded:
        assert encoded.size == original.size
        assert encoded.convert("RGB").tobytes() == original.tobytes()
        assert not encoded.getexif()
    thumbnail = original.copy()
    thumbnail.thumbnail((160, 120), Image.Resampling.LANCZOS)
    thumbnail.save(destination / f'{photo["file"]}-thumb.webp', "WEBP", quality=88, method=6)
    print(f'{photo["program"]}: {photo["file"]} - {output.stat().st_size:,} bytes; full image pixels verified')
