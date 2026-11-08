# Cinematic program artwork

Created November 8, 2026 with the built-in image-generation tool. These are decorative, photorealistic 3D concept renders, not photographs of Neal's actual artwork or clinical anatomy diagrams.

## Saved assets and resolution

- Geography master: [geography-3d.png](../design/program-art/geography-3d.png)
- Anatomy master: [anatomy-3d.png](../design/program-art/anatomy-3d.png)
- Art master: [art-3d.png](../design/program-art/art-3d.png)

The prompt requested 3840 × 2160. The built-in tool returned **1672 × 941** for each image. These are the native masters; no upscaling or extra generated detail is claimed. The layout is checked at a 3840-pixel desktop viewport and uses responsive image selection for high-density screens. The source resolution is separate from screen resolution.

The site serves WebP derivatives at widths 640, 1000, and 1672 in `public/programs/{geography,anatomy,art}-3d-{width}.webp`, encoded at quality 88. Smaller versions use Lanczos downsampling. Native PNG originals are stored outside the deployed public directory. Only mechanical resizing/encoding was used after generation; the artwork was not retouched with code.

## Motion

Each card has a slow camera-like drift of its artwork, a staggered light sweep, and a mouse-responsive perspective tilt with parallax and reflected light. Touch input does not tilt cards. The site's motion-pause control freezes the drifts/sweeps and disables pointer motion. Reduced-motion preference removes animated transforms and light sweeps. Existing program links, development labels, and media tabs are retained.

## Exact prompt set

### Shared brief

Use case: stylized-concept. Asset type: cinematic 3D hero artwork for a premium educational website program card, not a screenshot or UI mockup. Output a native 4K landscape image, 3840 x 2160 pixels, maximum available detail. One polished photorealistic physically rendered scene. Deep midnight navy environment, rich cobalt reflections, warm champagne-gold edge lighting, restrained earthy accents. Strong volumetric depth, ray-traced reflections, realistic material microtexture, subtle atmospheric scattering, crisp subject and softly defocused background. Composition: keep the complete main object in the central 50% of the canvas, occupying about 80% of image height, so it survives a narrow card crop; enough breathing room at every edge. Outer edges and bottom fade naturally into dark navy. Futuristic museum-exhibit aesthetic, sophisticated and wondrous, not a cartoon, flat vector, diagram, generic icon, or game interface. No text, numbers, labels, logos, watermarks, user interface, people or children's faces.

### geography

Subject: a striking realistic planet Earth suspended in space, with geographically recognizable North and South America facing the viewer and Europe/Africa curving toward the far side. Deep cobalt ocean, realistic muted moss-green land and ochre deserts, detailed terrain, delicate white cloud systems and a thin luminous blue atmosphere. Lit dramatically from upper left with a warm rim, dark terminator across the lower right. Two extremely thin champagne-metal orbital arcs curve around the actual 3D sphere with correct occlusion, one passing in front and one behind. Tiny golden light points along the arcs, restrained. Make Earth convincingly spherical, massive, tactile, almost a premium scientific globe coming to life. Full globe visible, no flat map, no lettering. Render as a realistic cinematic 3D space photograph.

### anatomy

Subject: an elegant futuristic adult human anatomical torso sculpture from base of neck to pelvis, shoulders included, no head, no face, no genitalia, no arms needed. A translucent smoke-glass outer torso reveals a carefully proportioned ivory rib cage, pale spine, subtle sage-green lungs and a warm copper heart in the correct central-left chest position. This is a clean, non-graphic educational museum model: no blood, wounds, exposed wet flesh or horror. Material detail is rich and realistic: polished glass refraction, satin ceramic bones, subtle amber light tracing connections. The torso is shown at a slight three-quarter angle, visibly deep and dimensional, suspended just above a dark circular display plinth with faint concentric light rings and believable reflected light. Keep the complete torso centrally framed, top and base visible. Real sculptural anatomy, not a line diagram or glowing humanoid robot.

### art

Subject: a breathtaking photorealistic 3D still life about drawing and creativity. A partially open charcoal-bound artist sketchbook floats at a three-quarter perspective, its textured warm ivory page bearing a subtle precise graphite drawing of a Cape Cod harbor building, with no text. A beautifully realistic sharpened golden-ochre wooden pencil and a slender artist paintbrush with a brushed brass ferrule float diagonally above the sketchbook. A single sculptural ribbon of thick cobalt-blue paint curls through the composition, accompanied by smaller restrained terracotta and champagne-gold paint accents. Real paper fibres, wood grain, bristles, graphite and glossy liquid surface texture. Strong tangible perspective and dramatic realistic shadows. Tiny thin luminous golden perspective lines near the drawing suggest future creative possibility, but the physical tools and paper remain dominant. Arrange all elements compactly within the central half of the frame for a narrow crop. No people, hands, faces, typography or brand names.

