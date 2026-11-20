use leptos::prelude::*;

struct Article {
    publisher: &'static str,
    title: &'static str,
    credit: &'static str,
    paragraphs: &'static [&'static str],
}

include!("reference_articles.rs");

fn has_articles(name: &str) -> bool {
    ARTICLES.iter().any(|article| article.publisher == name)
}

fn article_matches(name: &str, query: &str) -> bool {
    ARTICLES
        .iter()
        .filter(|article| article.publisher == name)
        .any(|article| {
            format!(
                "{} {} {}",
                article.title,
                article.credit,
                article.paragraphs.join(" ")
            )
            .to_lowercase()
            .contains(query)
        })
}

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
            "A supplied news clipping, ‘Geography game show comes to Naumann,’ records Neal’s two-day visit to Naumann Elementary. It describes third graders learning the 50 United States and ways to remember them while Neal draws a map from memory. The accompanying caption captures the question-and-answer part of the show, with a student receiving a prize for a correct response. The clipping provides a brief classroom account; its publisher and publication date are not visible.",
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
            "The supplied Cook Islands News clipping carries the headline ‘Visitor turns learning into lots of laughs’ above a classroom map demonstration. The archive preserves the headline and image, but no article body or publication date. It documents press coverage of Neal’s visit to the Cook Islands.",
        ),
    },
    Reference {
        name: "Digo Dristi",
        category: 2,
        context: "NEPAL · WEEKLY NEWSPAPER",
        detail: "Coverage of Neal’s school visits in Nepal",
        text: ReferenceText::Coverage(
            "A clipping from the Nepali weekly Digo Dristi reports on Neal’s educational visits in Nepal, using drawing and geography with students affected by an earthquake. The supplied archive includes the Nepali-language report and a classroom photograph. A complete, verified English translation is not currently available.",
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

fn reading_text(r: Reference) -> String {
    if has_articles(r.name) {
        let mut text = ARTICLES
            .iter()
            .filter(|a| a.publisher == r.name)
            .map(|a| format!("{}\n{}\n\n{}", a.title, a.credit, a.paragraphs.join("\n\n")))
            .collect::<Vec<_>>()
            .join("\n\n");
        text.push_str("\n\nOriginal summaries of the reporting, presented here for easy reading. Publication titles and credits identify the sources.");
        text
    } else {
        format!("{}\n\n{}", r.text.label(), r.text.body())
    }
}

#[component]
pub fn References() -> impl IntoView {
    let active = RwSignal::new(0usize);
    let panels=vec![
        ("Watch",view!{<crate::videos::FeaturedVideo/>}.into_any()),
        ("School story",view!{
            <h2>"Westbrook Hay Prep School"</h2>
            <crate::screen::Reader text="“It was a dynamic and fascinating way to learn more about the geography of the world.”\n\nSchool visit report · April 2022".to_string()/>
            <button type="button" on:click=move |_|active.set(3)>"Read the school stories"</button>
        }.into_any()),
        ("Featured in",view!{
            <crate::screen::Reader text="The Provincetown Independent\n\nThe Nome Nugget\n\nOverdrive".to_string()/>
            <button type="button" on:click=move |_|active.set(3)>"Explore the coverage"</button>
        }.into_any()),
        ("Directory",view!{<ReferenceDirectory/>}.into_any())
    ];
    view! {<section class="screen-page wrap"><h1>"References & press."</h1><crate::screen::Deck panels=panels selected=active/></section>}
}

#[component]
fn ReferenceDirectory() -> impl IntoView {
    let category = RwSignal::new(0usize);
    let query = RwSignal::new(String::new());
    let current = RwSignal::new(0usize);
    let reading = RwSignal::new(false);
    let filtered = Memo::new(move |_| {
        let q = query.get().trim().to_lowercase();
        REFERENCES
            .into_iter()
            .filter(|r| {
                in_group(r, category.get())
                    && (q.is_empty()
                        || article_matches(r.name, &q)
                        || format!("{} {} {} {}", r.name, r.context, r.detail, r.text.body())
                            .to_lowercase()
                            .contains(&q))
            })
            .collect::<Vec<_>>()
    });
    let selected = move || filtered.with(|r| r.get(current.get()).copied());
    view! {<div class="contained-directory">
        <Show when=move ||reading.get() fallback=move ||view!{
            <div class="directory-tools">
                <label><span class="sr-only">"Find a reference"</span><input id="reference-search" type="search" placeholder="School, publication, or place" prop:value=move ||query.get() on:input=move |e|{query.set(event_target_value(&e));current.set(0);}/></label>
                <label><span class="sr-only">"Reference category"</span><select prop:value=move ||category.get().to_string() on:change=move |e|{category.set(event_target_value(&e).parse().unwrap_or(0));current.set(0);}>
                    {GROUPS.into_iter().enumerate().map(|(i,(label,_,_))|view!{<option value=i.to_string()>{label}</option>}).collect_view()}
                </select></label>
            </div>
            <label><span class="sr-only">"Choose a reference"</span><select prop:value=move ||current.get().to_string() on:change=move |e|current.set(event_target_value(&e).parse().unwrap_or(0))>
                {move ||filtered.get().into_iter().enumerate().map(|(i,r)|view!{<option value=i.to_string()>{r.name}</option>}).collect_view()}
            </select></label>
            {move ||selected().map(|r|view!{
                <article class="reference-card"><p>{r.context}</p><h2>{r.name}</h2><p>{r.detail}</p><button type="button" on:click=move |_|reading.set(true)>{r.text.action()}</button></article>
            })}
            <Show when=move ||filtered.with(Vec::is_empty)><div class="reference-empty"><h2>"No references found."</h2><button type="button" on:click=move |_|{query.set(String::new());category.set(0);current.set(0); }>"Show all references"</button></div></Show>
            <nav class="reader-pagination" aria-label="Reference pages">
                <button type="button" disabled=move ||current.get()==0 on:click=move |_|current.update(|i|*i=i.saturating_sub(1))>"Previous"</button>
                <span role="status">{move ||format!("{} / {}",if filtered.with(Vec::is_empty){0}else{current.get()+1},filtered.with(Vec::len))}</span>
                <button type="button" disabled={move ||current.get()+1>=filtered.with(Vec::len)} on:click=move |_|current.update(|i|*i+=1)>"Next"</button>
            </nav>
        }>
            <button class="reader-back" type="button" on:click=move |_|reading.set(false)>"Back to references"</button>
            {move ||selected().map(|r|view!{<h2>{r.name}</h2><crate::screen::Reader text=reading_text(r)/>})}
        </Show>
    </div>}
}
