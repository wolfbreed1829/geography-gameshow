use leptos::prelude::*;

// Names are transcribed from the supplied reference collection.
// See docs/REFERENCES.md for evidence, deduplication, and unresolved identities.
#[derive(Clone, Copy)]
struct Reference {
    name: &'static str,
    category: usize,
    context: &'static str,
    detail: &'static str,
    link: Option<(&'static str, &'static str)>,
}

const REFERENCES: [Reference; 17] = [
    Reference {
        name: "Kitebrook Preparatory School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Instagram post",
        link: None,
    },
    Reference {
        name: "Sea Girt Elementary School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Instagram posts",
        link: None,
    },
    Reference {
        name: "Westbrook Hay Prep School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School website report · April 2022",
        link: Some((
            "https://www.westbrookhay.co.uk/news-article/geography-gameshow",
            "Read the school’s story",
        )),
    },
    Reference {
        name: "Totnes Independent School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Facebook post",
        link: None,
    },
    Reference {
        name: "Cushing School PTO",
        category: 0,
        context: "PARENT–TEACHER ORGANIZATION",
        detail: "PTO recommendations",
        link: None,
    },
    Reference {
        name: "H. W. Mountz School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Instagram post",
        link: None,
    },
    Reference {
        name: "Buttershaw Business and Enterprise College",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Facebook post",
        link: None,
    },
    Reference {
        name: "Naumann Elementary",
        category: 0,
        context: "REPORTED SCHOOL VISIT",
        detail: "Named in a news clipping",
        link: None,
    },
    Reference {
        name: "Nome Elementary School",
        category: 0,
        context: "REPORTED SCHOOL VISIT",
        detail: "Featured in The Nome Nugget · September 2010",
        link: None,
    },
    Reference {
        name: "CIS",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School post · Name shown as CIS",
        link: None,
    },
    Reference {
        name: "South",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School report · Name shown as South",
        link: None,
    },
    Reference {
        name: "Friends of Cape Cod National Seashore",
        category: 1,
        context: "COMMUNITY EVENT",
        detail: "Tuesdays at Salt Pond · Event acknowledgment",
        link: None,
    },
    Reference {
        name: "Wellfleet Rec",
        category: 1,
        context: "COMMUNITY ARTWORK",
        detail: "Recognition of Neal’s skateboard park artwork",
        link: None,
    },
    Reference {
        name: "Jefferey Clayton",
        category: 1,
        context: "PERSONAL RECOMMENDATION",
        detail: "Public Facebook recommendation",
        link: None,
    },
    Reference {
        name: "The Nome Nugget",
        category: 2,
        context: "ALASKA · SEPTEMBER 2010",
        detail: "Quick-draw delivers the world to Nome students",
        link: Some((
            "https://www.nomenugget.net/archives/2010/9.23.10%20nn.pdf#page=7",
            "Read the newspaper archive (PDF)",
        )),
    },
    Reference {
        name: "Cook Islands News",
        category: 2,
        context: "COOK ISLANDS · PRESS COVERAGE",
        detail: "Visitor turns learning into lots of laughs",
        link: None,
    },
    Reference {
        name: "Digo Dristi",
        category: 2,
        context: "NEPAL · WEEKLY NEWSPAPER",
        detail: "Coverage of Neal’s school visits in Nepal",
        link: None,
    },
];

const GROUPS: [(&str, &str, &str); 3] = [
    (
        "Schools & PTOs",
        "From the classroom.",
        "Schools and parent–teacher groups named in visit reports, recommendations, and news coverage.",
    ),
    (
        "Community",
        "Beyond the classroom.",
        "Community events, artwork, and personal recommendations from Neal’s reference collection.",
    ),
    (
        "In the press",
        "Stories from around the world.",
        "Publications that have covered Neal’s work, from Alaska to the South Pacific and Nepal.",
    ),
];

#[component]
pub fn References() -> impl IntoView {
    let category = RwSignal::new(0_usize);
    view! {
        <section class="references-page wrap page-section" aria-labelledby="references-title">
            <div class="references-heading"><div><p class="eyebrow">"CLASSROOMS. COMMUNITIES. CONNECTIONS."</p><h1 id="references-title">"References "<span class="serif-line">"& press."</span></h1></div><p>"A selection of the schools, community voices, and publications that have shared Neal’s work along the way."</p></div>
            <div class="reference-filters" role="group" aria-label="Browse references">
                {GROUPS.into_iter().enumerate().map(|(i, (label, _, _))| view! {
                    <button type="button" aria-controls="reference-results" aria-pressed=move || (category.get() == i).to_string() class:active=move || category.get() == i on:click=move |_| category.set(i)>{label}<span class="reference-count">{REFERENCES.iter().filter(|r| r.category == i).count()}</span></button>
                }).collect_view()}
            </div>
            <div class="reference-directory" id="reference-results">
                <div class="reference-section-heading"><div><h2>{move || GROUPS[category.get()].1}</h2><p>{move || GROUPS[category.get()].2}</p></div><span class="directory-caption" role="status">{move || format!("{} · {} entries", GROUPS[category.get()].0, REFERENCES.iter().filter(|r| r.category == category.get()).count())}</span></div>
                <ul class="reference-list" aria-label=move || GROUPS[category.get()].0>
                    {move || REFERENCES.into_iter().filter(|r| r.category == category.get()).map(|r| view! {
                        <li class="reference-entry"><p class="reference-context">{r.context}</p><h3>{r.name}</h3><p class="reference-detail">{r.detail}</p>{r.link.map(|(href,label)| view! { <a class="reference-source" href=href target="_blank" rel="noopener noreferrer">{label}<span aria-hidden="true">"↗"</span><span class="sr-only">" (opens in a new tab)"</span></a> })}</li>
                    }).collect_view()}
                </ul>
            </div>
            <div class="references-invitation"><div><p class="eyebrow">"YOUR SCHOOL COULD BE NEXT"</p><h2>"Let’s bring the world to you."</h2></div><a class="button" href="booking.html">"Inquire about a visit"<span>"↗"</span></a></div>
        </section>
    }
}
