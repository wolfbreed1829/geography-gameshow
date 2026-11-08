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
        image: "public/globe.svg",
        image_alt: "Illustrated globe with cobalt oceans and earthy green continents",
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
        image: "public/anatomy.svg",
        image_alt: "Stylized front-view illustration of the human torso with ribs, lungs, and heart",
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
        image: "public/art-tools.svg",
        image_alt: "An illustrated sketchbook, geometric drawing, and pencil",
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
                {PROGRAMS.into_iter().enumerate().map(|(index, p)| view! {
                    <a class=format!("show-card discovery-card {}-card", p.slug) href=format!("{}.html", p.slug)>
                        <div class="discovery-scene" aria-hidden="true">
                            <div class="discovery-grid"></div>
                            <div class="discovery-orbit orbit-a"></div><div class="discovery-orbit orbit-b"></div>
                            <span class="discovery-number">{format!("0{}", index + 1)}</span>
                            <span class="discovery-edition">{if index == 0 { "THE ORIGINAL LIVE SHOW" } else { "A NEW WORLD IN THE MAKING" }}</span>
                            <img class="discovery-illustration" src=if index == 2 { "public/art-discovery.svg" } else { p.image } alt="" width="540" height="540"/>
                            <span class="discovery-note">{match index { 0 => "A world drawn from memory", 1 => "Discover the world within", _ => "See it. Imagine it. Create it." }}</span>
                            <span class="discovery-cross cross-one">"+"</span><span class="discovery-cross cross-two">"+"</span>
                            <span class="discovery-coordinate">{match index { 0 => "EXPLORE / OUR PLANET", 1 => "EXPLORE / THE HUMAN BODY", _ => "EXPLORE / YOUR IMAGINATION" }}</span>
                        </div>
                        <div class="discovery-copy">
                            <div class="discovery-heading"><h2>{p.name}</h2><span class="discovery-arrow" aria-hidden="true">"↗"</span></div>
                            <p>{match index { 0 => "One blank page. The whole world. Watch a map come to life, entirely from memory.", 1 => "An extraordinary journey through bones, muscles, and the amazing world inside us.", _ => "A fresh way to look at the everyday. Follow a line and see where it takes you." }}</p>
                            <div class="discovery-topics" aria-hidden="true">{match index { 0 => "MAPS · HISTORY · DISCOVERY", 1 => "BODY · MOVEMENT · CONNECTION", _ => "LINE · PERSPECTIVE · POSSIBILITY" }}</div>
                            <div class="discovery-bottom"><span>{format!("Explore {}", p.name)}</span><span class="discovery-status">{if index == 0 { "LIVE EXPERIENCE" } else { "IN DEVELOPMENT" }}</span></div>
                        </div>
                    </a>
                }).collect_view()}
            </div>
            <div class="page-signpost"><span>"Planning a school visit?"</span><a class="text-link" href="planning.html">"Find the practical details"<span>"↗"</span></a></div>
        </section>
    }
}

#[component]
pub fn ProgramPage(index: usize) -> impl IntoView {
    let p = PROGRAMS[index];
    view! {
        <article class=format!("program-page program-{} wrap page-section", p.slug)>
            <nav class="breadcrumbs" aria-label="Breadcrumb"><a href="programs.html">"Programs"</a><span aria-hidden="true">"/"</span><span aria-current="page">{p.name}</span></nav>
            <ProgramSubnav index=index photos=false/>
            <div class="program-intro"><div><p class="eyebrow">{p.label}</p><h1>{p.name}</h1><h2>{p.headline}</h2><p class="program-description">{p.description}</p><div class="program-actions"><a class="button" href=format!("booking.html?program={}", p.slug)>{if index == 0 { "Ask about a show" } else { "Ask about the concept" }}<span>"↗"</span></a><a class="text-link" href="challenge.html">"Try a challenge"<span>"→"</span></a></div></div><div class="program-visual"><img src=p.image alt=p.image_alt width="540" height="540"/></div></div>
            <div class="program-features">{p.features.into_iter().enumerate().map(|(i,(heading,text))| view! {<section><span class="feature-number">{format!("0{}",i+1)}</span><h3>{heading}</h3><p>{text}</p></section>}).collect_view()}</div>
            <div class="page-signpost"><a class="text-link" href="programs.html"><span>"←"</span>"All programs"</a><a class="text-link" href=if index == 0 { "in-action.html" } else { "meet-neal.html" }>{if index == 0 { "See the original show in action" } else { "Meet the artist behind the show" }}<span>"↗"</span></a></div>
        </article>
    }
}

#[component]
fn ProgramSubnav(index: usize, photos: bool) -> impl IntoView {
    let p = PROGRAMS[index];
    view! {
        <nav class="program-subnav" aria-label=format!("{} sections", p.name)>
            <a href=format!("{}.html", p.slug) aria-current=(!photos).then_some("page")>"Overview"</a>
            <a href=format!("{}-photos.html", p.slug) aria-current=photos.then_some("page")>"Photos"<span class="subnav-count">{crate::gallery::photo_count(index)}</span></a>
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
            <ProgramSubnav index=index photos=true/>
            <div class="photos-heading"><h1>{title}</h1><p>{description}</p></div>
            <crate::gallery::ProgramGallery index=index/>
            <div class="page-signpost"><a class="text-link" href=format!("{}.html", p.slug)><span>"←"</span>{format!("Back to {}", p.name)}</a><a class="text-link" href=format!("booking.html?program={}", p.slug)>"Start a conversation"<span>"↗"</span></a></div>
        </article>
    }
}
