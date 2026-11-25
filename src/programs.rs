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

pub const PROGRAMS: [Program; 2] = [
    Program {
        slug: "geography",
        name: "Geography",
        label: "THE ORIGINAL LIVE EXPERIENCE",
        headline: "A world unfolds. Right before your eyes.",
        description: "Neal draws maps entirely from memory, weaving geography, history, and listening skills into a show that invites students to join in.",
        image: "public/programs/geography-sketchbook.webp",
        image_alt: "Cinematic sketchbook illustration inspired by Neal’s world map, with a pencil and blue-and-gold ribbons",
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
    let panels = PROGRAMS
        .into_iter()
        .enumerate()
        .map(|(index, p)| {
            (
                p.name,
                view! {<DiscoveryCard index=index program=p/>}.into_any(),
            )
        })
        .collect();
    view! {
        <section class="screen-page wrap"><h1>"Programs"</h1>
            <crate::screen::Deck panels=panels/>
            <div class="screen-links"><a href="programs-scheduling.html">"Programs & Scheduling"</a><a href="planning.html">"Plan a school visit"</a></div>
        </section>
    }
}

#[component]
fn DiscoveryCard(index: usize, program: Program) -> impl IntoView {
    let p = program;
    view! {
        <a class=format!("program-choice {}-card",p.slug) href=format!("{}.html",p.slug)>
            <div class="program-card-visual"><img src=p.image alt=p.image_alt width="1672" height="941"/></div>
            <div class="program-choice-copy"><p class="program-choice-label">{if index==0{"DRAWN FROM MEMORY"}else{"IN DEVELOPMENT"}}</p><h2>{p.name}</h2>
                <p>{if index==0{"Live illustration. Real geography. A show for every age."}else{"Drawing, perspective, and the confidence to create."}}</p>
                <span class="program-choice-action">{format!("Explore {}",p.name)}</span>
            </div>
        </a>
    }
}

#[component]
pub fn ProgramPage(index: usize) -> impl IntoView {
    let p = PROGRAMS[index];
    let (artwork_image, artwork_alt) = if index == 0 {
        (
            "public/photos/complete-world-map.webp",
            "Neal Nichols Jr.’s original completed hand-drawn world map",
        )
    } else {
        (p.image, p.image_alt)
    };
    let details = p
        .features
        .iter()
        .map(|(heading, text)| format!("{heading}\n\n{text}"))
        .collect::<Vec<_>>()
        .join("\n\n");
    let mut panels=vec![
        ("Overview",view!{
            <div class="program-overview"><div><h2>{p.headline}</h2>
                <crate::screen::Reader text=format!("{}\n\n{}",p.label,p.description)/>
                <div class="screen-links"><a href="challenge.html">"Try a challenge"</a><a href="programs.html">"All programs"</a></div>
            </div><div class="overview-art"><img src=p.image alt=p.image_alt width="1672" height="941"/></div></div>
        }.into_any()),
        ("Details",view!{<crate::screen::Reader text=details/>}.into_any()),
        ("Artwork",view!{<figure class="screen-media"><div class="art-display" class:original-drawing=index==0><img src=artwork_image alt=artwork_alt width="1171" height="708"/></div><figcaption>{if index==0{"A world drawn from memory · Neal Nichols Jr."}else{"Your imagination"}}</figcaption></figure>}.into_any()),
    ];
    if index == 0 {
        panels.insert(1, ("Curriculum", view! {<Curriculum/>}.into_any()));
        panels.insert(
            2,
            (
                "Program outline",
                view! {<crate::screen::Reader text=format!("{PROGRAM_OUTLINE}\n\nOther titles include {}, and more.", ADDITIONAL_TITLES.join(", "))/>}.into_any(),
            ),
        );
    }
    view! {<article class="screen-page wrap"><h1>{p.name}</h1><ProgramSubnav index=index active="overview"/><crate::screen::Deck panels=panels/></article>}
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
        _ => (
            "Art, up close.",
            "Artwork, drawing demonstrations, and a look inside Neal’s studio. Explore the details that make us look again.",
        ),
    };
    let panels = vec![
        (
            "Photos",
            view! {<crate::gallery::ProgramGallery index=index/>}.into_any(),
        ),
        (
            "About",
            view! {<crate::screen::Reader text=format!("{title}\n\n{description}")/>}.into_any(),
        ),
    ];
    view! {
        <article class="screen-page wrap"><h1>{format!("{} photos",p.name)}</h1>
            <ProgramSubnav index=index active="photos"/>
            <crate::screen::Deck panels=panels/>
        </article>
    }
}

const PROGRAM_OUTLINE: &str = "All programs feature Neal’s illustration and content from memory, and are educational for a variety of audiences. Additional venues include family fun night fundraising shows, libraries, camps, aftercare groups, tourist centers, home school assemblies, and Montessori schools.\n\nThe Geography Gameshow is a practical lesson plan in a game show format. Some students win prizes from various places around the world.\n\nThe curriculum is suggested: shows are interchangeable across grade levels and suitable for multi-age audiences. Schools can host events annually without repeat performances as students progress through the years.";

pub(crate) const ADDITIONAL_TITLES: [&str; 7] = [
    "Alaska History",
    "Texas History",
    "31 States of Mexico",
    "Latin America",
    "Europe Shows",
    "Britain 1066",
    "World Tectonic Plates and Disaster",
];

pub(crate) const CURRICULUM: [(&str, &str, &str); 8] = [
    (
        "K",
        "Drawing and Art\nLocal Regional Show\nCape Cod Introduction",
        "45–60 minutes",
    ),
    (
        "1",
        "New England and the Pilgrims\nUSA Map Introduction",
        "60 minutes",
    ),
    ("2", "World Map Introduction", "75 minutes"),
    (
        "3",
        "Massachusetts History\nUSA Capitals / National Parks",
        "75 minutes",
    ),
    ("4", "USA History", "90 minutes"),
    ("5", "The Original Colonies", "90 minutes"),
    ("6", "Ancient / Middle East", "90 minutes"),
    ("7", "World Map Challenge", "90 minutes"),
];

#[component]
fn Curriculum() -> impl IntoView {
    let grade = RwSignal::new(0usize);
    view! {<div class="curriculum">
        <label class="curriculum-select"><span class="sr-only">"Choose a grade"</span><select prop:value=move ||grade.get().to_string() on:change=move |e|grade.set(event_target_value(&e).parse().unwrap_or(0))>
            {CURRICULUM.iter().enumerate().map(|(i,(g,_,_))|view!{<option value=i.to_string()>{format!("Grade {g}")}</option>}).collect_view()}
        </select></label>
        <article class="curriculum-card"><p>"SUGGESTED CURRICULUM"</p><h2>{move ||format!("Grade {}",CURRICULUM[grade.get()].0)}</h2><p class="curriculum-titles">{move ||CURRICULUM[grade.get()].1}</p><strong>{move ||CURRICULUM[grade.get()].2}</strong></article>
        <nav class="reader-pagination" aria-label="Curriculum grades"><button type="button" disabled=move ||grade.get()==0 on:click=move |_|grade.update(|i|*i=i.saturating_sub(1))>"Previous"</button><span>{move ||format!("{} / 8",grade.get()+1)}</span><button type="button" disabled=move ||grade.get()==7 on:click=move |_|grade.update(|i|*i+=1)>"Next"</button></nav>
    </div>}
}
