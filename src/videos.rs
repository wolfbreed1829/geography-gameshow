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
        1 => ART_VIDEOS,
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

#[component]
pub fn VideoPage(index: usize) -> impl IntoView {
    let p = crate::programs::PROGRAMS[index];
    view! {<article class="screen-page wrap"><h1>{format!("{} videos",p.name)}</h1>
        <crate::programs::ProgramSubnav index=index active="videos"/>
        <ProgramVideos index=index/>
    </article>}
}

#[component]
fn ProgramVideos(index: usize) -> impl IntoView {
    let videos = collection(index);
    let current = RwSignal::new(0usize);
    let loaded = RwSignal::new(false);
    let query = RwSignal::new(String::new());
    let mode = RwSignal::new(0usize);
    let selected = move || videos[current.get()];
    let matches = Memo::new(move |_| {
        let q = query.get().trim().to_lowercase();
        videos
            .iter()
            .enumerate()
            .filter(|(_, v)| {
                format!("{} {} {}", v.title, v.description, v.channel)
                    .to_lowercase()
                    .contains(&q)
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
    });
    let panels=vec![
        ("Watch",view!{
            <label class="video-picker"><span class="sr-only">"Choose a video"</span><select prop:value=move || current.get().to_string() on:change=move |e|{if let Ok(i)=event_target_value(&e).parse::<usize>(){current.set(i);loaded.set(false);}}>
                {videos.iter().enumerate().map(|(i,v)|view!{<option value=i.to_string()>{v.title}</option>}).collect_view()}
            </select></label>
            <div class="contained-player">
                <Show when=move || loaded.get() fallback=move ||view!{<button class="video-start" type="button" aria-label=move ||format!("Play {}",selected().title) on:click=move |_|loaded.set(true)>
                    <img src=if index==0 {"public/photos/complete-world-map.webp"} else {"public/art-discovery.svg"} alt="" width="540" height="540"/><span>"Play video"</span>
                </button>}>
                    <iframe class="video-embed" src=move ||embed_url(selected().id) title=move ||selected().title allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen referrerpolicy="strict-origin-when-cross-origin"></iframe>
                </Show>
            </div>
            <p class="video-current-credit">{move ||format!("{} · {}",selected().channel,selected().duration)}</p>
        }.into_any()),
        ("Details",view!{<div class="video-detail-reader">
            {move || {let v=selected();view!{<crate::screen::Reader text=format!("{}\n\n{}\n\n{} · {}",v.title,v.description,v.channel,v.duration)/>}}}
            <div class="screen-links"><a href=move ||format!("https://www.youtube.com/watch?v={}",selected().id) target="_blank" rel="noopener noreferrer">"Watch on YouTube"</a></div>
        </div>}.into_any()),
        ("Find a video",view!{
            <div class="video-search-panel">
            <label class="sr-only" for="video-search">"Search videos"</label><input id="video-search" type="search" placeholder="Title, topic, or channel" prop:value=move ||query.get() on:input=move |e|query.set(event_target_value(&e))/>
            <p role="status">{move ||format!("{} videos",matches.with(Vec::len))}</p>
            <label><span class="sr-only">"Search results"</span><select size="1" on:change=move |e|{if let Ok(i)=event_target_value(&e).parse::<usize>(){current.set(i);loaded.set(false);mode.set(0);}}>
                <option value="">"Choose a result"</option>
                {move ||matches.get().into_iter().map(|i|view!{<option value=i.to_string()>{videos[i].title}</option>}).collect_view()}
            </select></label>
            <button type="button" on:click=move |_|query.set(String::new())>"Show all videos"</button>
            </div>
        }.into_any())
    ];
    view! {<crate::screen::Deck panels=panels selected=mode/>}
}
