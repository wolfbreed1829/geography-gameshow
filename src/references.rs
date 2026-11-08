use leptos::prelude::*;

// Names and review excerpts are transcribed from the supplied reference collection.
// See docs/REFERENCES.md for evidence, deduplication, and unresolved identities.
#[derive(Clone, Copy)]
enum ReferenceText {
    Review(&'static str),
    Coverage(&'static str),
    Mention(&'static str),
}

impl ReferenceText {
    fn action(self) -> &'static str {
        match self {
            Self::Review(_) => "Read review",
            Self::Coverage(_) => "Read coverage",
            Self::Mention(_) => "Read mention",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Review(_) => "Review excerpt",
            Self::Coverage(_) => "Coverage summary",
            Self::Mention(_) => "Community mention",
        }
    }

    fn body(self) -> &'static str {
        match self {
            Self::Review(text) | Self::Coverage(text) | Self::Mention(text) => text,
        }
    }
}

#[derive(Clone, Copy)]
struct Reference {
    name: &'static str,
    category: usize,
    context: &'static str,
    detail: &'static str,
    text: ReferenceText,
}

const REFERENCES: [Reference; 17] = [
    Reference {
        name: "Kitebrook Preparatory School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Instagram post",
        text: ReferenceText::Review(
            "Kitebrook children were transported across the world today when Neal Nichols Jr paid them a visit. He interacted with the children in a unique and memorable way by using the skill of drawing. Mr Nichols Jr drew the European and British maps entirely from memory, which was awe-inspiring and then he used these to impart geographical and historical knowledge in an exciting and engaging way.",
        ),
    },
    Reference {
        name: "Sea Girt Elementary School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Instagram posts",
        text: ReferenceText::Review(
            "Thank you @geographygameshow for returning to Sea Girt! Your incredible drawings and geography lessons have sparked our imaginations!",
        ),
    },
    Reference {
        name: "Westbrook Hay Prep School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School website report · April 2022",
        text: ReferenceText::Review(
            "We were very proud of our pupils who were clearly captivated by Neal’s Gameshow and when asked to participate, they were very keen to show off their geographical knowledge. Neal drew, freehand, a map of the world and then asked the children questions, testing their knowledge. It was a dynamic and fascinating way to learn more about the geography of the world.",
        ),
    },
    Reference {
        name: "Totnes Independent School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Facebook post",
        text: ReferenceText::Review(
            "We were extremely lucky to have a visit from a Geography specialist teacher Neal Nichols, who travels the schools of the world delivering a ‘Geography Game Show’. Neal has memorised how to draw the entire map of the world by hand, and teaches a wealth of knowledge as he does so.",
        ),
    },
    Reference {
        name: "Cushing School PTO",
        category: 0,
        context: "PARENT–TEACHER ORGANIZATION",
        detail: "PTO recommendations",
        text: ReferenceText::Review(
            "Grades 3-5 had the opportunity to experience Neal Nichols Jr. in action today for the Geography Gameshow assemblies. He was incredible! The students were engaged the ENTIRE time. The teachers enjoyed it.",
        ),
    },
    Reference {
        name: "H. W. Mountz School",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Instagram post",
        text: ReferenceText::Review(
            "Huge thanks to the talented Neal Nichols @geographygameshow for presenting to our students today...",
        ),
    },
    Reference {
        name: "Buttershaw Business and Enterprise College",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School Facebook post",
        text: ReferenceText::Review(
            "This week Y7 & 8 took part in the incredible Geography Gameshow from our American friend Neal Nichols. Guided by a world map Neal draws from memory, students were tested on their ability to remember in order to win prizes gathered from all over the World! Neal was amazed at our students’ enthusiasm and knowledge!",
        ),
    },
    Reference {
        name: "Naumann Elementary",
        category: 0,
        context: "REPORTED SCHOOL VISIT",
        detail: "Named in a news clipping",
        text: ReferenceText::Coverage(
            "A clipping titled ‘Geography game show comes to Naumann’ reports Neal’s visit to the school. The caption describes third graders learning the 50 U.S. states and ways to remember them as Neal draws a map from memory.",
        ),
    },
    Reference {
        name: "Nome Elementary School",
        category: 0,
        context: "REPORTED SCHOOL VISIT",
        detail: "Featured in The Nome Nugget · September 2010",
        text: ReferenceText::Coverage(
            "The Nome Nugget reported on Neal’s September 2010 visit to Nome Elementary School. Younger students explored a map of Alaska, while older students worked with a map of the Lower 48. The sessions combined maps drawn from memory with questions and prizes from Neal’s travels.",
        ),
    },
    Reference {
        name: "CIS",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School post · Name shown as CIS",
        text: ReferenceText::Review(
            "The Map Guy, Neal Nichols, brought his \"Geography Gameshow\" to CIS! The international traveler and artist brought his engaging performance to assist 4th and 5th graders with Social Studies skills! Thank you, Mr. Nichols!",
        ),
    },
    Reference {
        name: "South",
        category: 0,
        context: "SCHOOL VISIT",
        detail: "School report · Name shown as South",
        text: ReferenceText::Review(
            "Neal Nichols Jr. presented \"The Geography Game Show\". Mr. Nichols drew this map freehand and shared a ton of geography information while doing so. The students were intrigued and enthralled the entire time.",
        ),
    },
    Reference {
        name: "Friends of Cape Cod National Seashore",
        category: 1,
        context: "COMMUNITY EVENT",
        detail: "Tuesdays at Salt Pond · Event acknowledgment",
        text: ReferenceText::Review(
            "Thank you Neal Nichols for showing us our place on the planet at our Tuesdays at Salt Pond event. The Geography Game Show was fun, entertaining and educational.",
        ),
    },
    Reference {
        name: "Wellfleet Rec",
        category: 1,
        context: "COMMUNITY ARTWORK",
        detail: "Recognition of Neal’s skateboard park artwork",
        text: ReferenceText::Mention(
            "Wellfleet Rec recognized Neal as the artist behind T-shirt artwork for the Kevin Fitzgerald Skateboard Park. The post highlights his creative contribution to the local recreation community.",
        ),
    },
    Reference {
        name: "Jefferey Clayton",
        category: 1,
        context: "PERSONAL RECOMMENDATION",
        detail: "Public Facebook recommendation",
        text: ReferenceText::Review(
            "Nice knowing someone able to travel around the world doing what he loves. Teachers? Admin? History? Art? Science? Social Studies? Something to inspire the interest of learning.",
        ),
    },
    Reference {
        name: "The Nome Nugget",
        category: 2,
        context: "ALASKA · SEPTEMBER 2010",
        detail: "Quick-draw delivers the world to Nome students",
        text: ReferenceText::Coverage(
            "Tyler Rhodes’s article, published September 23, 2010, follows Neal’s visit to Nome Elementary School after presentations at Bering Strait schools. It describes maps drawn from memory, geography questions, and prizes collected on his travels. Younger students explored Alaska, while older students took on the Lower 48.",
        ),
    },
    Reference {
        name: "Cook Islands News",
        category: 2,
        context: "COOK ISLANDS · PRESS COVERAGE",
        detail: "Visitor turns learning into lots of laughs",
        text: ReferenceText::Coverage(
            "A Cook Islands News clipping features Neal’s classroom presentation under the headline ‘Visitor turns learning into lots of laughs.’",
        ),
    },
    Reference {
        name: "Digo Dristi",
        category: 2,
        context: "NEPAL · WEEKLY NEWSPAPER",
        detail: "Coverage of Neal’s school visits in Nepal",
        text: ReferenceText::Coverage(
            "The Nepali weekly Digo Dristi covered Neal’s school visits in Nepal and his use of drawing to teach geography.",
        ),
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
            <div class="references-heading"><div><p class="eyebrow">"CLASSROOMS. COMMUNITIES. CONNECTIONS."</p><h1 id="references-title">"References "<span class="serif-line">"& press."</span></h1></div><p>"In their own words. Read reviews from schools and communities, and explore stories about Neal’s work around the world."</p></div>
            <div class="reference-filters" role="group" aria-label="Browse references">
                {GROUPS.into_iter().enumerate().map(|(i, (label, _, _))| view! {
                    <button type="button" aria-controls="reference-results" aria-pressed=move || (category.get() == i).to_string() class:active=move || category.get() == i on:click=move |_| category.set(i)>{label}<span class="reference-count">{REFERENCES.iter().filter(|r| r.category == i).count()}</span></button>
                }).collect_view()}
            </div>
            <div class="reference-directory" id="reference-results">
                <div class="reference-section-heading"><div><h2>{move || GROUPS[category.get()].1}</h2><p>{move || GROUPS[category.get()].2}</p></div><span class="directory-caption" role="status">{move || format!("{} · {} entries", GROUPS[category.get()].0, REFERENCES.iter().filter(|r| r.category == category.get()).count())}</span></div>
                <ul class="reference-list" aria-label=move || GROUPS[category.get()].0>
                    <For
                        each={move || REFERENCES.into_iter().filter(|r| r.category == category.get()).collect::<Vec<_>>()}
                        key={|r| r.name}
                        children={|r| view! {
                        <li class="reference-entry">
                            <p class="reference-context">{r.context}</p><h3>{r.name}</h3><p class="reference-detail">{r.detail}</p>
                            <details class="reference-review" name="reference-review">
                                <summary>{r.text.action()}<span class="sr-only">{format!(" from {}", r.name)}</span><span class="review-toggle" aria-hidden="true">"+"</span></summary>
                                <div class="reference-review-body">
                                    <p class="review-label">{r.text.label()}</p>
                                    {if matches!(r.text, ReferenceText::Review(_)) {
                                        view! { <blockquote>{r.text.body()}</blockquote> }.into_any()
                                    } else {
                                        view! { <p class="coverage-text">{r.text.body()}</p> }.into_any()
                                    }}
                                </div>
                            </details>
                        </li>
                        }}
                    />
                </ul>
            </div>
            <div class="references-invitation"><div><p class="eyebrow">"YOUR SCHOOL COULD BE NEXT"</p><h2>"Let’s bring the world to you."</h2></div><a class="button" href="booking.html">"Inquire about a visit"<span>"↗"</span></a></div>
        </section>
    }
}
