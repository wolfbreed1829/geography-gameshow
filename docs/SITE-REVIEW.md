# Site review and redesign direction

Reviewed November 8, 2026: [geographygameshow.com](http://www.geographygameshow.com/), including the information, programs, guidelines, history, media, art, photos, and contact pages.

## What should stay

Neal is the differentiator. The live drawing from memory, classroom interaction, personal travel story, and original artwork make the experience specific and credible. The redesign preserves that identity and the recognizable Geography Gameshow name.

## What I would change

| Current finding | Recommended change | Included in this preview |
| --- | --- | --- |
| Navigation is made of image buttons with empty alternative text. | Use semantic, readable links with accessible names and a mobile menu. | Yes |
| A fixed-width, table-based layout and raster text make the site difficult to use on phones. | Responsive layouts, scalable type, clear hierarchy, and generous touch targets. | Yes |
| The home page shows the performance but gives the booking journey little emphasis. | Lead with the live-drawing hook, show the range, and provide clear inquiry actions. | Yes |
| Program details, logistics, biography, and media need a clearer navigation structure. | Keep the homepage compact, with a Programs hub and focused pages for each section. | Yes |
| The art page advertises summer 2016 classes. | Remove expired dates from the primary experience and establish a confirmed program catalog. | Yes; new concepts remain labeled |
| Anatomy has no dedicated offering on the current site. | Give Anatomy a distinct visual identity and a clearly labeled concept preview. | Yes |
| Video uses obsolete Flash embed markup. | Use a current, deliberate YouTube link or privacy-conscious player. | Original overview video linked; external playback availability depends on YouTube |
| Contact is a separate email/address page. | An international inquiry form should gather school, teacher/administrator, email, phone, full school address, country, and optional visit details. | Yes; local preview only, no sending or storage |
| Many photos have empty alternative text. | Describe meaningful images; leave purely decorative art out of the accessibility tree. | Yes |
| The reviewed site is reachable over HTTP; automated HTTPS retrieval failed. | Verify HTTPS and certificate configuration when moving the production domain. | The separate GitHub Pages preview uses HTTPS |

## Creative direction

“Big ideas. Drawn live.” retains the geography brand while making room for a broader family of experiences. The requested dark-ocean identity uses a photographic cobalt water backdrop with slow layered motion, midnight-blue surfaces, gold typography, olive land tones, and earthy program illustrations. Neal’s original globe-and-lettering logo has been recreated from the supplied photograph, with the surrounding scenery removed and the requested motto above it. The small program globe uses actual Natural Earth map data. Neal’s own artwork and classroom photos ground the design in his work. Ocean motion can be paused and respects reduced-motion preferences.

The homepage introduces the show, motto, and globe. Programs has its own hub, with Geography, Anatomy, and Art on separate pages. The show in action, challenge, Neal’s story, practical school information, and inquiry form also have their own URLs. The school information uses the supplied program document without publishing its pricing or payment terms.

## Content choices requiring Neal’s input

1. Does he want to retain “The Geography Gameshow” as the umbrella brand, or introduce a wider brand for all three subjects?
2. What are the actual Anatomy topics, audience ages, duration, materials, and learning objectives?
3. Is the art offering a performance, a hands-on workshop, or both? The site currently documents older drawing classes, not this proposed show format.
4. Are the published geography program lengths, group sizes, supplies, and email address still current?
5. Which existing images are approved for a new public website? Higher-resolution recent show photos and a short updated video would improve the next version.

## Technical direction

This preview is a Rust/Leptos client application compiled to WebAssembly and hosted as static files on GitHub Pages. Ten HTML entry files share the Rust application and stylesheets, with unique page metadata and working direct links. Fonts and images are self-hosted. The inquiry form validates and displays a local preview; nothing is sent or stored.

For a permanent marketing site, consider server-rendered or prerendered Leptos so search engines and first-time visitors receive the actual page content immediately. The current inquiry form is an intentionally local preview; connect delivery only when requested. Keep pricing off the website. Add dates or testimonials only when supplied and approved by Neal.

## Sources

- [Homepage and live-drawing description](http://www.geographygameshow.com/)
- [Geography programs and durations](http://www.geographygameshow.com/programs.html)
- [Audience sizes and room setup](http://www.geographygameshow.com/information.html)
- [Neal’s biography](http://www.geographygameshow.com/history.html)
- [Older drawing classes and artwork](http://www.geographygameshow.com/art.html)
- [Original show videos](http://www.geographygameshow.com/media.html)
- [Published contact email](http://www.geographygameshow.com/contact.html)
- [Leptos CSR and SSR tradeoffs](https://book.leptos.dev/getting_started/index.html)
