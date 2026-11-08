# Program photograph selection — November 8, 2026

Reviewed 96 supplied images: 77 in `geography`, one in `anatomy`, and 18 in
`references/art`, within `C:\Users\mpatr\Downloads\asset1`.

The user requested clear photographs with no children's faces. All 96 were
screened using contact sheets, then candidate files were inspected individually
at their source dimensions. Thirteen were selected for clarity, composition,
variety, and absence of children's faces. The selected images contain only the
adult presenter or artwork; no classroom audience is included. The originals
remain untouched in the supplied folders.

## Published selection

| Page | Image | Source review ID |
| --- | --- | --- |
| Geography | Drawing the world | geo71 |
| Geography | From a blank page | geo16 |
| Geography | New England, in detail | geo04 |
| Geography | Mexico & its neighbors | geo07 |
| Geography | A different view of the world | geo60 |
| Anatomy | Inside the digestive system | geo49 |
| Art | At work in the studio | art07 |
| Art | Color & character | art03 |
| Art | Texture in every line | art04 |
| Art | Light, shadow & stone | art06 |
| Art | Life along the harbor | art17 |
| Art | Learning to look | art12 |
| Art | Rhythm & perspective | art15 |

The digestive-system drawing was filed in `geography`; it is displayed on the
Anatomy page based on its content. The eye drawing in `anatomy` (also duplicated
as geo67) was left out because of soft focus. Other exclusions include visible
children's faces or profiles, blur, small low-resolution images, screenshots,
duplicate views, and weaker or repetitive compositions. The child portrait
drawing in the Art folder was also excluded.

The earlier `public/neal-live.png` and `.webp` classroom image contained a
child's profile at the right edge. Both files were removed from the published
assets. “In Action” now reuses the approved `world-in-progress.webp` photograph.

## Encoding and presentation

[PROGRAM-PHOTOS.json](PROGRAM-PHOTOS.json) records the source filenames, SHA-256
checksums, program placement, and original dimensions. Only these approved
images are copied into `public/photos`, using
`python scripts/encode-program-photos.py`. Full photographs use lossless WebP
with their original dimensions and visible pixels preserved; metadata is
omitted. Small navigation thumbnails are separately resized. No face blurring,
cropping, generative edits, sharpening, or enlargement was used.

The program overview pages remain uncluttered. Each has an Overview / Photos
subnavigation linking to its own `geography-photos.html`, `anatomy-photos.html`,
or `art-photos.html` gallery. The homepage remains compact. Full photographs are
displayed without cropping. Galleries are manual, with named thumbnail buttons, previous/next
controls, and announced captions; the single Anatomy image needs no controls.
Captions describe the visible work and do not invent dates, schools, or venue
names. The Art and Anatomy concepts retain their existing development status.
