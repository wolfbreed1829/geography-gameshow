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

const GEOGRAPHY: &[Photo] = &[
    Photo {
        file: "world-in-progress",
        title: "Drawing the world",
        caption: "A world map takes shape, one line at a time.",
        alt: "Neal drawing Europe on a large hand-drawn world map",
        width: 1290,
        height: 708,
    },
    Photo {
        file: "north-america",
        title: "From a blank page",
        caption: "The outline is just the beginning of the story.",
        alt: "Neal drawing the outline of North America on paper",
        width: 672,
        height: 745,
    },
    Photo {
        file: "new-england",
        title: "New England, in detail",
        caption: "Places, history, and memorable details share the same page.",
        alt: "Hand-drawn map of New England with landmarks, dates, and illustrations",
        width: 838,
        height: 567,
    },
    Photo {
        file: "mexico",
        title: "Mexico & its neighbors",
        caption: "A closer look at the places and stories within a map.",
        alt: "Colorful drawing of Mexico, Central America, and the Caribbean",
        width: 837,
        height: 583,
    },
    Photo {
        file: "pacific-world",
        title: "A different view of the world",
        caption: "A Pacific-centered map invites a fresh perspective.",
        alt: "Neal standing beside a wide, Pacific-centered world map",
        width: 1136,
        height: 470,
    },
];

const ANATOMY: &[Photo] = &[Photo {
    file: "digestive-system",
    title: "Inside the digestive system",
    caption: "An example of Neal’s illustrated approach to exploring the human body.",
    alt: "Colorful, labeled drawing of the human digestive system",
    width: 601,
    height: 772,
}];

const ART: &[Photo] = &[
    Photo {
        file: "artist-in-studio",
        title: "At work in the studio",
        caption: "Neal with a detailed harbor drawing, ready for its close-up.",
        alt: "Neal holding a drawing of fishing boats beside a studio camera",
        width: 1170,
        height: 927,
    },
    Photo {
        file: "windmill",
        title: "Color & character",
        caption: "Rich color and careful detail bring a windmill to life.",
        alt: "Color illustration of a wooden windmill against a deep blue sky",
        width: 1170,
        height: 1541,
    },
    Photo {
        file: "queensland-timber",
        title: "Texture in every line",
        caption: "Weathered timber and corrugated roofs in Yungaburra, Queensland.",
        alt: "Pen drawing of an old timber building in Yungaburra, Queensland",
        width: 1170,
        height: 784,
    },
    Photo {
        file: "stone-fortress",
        title: "Light, shadow & stone",
        caption: "Architecture becomes a study in depth and contrast.",
        alt: "Detailed monochrome drawing of stone towers, walls, and a mountain",
        width: 640,
        height: 797,
    },
    Photo {
        file: "harbor-boats",
        title: "Life along the harbor",
        caption: "Fishing boats, reflections, and the quiet detail of a working waterfront.",
        alt: "Monochrome drawing of two fishing boats moored alongside wooden pilings",
        width: 765,
        height: 634,
    },
    Photo {
        file: "hand-studies",
        title: "Learning to look",
        caption: "A familiar subject reveals endless possibilities for drawing.",
        alt: "Three pencil studies of hands, including one holding a pencil",
        width: 640,
        height: 831,
    },
    Photo {
        file: "pagoda",
        title: "Rhythm & perspective",
        caption: "Repeating rooflines and trees create a layered architectural portrait.",
        alt: "Black-and-white drawing of a multi-story pagoda among tall trees",
        width: 640,
        height: 1136,
    },
];

#[component]
pub fn ProgramGallery(index: usize) -> impl IntoView {
    let (photos, name, label) = match index {
        0 => (GEOGRAPHY, "Geography", "FROM THE SHOW"),
        1 => (ANATOMY, "Anatomy", "FROM THE DRAWING BOARD"),
        _ => (ART, "Art", "FROM NEAL’S PORTFOLIO"),
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
