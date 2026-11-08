"""Losslessly encode web assets; original PNG downloads remain untouched.

Requires Pillow. Dimensions, alpha, and every visible pixel are verified.
WebP may normalize the invisible RGB values of fully transparent pixels.
"""
from pathlib import Path
from PIL import Image

public = Path(__file__).resolve().parent.parent / "public"
names = ["ocean-background", "geography-gameshow-logo", "neal-live", "neal-art", "neal-portrait"]
for name in names:
    source = public / f"{name}.png"
    destination = public / f"{name}.webp"
    original = Image.open(source).convert("RGBA")
    original.save(destination, format="WEBP", lossless=True, method=6)
    encoded = Image.open(destination).convert("RGBA")
    assert original.size == encoded.size
    assert original.getchannel("A").tobytes() == encoded.getchannel("A").tobytes()
    pixels = lambda image: image.get_flattened_data() if hasattr(image, "get_flattened_data") else image.getdata()
    assert all(a == b or a[3] == b[3] == 0 for a, b in zip(pixels(original), pixels(encoded)))
    print(f"{name}: {source.stat().st_size:,} -> {destination.stat().st_size:,} bytes; visible pixels verified")
