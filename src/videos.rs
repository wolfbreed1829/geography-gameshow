use leptos::prelude::*;

#[derive(Clone, Copy)]
struct Video {
    id: &'static str,
    title: &'static str,
    description: &'static str,
    channel: &'static str,
    duration: &'static str,
}

include!("video_data.rs");

fn collection(index: usize) -> &'static [Video] {
    match index {
        0 => GEOGRAPHY_VIDEOS,
        2 => ART_VIDEOS,
        _ => &[],
    }
}

pub fn video_count(index: usize) -> usize {
    collection(index).len()
}

fn embed_url(id: &str) -> String {
    let origin = web_sys::window()
        .and_then(|window| window.location().origin().ok())
        .unwrap_or_default();
    format!(
        "https://www.youtube-nocookie.com/embed/{id}?autoplay=1&playsinline=1&rel=0&origin={origin}"
    )
}

#[component]
pub fn FeaturedVideo() -> impl IntoView {
    let loaded = RwSignal::new(false);
    let video = GEOGRAPHY_VIDEOS[0];
    view! {
        <div class="featured-performance" id="home-performance">
            <div class="featured-player">
                <Show when=move || loaded.get() fallback=move || view! {
                    <button class="featured-play" type="button" aria-label="Play the Geography Gameshow introduction" on:click=move |_| loaded.set(true)>
                        <img src="public/show-preview.webp" alt="" width="720" height="395" loading="lazy" decoding="async"/>
                        <span class="featured-play-symbol" aria-hidden="true">"▶"</span>
                        <span class="featured-play-copy"><span>"A BLANK PAGE. A LIVE AUDIENCE."</span><strong>"Watch the world take shape."</strong></span>
                        <span class="featured-duration">{video.duration}</span>
                    </button>
                }>
                    <iframe src=embed_url(video.id) title="Geography Gameshow introduction — YouTube video player"
                        allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                        allowfullscreen referrerpolicy="strict-origin-when-cross-origin"></iframe>
                </Show>
            </div>
            <div class="featured-caption"><span>"Neal Nichols Jr. · The original live show"</span><a href="geography-videos.html">"All show videos ↗"</a></div>
        </div>
    }
}

fn reveal_player() {
    if let Some(window) = web_sys::window()
        && window
            .inner_width()
            .ok()
            .and_then(|width| width.as_f64())
            .is_some_and(|width| width <= 900.0)
        && let Some(player) = window
            .document()
            .and_then(|document| document.get_element_by_id("program-video-player"))
    {
        player.scroll_into_view();
    }
}

#[component]
pub fn VideoPage(index: usize) -> impl IntoView {
    let program = crate::programs::PROGRAMS[index];
    let (title, description) = match index {
        0 => (
            "A world in motion.",
            "Watch the Geography Gameshow unfold, from the first line to a map full of stories.",
        ),
        2 => (
            "Draw along with Neal.",
            "A closer look at the artist at work. Explore drawing demonstrations and lessons from Neal’s video archive.",
        ),
        _ => (
            "A new story taking shape.",
            "The Anatomy show is in development. There are no anatomy videos to share just yet.",
        ),
    };
    view! {
        <article class="videos-page wrap page-section">
            <nav class="breadcrumbs" aria-label="Breadcrumb"><a href="programs.html">"Programs"</a><span aria-hidden="true">"/"</span><a href=format!("{}.html", program.slug)>{program.name}</a><span aria-hidden="true">"/"</span><span aria-current="page">"Videos"</span></nav>
            <crate::programs::ProgramSubnav index=index active="videos"/>
            <div class="photos-heading video-heading"><div><p class="eyebrow">"THE VIDEO COLLECTION"</p><h1>{title}</h1></div><p>{description}</p></div>
            {if video_count(index) > 0 {
                view! { <ProgramVideos index=index/> }.into_any()
            } else {
                view! { <section class="video-empty"><img src="public/anatomy.svg" width="180" height="180" alt=""/><div><p class="eyebrow">"ANATOMY / IN DEVELOPMENT"</p><h2>"More to discover in the meantime."</h2><p>"Explore Neal’s anatomy drawings, or watch the Geography Gameshow and art demonstrations."</p><div class="program-actions"><a class="button" href="anatomy-photos.html">"Explore anatomy photos"<span>"↗"</span></a><a class="text-link" href="art-videos.html">"Watch an art demonstration"<span>"→"</span></a></div></div></section> }.into_any()
            }}
            <div class="page-signpost"><a class="text-link" href=format!("{}.html", program.slug)><span>"←"</span>{format!("Back to {}", program.name)}</a><a class="text-link" href=format!("booking.html?program={}", program.slug)>"Bring the experience to your school"<span>"↗"</span></a></div>
        </article>
    }
}

