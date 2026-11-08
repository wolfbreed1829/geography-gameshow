# The Geography Gameshow

**Big ideas. Drawn live.** A fresh website concept for Neal Nichols Jr., bringing the original Geography Gameshow together with proposed biology and art shows.

[Open the live preview](https://wolfbreed1829.github.io/geography-gameshow/) · [Read the site review](docs/SITE-REVIEW.md)

![Cobalt-blue oceans and earth tones in the Geography Gameshow redesign](docs/preview-desktop.png)

## What is here

- A responsive dark-ocean design with a custom photographic water backdrop, drifting currents, warm gold accents, earthy land tones, and custom illustrations. Ocean motion can be paused and respects reduced-motion preferences.
- Neal’s actual classroom photos and artwork from his existing website.
- Geography, biology, and art selectors with distinct program information.
- Three playable, three-question challenges with scoring, feedback, and replay.
- An accessible mobile menu, keyboard-operable program tabs, FAQs, and reduced-motion support.
- A validated inquiry form that prepares a reviewable email. It **does not send, store, or submit inquiries to a backend**. The visitor sends through their own email app.
- GitHub Actions builds the Rust application and publishes it to GitHub Pages.

This is a review concept. Biology and the new art show are clearly marked as concepts in development. Existing geography program information comes from the current website and needs Neal’s confirmation before a production launch. No invented testimonials, prices, availability, or endorsements are included.

## Run locally

Install a current stable Rust toolchain, the `wasm32-unknown-unknown` target, and [Trunk](https://trunk-rs.github.io/trunk/).

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
trunk serve
```

Open http://127.0.0.1:8080. The application is written in **Rust with Leptos 0.8 CSR**, compiled to WebAssembly. JavaScript is limited to the generated WASM loader; the application interactions live in Rust.

```sh
cargo fmt --all -- --check
cargo check --locked --target wasm32-unknown-unknown
trunk build --release --locked --public-url ./
```

The output is `dist/`. Relative asset URLs support both a GitHub Pages project path and a root-domain deployment. Fonts and visual assets are hosted locally. No third-party analytics or embedded tracking is included.

## Structure

| File | Purpose |
| --- | --- |
| `src/main.rs` | Page components, program selection, inquiry builder |
| `src/quiz.rs` | Quiz questions, feedback, score, and reset behavior |
| `style.css`, `ocean.css` | Responsive layout, dark ocean identity, motion, and accessibility |
| `public/` | Images, custom SVG illustrations, fonts and licenses |
| `.github/workflows/deploy.yml` | Reproducible Rust build and GitHub Pages deployment |
| `docs/SITE-REVIEW.md` | Review findings and recommended next steps |
| `docs/ASSETS.md` | Asset sources and rights notes |

## Optional illustration tooling

Node is used for optional illustration generation and browser tests, not to build or run the website. The normal Rust build does not require npm.

```sh
npm ci
npm run assets:globe
python scripts/download-fonts.py
```

## Browser verification

After building and serving `dist/` at port 8080:

```sh
npm ci
npx playwright install chromium
npm test
```

Set `PLAYWRIGHT_CHANNEL=msedge` to use an installed Microsoft Edge browser, or `PREVIEW_URL` to test another deployment. The eight browser checks cover image/font loading, runtime errors, program tabs and keyboard focus, all three quizzes, email validation/encoding, menu behavior, overflow at five viewport widths, and motion pause/resume/reduced-motion behavior.

## Before a full public launch

Ask Neal to confirm the new show names and descriptions, age ranges, current contact email, geography timings, technical needs, and permission to reuse the existing photos and artwork. Replace concept labels when each show is ready. A future production build should consider Leptos SSR or static prerendering for better initial HTML and search indexing; CSR is convenient for this GitHub Pages review.

Original Geography Gameshow photography, artwork, name, and content remain the property of their respective owners. Font licenses are included in `public/`; Natural Earth geographic data is public domain. No blanket license is granted over third-party assets.
