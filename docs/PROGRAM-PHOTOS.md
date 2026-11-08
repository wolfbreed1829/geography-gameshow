# Program photograph selection — November 8, 2026

Reviewed all 96 supplied images again: 77 in `geography`, one in `anatomy`, and
18 in `references/art`, within `C:\Users\mpatr\Downloads\asset1`.

The user clarified that children may appear when their faces are not visible,
and that usable archive photographs should be included even if they are not
perfectly sharp. The second pass added 47 images to the original 13, for 60
published images. Selection is no longer limited to the strongest compositions
or photographs without an audience. The source originals remain untouched.

| Gallery | Images |
| --- | ---: |
| Geography | 35 |
| Anatomy | 2 |
| Art | 23 |

## Review and placement

All images were screened in contact sheets, and candidates were inspected at
their source dimensions. Approved classroom views show children from behind,
without visible faces. Small profiles, people at the edges, and portraits
within the scene were considered. Images with visible or uncertain partial
faces were left out. Softer but understandable images, smaller originals,
collages, and supplied publication layouts were retained under the revised
quality standard.

The digestive-system drawing was filed in `geography`; it appears on Anatomy.
The eye drawing from `anatomy` is now included too. Drawing-class and studio
images filed under `geography` appear on Art. Captions describe the visible
content without inventing dates, schools, or venue names. The illustrated
Geography portrait is described as stylized; the handmade thank-you is not
presented as Neal's own artwork.

The review IDs below follow the original alphabetical contact-sheet catalog.

| Excluded group | Review IDs |
| --- | --- |
| Visible faces, partial profiles, or uncertainty about a face, including printed student portraits | geo02, geo03, geo05, geo06, geo09, geo13, geo14, geo17, geo18, geo22, geo26, geo27, geo29, geo30, geo33, geo34, geo36, geo42, geo46, geo47, geo48, geo53, geo56, geo59, geo63, geo68, geo70, geo76 |
| Heavy blur and a partial side view of a child | geo75 |
| Duplicate views of images already included | geo32, geo38, geo66, geo67, geo72 |
| Newspaper document, rather than a program or artwork photograph | geo15 |
| Child portrait drawing | art02 |

The earlier `public/neal-live.png` and `.webp` remain removed: that classroom
photo contains a child's profile at the right edge. “In Action” continues to
use the approved `world-in-progress.webp` photograph.

## Sources and encoding

[PROGRAM-PHOTOS.json](PROGRAM-PHOTOS.json) is the approved manifest. It records
all 60 filenames, SHA-256 source checksums, review IDs, program placement,
original dimensions, titles, captions, and alternative text.

`python scripts/encode-program-photos.py` creates lossless WebP images under
`public/photos`, retaining every oriented RGB pixel and the original dimensions
while omitting metadata. Small navigation thumbnails are separately resized.
No face blurring, cropping, generative edits, sharpening, or enlargement is
applied. Only the approved manifest is encoded; unselected source files are
never copied to the website.

`python scripts/generate-photo-data.py` generates `src/gallery_photos.rs` from
the manifest. Run it with `--check` to verify the committed Rust data matches.

## Presentation

The program overview pages remain uncluttered. Each has Overview / Photos
subnavigation linking to `geography-photos.html`, `anatomy-photos.html`, or
`art-photos.html`. Full photographs display without cropping and are capped at
their original pixel dimensions so small archive images are not enlarged.

Galleries are manual, with named thumbnails, previous/next controls, and
announced captions. Geography and Art also have a selector listing every photo
by title, making the larger collections easy to browse without a long page.
The homepage and references remain intact. Art and Anatomy retain their
existing development status.
