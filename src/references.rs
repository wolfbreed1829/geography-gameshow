use leptos::prelude::*;

// Names and review excerpts are transcribed from the supplied reference collection.
// See docs/REFERENCES.md for evidence, deduplication, and unresolved identities.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReferenceText {
    Review(&'static str),
    Coverage(&'static str),
    Mention(&'static str),
    Record(&'static str),
    Listing(&'static str),
}

impl ReferenceText {
    fn action(self) -> &'static str {
        match self {
            Self::Review(_) => "Read review",
            Self::Coverage(_) => "Read coverage",
            Self::Mention(_) => "Read mention",
            Self::Record(_) => "Read reference",
            Self::Listing(_) => "View article details",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Review(_) => "Review excerpt",
            Self::Coverage(_) => "Coverage summary",
            Self::Mention(_) => "Community mention",
            Self::Record(_) => "Reference summary",
            Self::Listing(_) => "Article listing",
        }
    }

    fn body(self) -> &'static str {
        match self {
            Self::Review(text)
            | Self::Coverage(text)
            | Self::Mention(text)
            | Self::Record(text)
            | Self::Listing(text) => text,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Reference {
    name: &'static str,
    category: usize,
    context: &'static str,
    detail: &'static str,
    text: ReferenceText,
}

const REFERENCES: [Reference; 38] = [
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
        detail: "School reporting · 2010 & 2013",
        text: ReferenceText::Coverage(
            "September 23, 2010 — ‘Quick-draw delivers the world to Nome students.’ Tyler Rhodes’s article follows Neal’s visit to Nome Elementary School after presentations at Bering Strait schools. It describes maps drawn from memory, geography questions, and prizes collected on his travels. Younger students explored Alaska, while older students took on the Lower 48.\n\nNovember 21, 2013 — Classroom photo captions name Neal and the Geography Gameshow. This is additional classroom coverage, rather than a separate feature article.",
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
    Reference {
        name: "The Provincetown Independent",
        category: 2,
        context: "CAPE COD · ARTIST PROFILES",
        detail: "Two profiles · 2021 & 2025",
        text: ReferenceText::Coverage(
            "March 31, 2021 — ‘For Illustrator Neal Nichols Jr., Drawing Is Like Breathing,’ by Susannah Elisabeth Fulcher. The profile explores Neal’s illustration work and Geography Gameshow performances in the United States, Europe, Australia, New Zealand, and Japan.\n\nAugust 27, 2025 — ‘A Life That’s Everywhere and All Right Here,’ by Dorothea Samaha. This later profile connects Neal’s mapmaking, travels, and Cape Cod roots with the Geography Gameshow.",
        ),
    },
    Reference {
        name: "Overdrive",
        category: 2,
        context: "MAGAZINE FEATURE · DECEMBER 2004",
        detail: "The World At His Fingertips",
        text: ReferenceText::Coverage(
            "Published December 13, 2004, this feature traces the development of the Geography Game Show. It describes Neal drawing world maps from memory in classrooms, answering students’ questions, and using souvenirs from his travels to make geography engaging. The article also connects his art and teaching with his years on the road.",
        ),
    },
    Reference {
        name: "The Patriot Ledger",
        category: 2,
        context: "ARTICLE LISTING · MAY 2008",
        detail: "Artist brings Geography Gameshow to Weymouth school",
        text: ReferenceText::Listing(
            "‘Artist brings Geography Gameshow to Weymouth school’ — The Patriot Ledger, May 7, 2008. The article’s title and date are recorded in an online index; the full article was not available for a coverage summary.",
        ),
    },
    Reference {
        name: "University of Delaware",
        category: 0,
        context: "DELAWARE · GIS DAY",
        detail: "University & Delaware Geological Survey report · 2009",
        text: ReferenceText::Record(
            "The university’s November 24, 2009 report on a Delaware GIS Day field trip describes a Geography Game Show for fifth graders. The Delaware Geological Survey republished the same account the following day.",
        ),
    },
    Reference {
        name: "Peacehaven Community School",
        category: 0,
        context: "ENGLAND · SCHOOL REPORTS",
        detail: "School news & December 2022 newsletter",
        text: ReferenceText::Record(
            "A school news report describes Neal’s world-map demonstration during a December 4 visit.\n\nThe December 16, 2022 newsletter separately reports a Geography Gameshow visit on December 1 that year. Both publications identify Neal and the program.",
        ),
    },
    Reference {
        name: "Lancing Prep Hove",
        category: 0,
        context: "ENGLAND · RETURN VISITS",
        detail: "Five school publications · 2016–2024",
        text: ReferenceText::Record(
            "The school’s 2016–2017 and 2017–2018 annual reviews describe the Geography Gameshow, including a program for Years 6 and 7.\n\nDecember 2022 — The newsletter reports Neal’s return from Cape Cod for Years 7 and 8 after the pandemic interruption.\n\nDecember 15, 2023 — A newsletter describes Europe and world-map shows for Years 7 and 8.\n\nDecember 20, 2024 — The school records another Geography Game Show visit.",
        ),
    },
    Reference {
        name: "James Hargest College",
        category: 0,
        context: "NEW ZEALAND · STUDENT ACCOUNT",
        detail: "Year 10 Social Studies · May 2024",
        text: ReferenceText::Record(
            "The college’s May 10, 2024 newsletter includes a Year 10 Social Studies student account naming Neal Nichols and The Geography Gameshow. The account documents a visit to the New Zealand school.",
        ),
    },
    Reference {
        name: "Orleans Elementary School",
        category: 0,
        context: "MASSACHUSETTS · TOWN REPORT",
        detail: "School community partnerships · Grades 3–5",
        text: ReferenceText::Record(
            "The Town of Orleans annual report describes a PTC-sponsored Geography Game Show with Neal Nichols for Grades 3–5 as part of the elementary school’s community partnerships.",
        ),
    },
    Reference {
        name: "Waynesboro Area School District",
        category: 0,
        context: "PENNSYLVANIA · SCHOOL BOARD RECORD",
        detail: "Seventh & eighth grades · November 2009",
        text: ReferenceText::Record(
            "School board minutes dated November 17, 2009 name Neal Nichols and the Geography Game Show and describe geography lessons for seventh and eighth graders.",
        ),
    },
    Reference {
        name: "Middleton PTO",
        category: 0,
        context: "MASSACHUSETTS · EDUCATIONAL PROGRAMS",
        detail: "PTO program directory",
        text: ReferenceText::Record(
            "Middleton PTO includes the Geography Gameshow on its Educational Programs page and identifies Neal Nichols Jr. as the program’s creator and host.",
        ),
    },
    Reference {
        name: "Lynnfield Educational Trust",
        category: 0,
        context: "MASSACHUSETTS · EDUCATIONAL GRANT",
        detail: "Lynnfield Middle School · Spring 2024",
        text: ReferenceText::Record(
            "The trust’s Spring 2024 grantmaking record includes the LMS Geography Game Show. Its description names Neal Nichols Jr. and the planned fifth-grade learning program.",
        ),
    },
    Reference {
        name: "Mrs. Thomas’s Fourth Grade",
        category: 0,
        context: "CLASSROOM BLOG · NOVEMBER 2013",
        detail: "Neil ‘The Geography Guy’",
        text: ReferenceText::Record(
            "A November 7, 2013 classroom account describes an artist named Neil Nichols teaching U.S. geography through drawing. The post uses ‘Neil,’ a spelling variant in the school account.",
        ),
    },
    Reference {
        name: "Mrs. Begin’s Happy Campers",
        category: 0,
        context: "CLASSROOM BLOG · JANUARY 2016",
        detail: "A Very Busy December!",
        text: ReferenceText::Record(
            "The January 5, 2016 classroom post ‘A Very Busy December!’ names Neal Nichols Jr. and his Geography Game Show among the class’s December activities.",
        ),
    },
    Reference {
        name: "Mike Mahaffie",
        category: 1,
        context: "FIRST-PERSON ACCOUNT · NOVEMBER 2009",
        detail: "Mike’s Musings · Such a Busy Week",
        text: ReferenceText::Record(
            "Delaware GIS coordinator Mike Mahaffie describes Neal’s GIS Day show in his November 20, 2009 blog post ‘Such a Busy Week.’ This is a personal account of the event also covered by the University of Delaware.",
        ),
    },
    Reference {
        name: "Brewster Council on Aging",
        category: 1,
        context: "CAPE COD · FAMILY EVENT",
        detail: "Bayside Chatter · Summer 2023",
        text: ReferenceText::Record(
            "The July–August 2023 municipal newsletter advertised a Family Fun Night featuring the Geography Gameshow on August 11. The notice identifies Neal as the program’s creator and host.",
        ),
    },
    Reference {
        name: "Cotuit Library",
        category: 1,
        context: "CAPE COD · LIBRARY PROGRAM",
        detail: "Geography Game Show · April 22, 2026",
        text: ReferenceText::Record(
            "The library’s April 22, 2026 event listing names Neal Nichols Jr. and describes his international school performances. The listing announces a Geography Game Show library program.",
        ),
    },
    Reference {
        name: "Osterville Village Library",
        category: 1,
        context: "CAPE COD · LIBRARY PROGRAM",
        detail: "Geography Gameshow · July 22, 2026",
        text: ReferenceText::Record(
            "A Cape Cod Chamber event notice lists the Geography Gameshow at Osterville Village Library on July 22, 2026 and names Neal Nichols Jr. Matching library and community calendar notices describe the same event.",
        ),
    },
    Reference {
        name: "Wellfleet Cultural District",
        category: 1,
        context: "CAPE COD · ART PROGRAMS",
        detail: "Drawing class & family demonstration · 2023 & 2026",
        text: ReferenceText::Record(
            "July 17, 2023 — A week-long drawing-class notice identifies Neal as the founder of Geography Game Show.\n\nSeptember 5, 2026 — A family art demonstration listing identifies him as the creator of the Geography Map Show. These notices document Neal’s art programs.",
        ),
    },
    Reference {
        name: "Cape Cod Village",
        category: 1,
        context: "COMMUNITY FUNDRAISER · 2019",
        detail: "Drawn Together · Auction catalog",
        text: ReferenceText::Record(
            "Cape Cod Village’s 2019 live-auction catalog includes ‘Drawn Together’ and identifies Neal Nichols as the creator of Geography Gameshow.",
        ),
    },
    Reference {
        name: "Cape Cod National Seashore",
        category: 1,
        context: "NATIONAL PARK SERVICE · PUBLIC PROGRAMS",
        detail: "Five published event records · 2006–2024",
        text: ReferenceText::Record(
            "The park’s 2006–2007 visitor newspaper lists an August 22 Geography Gameshow, and its Summer 2015 issue announces an August 25 show.\n\nA July 2019 National Park Service release includes Neal’s Geography Show in the August 13 evening program.\n\nThe June 2023 summer-program release lists a July 5 Geography Gameshow, also promoted by Friends of Cape Cod National Seashore.\n\nThe park’s 2024 Evenings at Salt Pond schedule names Neal and the Geography Gameshow for July 23. These are public-program announcements.",
        ),
    },
    Reference {
        name: "K12 Academics",
        category: 1,
        context: "EDUCATION DIRECTORY",
        detail: "Geography Gameshow performer listing",
        text: ReferenceText::Record(
            "K12 Academics lists The Geography Gameshow in its performer directory and identifies Neal Nichols Jr. as director and creator. This is a program listing.",
        ),
    },
];

const PAGE_SIZE: usize = 6;

const GROUPS: [(&str, &str, &str); 4] = [
    (
        "All references",
        "A world of shared experiences.",
        "Reviews, school reports, community programs, and stories from Neal’s travels.",
    ),
    (
        "Schools & education",
        "From the classroom.",
        "School reviews, classroom accounts, and educational reports from around the world.",
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

fn in_group(reference: &Reference, group: usize) -> bool {
    group == 0 || reference.category + 1 == group
}

fn turn_page(page: RwSignal<usize>, next: usize) {
    page.set(next);
    if let Some(results) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id("reference-results"))
    {
        results.scroll_into_view();
    }
}

#[component]
pub fn References() -> impl IntoView {
    let category = RwSignal::new(0_usize);
    let query = RwSignal::new(String::new());
    let page = RwSignal::new(0_usize);
    let filtered = Memo::new(move |_| {
        let query = query.get().trim().to_lowercase();
        REFERENCES
            .into_iter()
            .filter(|r| {
                in_group(r, category.get())
                    && (query.is_empty()
                        || format!("{} {} {} {}", r.name, r.context, r.detail, r.text.body())
                            .to_lowercase()
                            .contains(&query))
            })
            .collect::<Vec<_>>()
    });
    let page_count = move || filtered.with(|items| items.len().div_ceil(PAGE_SIZE).max(1));
    view! {
        <section class="references-page wrap page-section" aria-labelledby="references-title">
            <div class="references-heading"><div><p class="eyebrow">"CLASSROOMS. COMMUNITIES. CONNECTIONS."</p><h1 id="references-title">"References "<span class="serif-line">"& press."</span></h1></div><p>"In their own words. Read reviews from schools and communities, and explore stories about Neal’s work around the world."</p></div>
            <div class="reference-search">
                <label for="reference-search">"Find a reference"</label>
                <div><input id="reference-search" type="search" placeholder="School, publication, or place…" prop:value=move || query.get() on:input=move |ev| { query.set(event_target_value(&ev)); page.set(0); }/><button type="button" disabled=move || query.get().is_empty() on:click=move |_| { query.set(String::new()); page.set(0); }>"Clear"</button></div>
            </div>
            <div class="reference-filters" role="group" aria-label="Browse references">
                {GROUPS.into_iter().enumerate().map(|(i, (label, _, _))| view! {
                    <button type="button" aria-controls="reference-results" aria-pressed=move || (category.get() == i).to_string() class:active=move || category.get() == i on:click=move |_| { category.set(i); page.set(0); }>{label}<span class="reference-count">{REFERENCES.iter().filter(|r| in_group(r, i)).count()}</span></button>
                }).collect_view()}
            </div>
            <div class="reference-directory" id="reference-results">
                <div class="reference-section-heading"><div><h2>{move || GROUPS[category.get()].1}</h2><p>{move || GROUPS[category.get()].2}</p></div><span class="directory-caption" role="status">{move || {
                    let count = filtered.with(Vec::len);
                    if count == 0 { "No matching references".to_string() } else {
                        format!("Showing {}–{} of {}", page.get() * PAGE_SIZE + 1, ((page.get() + 1) * PAGE_SIZE).min(count), count)
                    }
                }}</span></div>
                <ul class="reference-list" aria-label=move || GROUPS[category.get()].0>
                    <For
                        each={move || filtered.get().into_iter().skip(page.get() * PAGE_SIZE).take(PAGE_SIZE).collect::<Vec<_>>()}
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
                <Show when=move || filtered.with(Vec::is_empty)>
                    <div class="reference-empty"><h3>"No references found."</h3><p>"Try another name or place, or browse the full collection."</p><button type="button" on:click=move |_| { query.set(String::new()); category.set(0); page.set(0); }>"Show all references"</button></div>
                </Show>
                <Show when={move || page_count() > 1}>
                    <nav class="reference-pagination" aria-label="Reference pages">
                        <button type="button" disabled=move || page.get() == 0 on:click=move |_| turn_page(page, page.get().saturating_sub(1))><span aria-hidden="true">"←"</span>"Previous"</button>
                        <span>{move || format!("Page {} of {}", page.get() + 1, page_count())}</span>
                        <button type="button" disabled={move || page.get() + 1 >= page_count()} on:click=move |_| turn_page(page, (page.get() + 1).min(page_count() - 1))>"Next"<span aria-hidden="true">"→"</span></button>
                    </nav>
                </Show>
            </div>
            <div class="references-invitation"><div><p class="eyebrow">"YOUR SCHOOL COULD BE NEXT"</p><h2>"Let’s bring the world to you."</h2></div><a class="button" href="booking.html">"Inquire about a visit"<span>"↗"</span></a></div>
        </section>
    }
}