#[component]
fn ProgramVideos(index: usize) -> impl IntoView {
    let videos = collection(index);
    let current = RwSignal::new(0usize);
    let loaded = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let selected = move || videos[current.get()];
    let matches = Memo::new(move |_| {
        let search = query.get().trim().to_lowercase();
        videos
            .iter()
            .enumerate()
            .filter(|(_, video)| {
                format!("{} {} {}", video.title, video.description, video.channel)
                    .to_lowercase()
                    .contains(&search)
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    });
    let embed = move || embed_url(selected().id);
    let poster = if index == 0 {
        "public/globe.svg"
    } else {
        "public/art-discovery.svg"
    };
    let name = crate::programs::PROGRAMS[index].name;
    view! {
        <section class="video-library" aria-label=format!("{name} video library")>
            <div class="video-stage">
                <div id="program-video-player" class="video-screen" class:video-loaded=move || loaded.get()>
                    <Show when=move || loaded.get() fallback=move || view! {
                        <button class="video-start" type="button" on:click=move |_| loaded.set(true) aria-label=move || format!("Play {}", selected().title)>
                            <img class="video-poster-art" src=poster alt="" width="540" height="540"/>
                            <span class="video-poster-kicker">"NEAL NICHOLS JR. / ON FILM"</span>
                            <span class="video-play-icon" aria-hidden="true">"▶"</span>
                            <span class="video-poster-action">"Watch the video"</span>
                            <span class="video-poster-meta">"Press play to begin · YouTube"</span>
                        </button>
                    }>
                        <iframe class="video-embed" src=embed title=move || format!("{} — YouTube video player", selected().title)
                            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                            allowfullscreen referrerpolicy="strict-origin-when-cross-origin"></iframe>
                    </Show>
                </div>
                <div class="video-information" aria-live="polite" aria-atomic="true"><p class="eyebrow">{move || format!("VIDEO {:02} / {:02}", current.get()+1, videos.len())}</p><h2>{move || selected().title}</h2><p class="video-description">{move || selected().description}</p><div class="video-credit"><span>{move || format!("From {} · {}", selected().channel, selected().duration)}</span><a href=move || format!("https://www.youtube.com/watch?v={}", selected().id) target="_blank" rel="noopener noreferrer">"Watch on YouTube ↗"</a></div></div>
            </div>
            <div class="video-catalog">
                <div class="video-catalog-heading"><h2>"Choose a video"</h2><span>{videos.len()}</span></div>
                <label class="sr-only" for="video-search">"Search videos"</label>
                <div class="video-search"><input id="video-search" type="search" placeholder="Find a video…" prop:value=move || query.get() on:input=move |event| query.set(event_target_value(&event))/><button type="button" disabled=move || query.get().is_empty() on:click=move |_| query.set(String::new())>"Clear"</button></div>
                <p class="video-results" role="status">{move || format!("{} of {} {}", matches.with(Vec::len), videos.len(), if videos.len() == 1 { "video" } else { "videos" })}</p>
                <ol class="video-playlist" aria-label="Available videos">
                    <For each=move || matches.get() key=|i| *i children=move |i| {
                        let video = videos[i];
                        view! { <li><button type="button" class="video-choice" aria-label=format!("Play {}", video.title) aria-pressed=move || (current.get() == i).to_string() on:click=move |_| { current.set(i); loaded.set(true); reveal_player(); }>
                            <span class="video-choice-icon" aria-hidden="true">{move || if current.get() == i { "▶".to_string() } else { format!("{:02}", i+1) }}</span>
                            <span class="video-choice-copy"><strong>{video.title}</strong><span>{video.channel}</span></span><span class="video-duration">{video.duration}</span>
                        </button></li> }
                    }/>
                </ol>
                <Show when=move || matches.with(Vec::is_empty)><div class="video-no-results"><p>"No videos match that search."</p><button type="button" on:click=move |_| query.set(String::new())>"Show all videos"</button></div></Show>
            </div>
        </section>
    }
}
