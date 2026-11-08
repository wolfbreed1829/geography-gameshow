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

`anatomy.svg`, `art-tools.svg`, `ocean-waves.svg`, and `favicon.svg` are custom vector illustrations made for this concept. The Anatomy illustration is stylized educational artwork, not a medical diagram. The earlier `botanical.svg` is retained but no longer used by the site. The globe is generated using D3 Geo and Natural Earth’s public-domain geographic data via `world-atlas@2.0.2`. Source and generation instructions are in `scripts/generate-globe.mjs`. The dashed arc and dots are decorative and do not represent a touring schedule. Ocean current lines are an artistic effect, not a scientific current map.

## Ocean background

`public/ocean-background.png` is a new background created with the built-in image generation tool, then copied into the project. The website animates layered copies with CSS transforms and opacity; the backdrop is decorative and does not receive pointer input. Motion can be paused and is disabled for reduced-motion preferences. The website loads lossless WebP copies of the background, logo, and photographs to reduce transfer size; original PNGs are retained. `scripts/encode-web-assets.py` verifies unchanged dimensions, alpha, and visible pixels.

Prompt: Create a premium cinematic ocean texture for the full-screen background of an educational performance website. Wide landscape composition, 16:9. Directly overhead aerial view of deep open ocean at blue hour, only water from edge to edge, absolutely no horizon, sky, land, boats, people, objects, text, or logos. Rich true cobalt-blue water blending into very dark midnight-navy depths. Elegant long flowing current patterns and gentle natural wave ridges, with subtle turquoise/cobalt reflected light catching the crests. Photorealistic, tactile, refined, atmospheric. Keep the left third and upper edge darker and calmer, with the most beautiful luminous ripples concentrated through the center-right and lower-right. Avoid bright white foam, white highlights, glitter, storm waves, excessive contrast, black crushed shadows, and generic gradient appearance. The image should feel like an immersive living ocean while being subdued enough to sit behind readable gold and ivory website typography. Fine natural water detail, flowing rhythmic movement implied in the water, luxurious and serene. No vignette frame. High resolution landscape background asset.

## Fonts

DM Sans, Space Grotesk, and Caveat are downloaded from Google Fonts and served locally, with OFL licenses included alongside the font files. `scripts/download-fonts.py` refreshes the Latin subsets and licenses.
