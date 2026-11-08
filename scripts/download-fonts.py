"""Refresh locally hosted Latin font subsets and their OFL licenses."""
from pathlib import Path
from urllib.request import Request, urlopen
import re

OUT = Path(__file__).resolve().parent.parent / "public"
FONTS = [("DM+Sans", "dm-sans", "dmsans"), ("Space+Grotesk", "space-grotesk", "spacegrotesk"), ("Caveat", "caveat", "caveat")]
for family, filename, directory in FONTS:
    url = f"https://fonts.googleapis.com/css2?family={family}:wght@400..700&display=swap"
    req = Request(url, headers={"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36"})
    css = urlopen(req).read().decode()
    latin = css.split("/* latin */")[-1]
    match = re.search(r"url\((https:[^)]+)\)", latin)
    if not match:
        raise RuntimeError(f"No Latin font found for {family}")
    font_bytes = urlopen(match.group(1)).read()
    if font_bytes[:4] != b"wOF2":
        raise RuntimeError(f"Expected WOFF2 for {family}, received {font_bytes[:4]!r}")
    (OUT / f"{filename}.woff2").write_bytes(font_bytes)
    license_url = f"https://raw.githubusercontent.com/google/fonts/main/ofl/{directory}/OFL.txt"
    (OUT / f"{filename}-LICENSE.txt").write_bytes(urlopen(license_url).read())
    print(f"Downloaded {filename} and license")
