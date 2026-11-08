use leptos::prelude::*;

#[derive(Clone, Copy)]
struct Photo {
    file: &'static str,
    title: &'static str,
    caption: &'static str,
    alt: &'static str,
    width: u32,
    height: u32,
}

include!("gallery_photos.rs");

#[component]
pub fn ProgramGallery(index: usize) -> impl IntoView {
    let (photos, name, label) = match index {
        0 => (GEOGRAPHY, "Geography", "FROM THE SHOW"),
        1 => (ANATOMY, "Anatomy", "FROM THE DRAWING BOARD"),
        _ => (ART, "Art", "ARTWORK & DRAWING MOMENTS"),
    };
    let current = RwSignal::new(0usize);
    let photo = move || photos[current.get()];
    let count = photos.len();
    let role_description = if count > 1 { "carousel" } else { "gallery" };

    view! {
        <section class="program-gallery" aria-label=format!("{name} photo gallery") aria-roledescription=role_description>
            <p class="gallery-label">{label}</p>
            <figure>
                <div class="gallery-frame">
                    <img class="gallery-image"
                        src=move || format!("public/photos/{}.webp", photo().file)
                        alt=move || photo().alt
                        width=move || photo().width height=move || photo().height
                        style=move || format!("max-width: {}px; max-height: {}px", photo().width, photo().height)
                        decoding="async"/>
                </div>
                <figcaption>
                    <div class="gallery-caption" aria-live="polite" aria-atomic="true">
                        <strong>{move || photo().title}</strong>
                        <p>{move || photo().caption}</p>
                    </div>
                    {(count > 1).then(|| view! {
                        <div class="gallery-controls">
                            <span class="gallery-count" aria-label=move || format!("Photo {} of {count}", current.get() + 1)>{move || format!("{:02} / {count:02}", current.get() + 1)}</span>
                            <div class="gallery-arrows">
                                <button type="button" aria-label="Previous photo" on:click=move |_| current.update(|i| *i = (*i + count - 1) % count)><span aria-hidden="true">"←"</span></button>
                                <button type="button" aria-label="Next photo" on:click=move |_| current.update(|i| *i = (*i + 1) % count)><span aria-hidden="true">"→"</span></button>
                            </div>
                        </div>
                    })}
                </figcaption>
            </figure>
            {(count > 8).then(|| view! {
                <label class="gallery-jump" for="gallery-photo-select">
                    <span>{format!("Browse all {count} photos")}</span>
                    <select id="gallery-photo-select" prop:value=move || current.get().to_string()
                        on:change=move |event| {
                            if let Ok(index) = event_target_value(&event).parse::<usize>() {
                                if index < count { current.set(index); }
                            }
                        }>
                        {photos.iter().enumerate().map(|(i, p)| view! {
                            <option value=i.to_string()>{format!("{:02} — {}", i + 1, p.title)}</option>
                        }).collect_view()}
                    </select>
                </label>
            })}
            {(count > 1).then(|| view! {
                <div class="gallery-thumbnails" role="group" aria-label="Choose a photo">
                    {photos.iter().enumerate().map(|(i, p)| view! {
                        <button type="button" aria-label=format!("Show photo {}: {}", i + 1, p.title)
                            aria-pressed=move || (current.get() == i).to_string()
                            on:click=move |_| current.set(i)>
                            <img src=format!("public/photos/{}-thumb.webp", p.file) alt="" width="128" height="96" loading="lazy" decoding="async"/>
                        </button>
                    }).collect_view()}
                </div>
            })}
        </section>
    }
}

pub fn photo_count(index: usize) -> usize {
    match index {
        0 => GEOGRAPHY.len(),
        1 => ANATOMY.len(),
        _ => ART.len(),
    }
}
