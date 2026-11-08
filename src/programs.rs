use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct Program {
    pub slug: &'static str,
    pub name: &'static str,
    pub label: &'static str,
    pub headline: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub image_alt: &'static str,
    pub features: [(&'static str, &'static str); 3],
}

pub const PROGRAMS: [Program; 3] = [
    Program {
        slug: "geography",
        name: "Geography",
        label: "THE ORIGINAL LIVE EXPERIENCE",
        headline: "A world unfolds. Right before your eyes.",
        description: "Neal draws maps entirely from memory, weaving geography, history, and listening skills into a show that invites students to join in.",
        image: "public/programs/geography-3d-1000.webp",
        image_alt: "Cinematic globe with cobalt oceans, earthy continents, and illuminated orbital rings",
        features: [
            (
                "A map for every age",
                "Existing programs span kindergarten through high school, from local maps to world geography and history.",
            ),
            (
                "Made to take part",
                "Students follow clues and identify places as the drawing develops. Smaller groups offer more opportunities to participate.",
            ),
            (
                "Connected to your classroom",
                "Discuss your grade level, current lessons, and preferred region with Neal to find the right program.",
            ),
        ],
    },
    Program {
        slug: "anatomy",
        name: "Anatomy",
        label: "NEW PROGRAM / IN DEVELOPMENT",
        headline: "A whole world of wonder. Inside you.",
        description: "A proposed live-drawing journey through the human body: how bones support us, muscles move us, and organs work together. Neal is developing the topics, age ranges, and format for this new show.",
        image: "public/programs/anatomy-3d-1000.webp",
        image_alt: "Sculptural anatomy illustration with a translucent torso, ribs, lungs, and heart",
        features: [
            (
                "Bones & movement",
                "Explore a proposed introduction to the skeleton, joints, and muscles through clear, memorable drawings.",
            ),
            (
                "Organs & connections",
                "Discover how the heart, lungs, and other parts of the body fit together. Final lesson topics are still being developed.",
            ),
            (
                "A new show taking shape",
                "Share your students’ ages and what they are learning. Neal can discuss the developing concept and future possibilities.",
            ),
        ],
    },
    Program {
        slug: "art",
        name: "Art",
        label: "NEW SHOW CONCEPT / IN DEVELOPMENT",
        headline: "Every great idea starts with a mark.",
        description: "Building on Neal’s background in illustration and drawing classes, this proposed show explores how artists see. Think line, perspective, texture, and the confidence to create. The new show format is still taking shape.",
        image: "public/programs/art-3d-1000.webp",
        image_alt: "Cinematic artist’s workspace with a sketchbook, pencil, and sculptural ribbons of color",
        features: [
            (
                "Learn to look",
                "Explore line, shape, and the details that make an everyday subject worth drawing.",
            ),
            (
                "Bring a page to life",
                "Perspective, light, shadow, and texture are possible themes for the new experience.",
            ),
            (
                "Make room for creativity",
                "Ask Neal about the developing show and how it could connect with your group’s interests.",
            ),
        ],
    },
];

#[component]
pub fn Programs() -> impl IntoView {
    view! {
        <section class="programs-page wrap page-section" aria-labelledby="programs-title">
            <div class="section-heading"><div><p class="eyebrow">"CHOOSE YOUR ADVENTURE"</p><h1 id="programs-title">"Three worlds."<br/>"One curious mind."</h1></div><p>"Explore the original Geography Gameshow and the new Anatomy and Art concepts. Each has its own story to discover."</p></div>
            <div class="show-cards">
                {PROGRAMS.into_iter().enumerate().map(|(index, p)| view! { <DiscoveryCard index=index program=p/> }).collect_view()}
            </div>
            <div class="page-signpost"><span>"Planning a school visit?"</span><a class="text-link" href="planning.html">"Find the practical details"<span>"↗"</span></a></div>
        </section>
    }
}

