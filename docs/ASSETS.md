# Asset provenance

## Existing Geography Gameshow materials

Used to illustrate the redesign requested for the existing business. Copyright and permissions remain with the original owners; confirm continued reuse before replacing the production website.

| Local asset | Source |
| --- | --- |
| `public/neal-live.png` | http://www.geographygameshow.com/images/p101513-home-5.png |
| `public/neal-portrait.png` | http://www.geographygameshow.com/images/NealBeowulf.png |
| `public/neal-art.png` | http://www.geographygameshow.com/images/Draw_UncleTimsBridge.png |

The linked YouTube overview, `YnSI6XrQm8E`, is the video identified by the current site’s media page. It is linked rather than rehosted.

## New illustrations

The two `geography-gameshow-logo*.png` files are AI-assisted recreations of the existing logo from a photograph supplied by the user, with the surrounding scenery and phone UI removed. The original brand belongs to its owner. Generation prompts and the exact requested motto are in [LOGO-PROMPTS.md](LOGO-PROMPTS.md).

`botanical.svg`, `art-tools.svg`, `ocean-waves.svg`, and `favicon.svg` are custom vector illustrations made for this concept. The globe is generated using D3 Geo and Natural Earth’s public-domain geographic data via `world-atlas@2.0.2`. Source and generation instructions are in `scripts/generate-globe.mjs`. The dashed arc and dots are decorative and do not represent a touring schedule. Ocean current lines are an artistic effect, not a scientific current map.

## Fonts

DM Sans, Space Grotesk, and Caveat are downloaded from Google Fonts and served locally, with OFL licenses included alongside the font files. `scripts/download-fonts.py` refreshes the Latin subsets and licenses.