#[component]
fn DiscoveryCard(index: usize, program: Program) -> impl IntoView {
    let p = program;
    let card = NodeRef::<leptos::html::A>::new();
    let tilt = RwSignal::new((0.0_f64, 0.0_f64));
    view! {
        <a node_ref=card class=format!("show-card discovery-card {}-card", p.slug) href=format!("{}.html", p.slug)
            style=move || { let (x, y) = tilt.get(); format!("--rx:{:.3}deg;--ry:{:.3}deg;--mx:{:.2}%;--my:{:.2}%;--px:{:.2}px;--py:{:.2}px", -y*4.0, x*5.0, 50.0+x*40.0, 50.0+y*40.0, x*7.0, y*5.0) }
            on:pointermove=move |event| {
                if event.pointer_type() == "mouse" && let Some(element) = card.get() {
                    let bounds = element.get_bounding_client_rect();
                    if bounds.width() > 0.0 && bounds.height() > 0.0 {
                        tilt.set((((event.client_x() as f64 - bounds.left()) / bounds.width() * 2.0 - 1.0).clamp(-1.0, 1.0), ((event.client_y() as f64 - bounds.top()) / bounds.height() * 2.0 - 1.0).clamp(-1.0, 1.0)));
                    }
                }
            }
            on:pointerleave=move |_| tilt.set((0.0, 0.0)) on:blur=move |_| tilt.set((0.0, 0.0))>
            <div class="discovery-scene" aria-hidden="true">
                <div class="discovery-artwork"><img class="discovery-illustration"
                    src=format!("public/programs/{}-3d-1000.webp", p.slug)
                    srcset=format!("public/programs/{0}-3d-640.webp 640w, public/programs/{0}-3d-1000.webp 1000w, public/programs/{0}-3d-1672.webp 1672w", p.slug)
                    sizes="(max-width: 680px) 135vw, (max-width: 1000px) 65vw, 620px"
                    alt="" width="1672" height="941" decoding="async"/></div>
                <div class="discovery-sheen"></div>
                <div class="discovery-scene-top"><span class="discovery-number">{format!("0{}", index + 1)}</span><span class="discovery-edition">{if index == 0 { "THE ORIGINAL LIVE SHOW" } else { "A NEW WORLD IN THE MAKING" }}</span></div>
                <span class="discovery-note">{match index { 0 => "A world drawn from memory", 1 => "Discover the world within", _ => "See it. Imagine it. Create it." }}</span>
            </div>
            <div class="discovery-copy">
                <div class="discovery-heading"><h2>{p.name}</h2><span class="discovery-arrow" aria-hidden="true">"↗"</span></div>
                <p>{match index { 0 => "One blank page. The whole world. Watch a map come to life, entirely from memory.", 1 => "An extraordinary journey through bones, muscles, and the amazing world inside us.", _ => "A fresh way to look at the everyday. Follow a line and see where it takes you." }}</p>
                <div class="discovery-topics" aria-hidden="true">{match index { 0 => "MAPS · HISTORY · DISCOVERY", 1 => "BODY · MOVEMENT · CONNECTION", _ => "LINE · PERSPECTIVE · POSSIBILITY" }}</div>
                <div class="discovery-bottom"><span>{format!("Explore {}", p.name)}</span><span class="discovery-status">{if index == 0 { "LIVE EXPERIENCE" } else { "IN DEVELOPMENT" }}</span></div>
            </div>
            <span class="discovery-glare" aria-hidden="true"></span>
        </a>
    }
}

#[component]
pub fn ProgramPage(index: usize) -> impl IntoView {
    let p = PROGRAMS[index];
    view! {
        <article class=format!("program-page program-{} wrap page-section", p.slug)>
            <nav class="breadcrumbs" aria-label="Breadcrumb"><a href="programs.html">"Programs"</a><span aria-hidden="true">"/"</span><span aria-current="page">{p.name}</span></nav>
            <ProgramSubnav index=index active="overview"/>
            <div class="program-intro"><div><p class="eyebrow">{p.label}</p><h1>{p.name}</h1><h2>{p.headline}</h2><p class="program-description">{p.description}</p><div class="program-actions"><a class="button" href=format!("booking.html?program={}", p.slug)>{if index == 0 { "Ask about a show" } else { "Ask about the concept" }}<span>"↗"</span></a><a class="text-link" href="challenge.html">"Try a challenge"<span>"→"</span></a></div></div>
                <figure class="program-visual program-visual-cinematic">
                    <img src=p.image
                        srcset=format!("public/programs/{0}-3d-640.webp 640w, public/programs/{0}-3d-1000.webp 1000w, public/programs/{0}-3d-1672.webp 1672w", p.slug)
                        sizes="(max-width: 680px) 95vw, (max-width: 1100px) 55vw, 760px"
                        alt=p.image_alt width="1672" height="941" decoding="async"/>
                    <figcaption><span>{format!("0{} / {}", index + 1, p.name)}</span><span>{match index { 0 => "Our planet", 1 => "The world within", _ => "Your imagination" }}</span></figcaption>
                </figure>
            </div>
            <div class="program-features">{p.features.into_iter().enumerate().map(|(i,(heading,text))| view! {<section><span class="feature-number">{format!("0{}",i+1)}</span><h3>{heading}</h3><p>{text}</p></section>}).collect_view()}</div>
            <div class="page-signpost"><a class="text-link" href="programs.html"><span>"←"</span>"All programs"</a><a class="text-link" href=if index == 0 { "in-action.html" } else { "meet-neal.html" }>{if index == 0 { "See the original show in action" } else { "Meet the artist behind the show" }}<span>"↗"</span></a></div>
        </article>
    }
}

#[component]
pub fn ProgramSubnav(index: usize, active: &'static str) -> impl IntoView {
    let p = PROGRAMS[index];
    view! {
        <nav class="program-subnav" aria-label=format!("{} sections", p.name)>
            <a href=format!("{}.html", p.slug) aria-current=(active == "overview").then_some("page")>"Overview"</a>
            <a href=format!("{}-photos.html", p.slug) aria-current=(active == "photos").then_some("page")>"Photos"<span class="subnav-count">{crate::gallery::photo_count(index)}</span></a>
            <a href=format!("{}-videos.html", p.slug) aria-current=(active == "videos").then_some("page")>"Videos"<span class="subnav-count">{crate::videos::video_count(index)}</span></a>
        </nav>
    }
}

#[component]
pub fn PhotoPage(index: usize) -> impl IntoView {
    let p = PROGRAMS[index];
    let (title, description) = match index {
        0 => (
            "Geography in pictures.",
            "From the first outline to a world full of stories. Take a closer look at Neal’s live drawings.",
        ),
        1 => (
            "Anatomy, illustrated.",
            "A look at Neal’s drawing work as a new program about the human body takes shape.",
        ),
        _ => (
            "Art, up close.",
            "Artwork, drawing demonstrations, and a look inside Neal’s studio. Explore the details that make us look again.",
        ),
    };
    view! {
        <article class="photos-page wrap page-section">
            <nav class="breadcrumbs" aria-label="Breadcrumb"><a href="programs.html">"Programs"</a><span aria-hidden="true">"/"</span><a href=format!("{}.html", p.slug)>{p.name}</a><span aria-hidden="true">"/"</span><span aria-current="page">"Photos"</span></nav>
            <ProgramSubnav index=index active="photos"/>
            <div class="photos-heading"><h1>{title}</h1><p>{description}</p></div>
            <crate::gallery::ProgramGallery index=index/>
            <div class="page-signpost"><a class="text-link" href=format!("{}.html", p.slug)><span>"←"</span>{format!("Back to {}", p.name)}</a><a class="text-link" href=format!("booking.html?program={}", p.slug)>"Start a conversation"<span>"↗"</span></a></div>
        </article>
    }
}
