use leptos::prelude::*;

mod quiz;
use quiz::Quiz;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let menu_open = RwSignal::new(false);
    let selected = RwSignal::new(0_usize);
    let motion_paused = RwSignal::new(false);
    view! {
        <div class="site-shell" class:motion-paused=move || motion_paused.get()>
        <div class="ocean-backdrop" aria-hidden="true"><div class="ocean-texture"></div><div class="ocean-reflection"></div><div class="ocean-light"></div></div>
        <a class="skip-link" href="#main">"Skip to content"</a>
        <div class="preview-bar"><span>"A NEW CHAPTER IN CURIOSITY"</span><span>"A concept preview for Neal"<button class="motion-toggle" aria-pressed=move || motion_paused.get().to_string() aria-label=move || if motion_paused.get() { "Resume ocean animation" } else { "Pause ocean animation" } on:click=move |_| motion_paused.update(|v| *v = !*v)>{move || if motion_paused.get() { "▶ Resume ocean" } else { "Ⅱ Pause ocean" }}</button></span></div>
        <header class="site-header wrap">
            <a class="brand heritage-brand" href="#" aria-label="The Geography Gameshow home"><img src="public/geography-gameshow-logo.webp" alt="The Geography Gameshow" width="1536" height="1024"/></a>
            <button class="menu-toggle" aria-controls="main-nav" aria-expanded=move || menu_open.get().to_string() on:click=move |_| menu_open.update(|v| *v = !*v)>{move || if menu_open.get() { "Close −" } else { "Menu +" }}</button>
            <nav id="main-nav" class:open=move || menu_open.get() aria-label="Main navigation">
                <a href="#shows" on:click=move |_| menu_open.set(false)>"The shows"</a>
                <a href="#meet-neal" on:click=move |_| menu_open.set(false)>"Meet Neal"</a>
                <a href="#challenge" on:click=move |_| menu_open.set(false)>"Try a challenge"<span class="tiny-star">"✳"</span></a>
                <a class="button button-small" href="#booking" on:click=move |_| menu_open.set(false)>"Bring a show to you"<span>"↗"</span></a>
            </nav>
        </header>
        <main id="main">
            <section class="hero wrap" aria-labelledby="hero-title">
                <div class="hero-copy">
                    <p class="eyebrow"><span class="live-dot"></span>"LIVE SHOWS. LIMITLESS CURIOSITY."</p>
                    <h1 id="hero-title">"Big ideas."<br/><span class="underlined">"Drawn live."<svg viewBox="0 0 490 24" preserveAspectRatio="none" aria-hidden="true"><path d="M5 17Q190-3 484 8M42 23Q280 5 457 17"/></svg></span></h1>
                    <p class="hero-description">"A map becomes an adventure. A question becomes a discovery. With Neal Nichols Jr., learning comes to life—one drawing at a time."</p>
                    <div class="hero-actions"><a class="button" href="#shows">"Find your next adventure"<span>"↗"</span></a><a class="text-link" href="#in-action"><span class="play-icon">"▶"</span>"See the magic"</a></div>
                    <div class="hero-footnote"><span class="hand-star">"✳"</span><span>"For curious minds."<br/><b>"And the people who inspire them."</b></span></div>
                </div>
                <div class="hero-art brand-hero">
                    <p class="logo-motto">"Every child deserves the opportunity to experience"<br/><strong>"the Geography Game Show"</strong></p>
                    <img class="hero-brand-logo" src="public/geography-gameshow-logo.webp" alt="The Geography Gameshow: gold lettering across a globe with cobalt-blue oceans and earthy green continents" fetchpriority="high" width="1536" height="1024"/>
                </div>
            </section>
            <div class="ticker" aria-label="Geography, biology, and art: a world of discovery"><div><span>"GEOGRAPHY"</span><i>"✳"</i><span>"BIOLOGY"</span><i>"✳"</i><span>"ART"</span><i>"✳"</i><span class="ticker-outline">"A WORLD OF DISCOVERY"</span><i>"✳"</i></div></div>
            <section class="shows wrap section-space" id="shows" aria-labelledby="shows-title">
                <div class="section-heading"><div><p class="eyebrow">"01 / CHOOSE YOUR ADVENTURE"</p><h2 id="shows-title">"Three worlds."<br/>"One curious mind."</h2></div><p>"The Geography Gameshow is growing. Explore the original experience—and a first look at where curiosity could take us next."</p></div>
                <div class="show-cards">
                    <a class="show-card geography-card" href="#show-details" on:click=move |_| selected.set(0)>
                        <div class="card-top"><span>"01 / THE ORIGINAL"</span><span class="round-arrow">"↗"</span></div>
                        <div class="card-art geography-art"><img src="public/globe.svg" alt="" loading="lazy"/><span class="card-orbit"></span></div>
                        <div class="card-copy"><h3>"Geography"</h3><p>"The whole world. A blank page. A map drawn entirely from memory."</p><span class="card-link">"Explore the show"<span>"→"</span></span></div>
                    </a>
                    <a class="show-card biology-card" href="#show-details" on:click=move |_| selected.set(1)>
                        <div class="card-top"><span>"02 / A NEW DIRECTION"</span><span class="round-arrow">"↗"</span></div>
                        <div class="card-art biology-art"><img src="public/botanical.svg" alt="" loading="lazy"/><span class="specimen-label">"FIG. 02 — LIFE, UP CLOSE"</span></div>
                        <div class="card-copy"><h3>"Biology"<span class="new-label">"IN DEVELOPMENT"</span></h3><p>"Small wonders. Big connections. A new way to explore the living world."</p><span class="card-link">"Discover the concept"<span>"→"</span></span></div>
                    </a>
                    <a class="show-card art-card" href="#show-details" on:click=move |_| selected.set(2)>
                        <div class="card-top"><span>"03 / MAKE SOMETHING"</span><span class="round-arrow">"↗"</span></div>
                        <div class="card-art drawing-art"><img src="public/art-tools.svg" alt="" loading="lazy"/></div>
                        <div class="card-copy"><h3>"Art"<span class="new-label">"NEW SHOW CONCEPT"</span></h3><p>"Look a little closer. Think a little differently. See what you can create."</p><span class="card-link">"Imagine the possibilities"<span>"→"</span></span></div>
                    </a>
                </div>
                <ShowDetails selected/>
            </section>
            <section class="in-action section-space" id="in-action" aria-labelledby="action-title">
                <div class="wrap action-grid"><div class="action-photo"><img src="public/neal-live.webp" alt="Neal Nichols Jr. drawing the United States on a classroom whiteboard as students watch" width="700" height="497" loading="lazy"/><div class="photo-label"><span class="live-dot"></span>"REAL MAPS. REAL MOMENTS. REAL CURIOSITY."</div></div>
                    <div class="action-copy"><p class="eyebrow">"MORE THAN A SCHOOL ASSEMBLY"</p><h2 id="action-title">"You don’t just"<br/>"watch the show."<br/><em>"You’re part of it."</em></h2><p>"It starts with a blank page. As Neal draws, students follow clues, connect ideas, and put their knowledge to the test. Geography becomes something they experience together."</p><a class="button button-light" href="https://www.youtube.com/watch?v=YnSI6XrQm8E" target="_blank" rel="noopener noreferrer"><span class="play-icon">"▶"</span>"Watch the original show"<span>"↗"</span></a><span class="video-caption">"From the Geography Gameshow archive · Opens YouTube"</span></div>
                </div>
            </section>
            <Quiz/>
            <section class="meet wrap section-space" id="meet-neal" aria-labelledby="meet-title">
                <div class="meet-art"><div class="artwork-frame"><img src="public/neal-art.webp" alt="Neal's detailed black-and-white drawing of Uncle Tim's Bridge in Wellfleet" loading="lazy" width="480" height="382"/><span>"Uncle Tim’s Bridge, Wellfleet · Artwork by Neal"</span></div><img class="neal-portrait" src="public/neal-portrait.webp" alt="Neal outdoors in the snow with a wolf" width="316" height="436" loading="lazy"/><span class="handwritten meet-note">"Artist. Explorer. Educator."</span></div>
                <div class="meet-copy"><p class="eyebrow">"02 / THE MIND BEHIND THE MAPS"</p><h2 id="meet-title">"Meet Neal."<br/><span class="serif-line">"Forever curious."</span></h2><p>"Raised in Wellfleet on Cape Cod, Neal Nichols Jr. grew up drawing the world around him. His love of art and travel became something bigger: a way to help students see the world for themselves."</p><p>"An artist, traveler, and the creator of The Geography Gameshow, Neal combines maps, stories, and audience participation to turn a lesson into a shared adventure."</p><p class="meet-next">"Now, the next chapter is taking shape—with new possibilities in biology and art."</p><a class="text-link" href="#booking">"Start a conversation with Neal"<span>"↗"</span></a></div>
            </section>
            <section class="planning section-space" id="for-educators" aria-labelledby="planning-title"><div class="wrap planning-grid"><div><p class="eyebrow">"03 / BIG CURIOSITY. SIMPLE PLANNING."</p><h2 id="planning-title">"A good fit for"<br/>"your curious crew."</h2><p>"Every audience is different. Start with your students, your subject, and the kind of experience you have in mind."</p><a class="text-link" href="#booking">"Let’s plan something memorable"<span>"↗"</span></a></div><div class="faq">
                <details open><summary>"Who are the shows for?"<span>"+"</span></summary><p>"Geography programs can be adapted from kindergarten through high school. Neal also welcomes inquiries from libraries, camps, and other groups. Biology and art show formats are still being developed."</p></details>
                <details><summary>"How long is a geography show?"<span>"+"</span></summary><p>"Existing programs range from 30–45 minutes for kindergarten to 90–120 minutes for older students. Neal can help match the topic and length to your group."</p></details>
                <details><summary>"What does our space need?"<span>"+"</span></summary><p>"A place for students to gather, wall space for large paper, rolled bulletin paper, scissors, masking tape, and a table for props. For larger audiences, discuss a microphone with Neal. Confirm the setup when booking."</p></details>
                <details><summary>"How many students can participate?"<span>"+"</span></summary><p>"The existing geography program suggests 30–90 students per assembly for more opportunities to participate. Larger groups can be discussed directly with Neal."</p></details>
                <details><summary>"What does a show cost?"<span>"+"</span></summary><p>"Ask Neal for current pricing and availability. Share your location, group size, preferred dates, and whether you are interested in multiple shows or school visits."</p></details>
            </div></div></section>
            <Booking/>
        </main>
        <footer class="wrap"><div class="footer-main"><a class="brand heritage-brand" href="#" aria-label="The Geography Gameshow home"><img src="public/geography-gameshow-logo.webp" alt="The Geography Gameshow" width="1536" height="1024" loading="lazy"/></a><p>"A whole world of curiosity."</p><a class="text-link" href="#main">"Back to the top"<span>"↑"</span></a></div><div class="footer-bottom"><span>"© 2026 The Geography Gameshow · Neal Nichols Jr."</span><span>"Design preview · Biology & art concepts subject to confirmation"</span></div></footer>
        </div>
    }
}

const PROGRAMS: [(&str, &str, &str, &str, [&str; 3]); 3] = [
    (
        "THE ORIGINAL LIVE EXPERIENCE",
        "A world unfolds. Right before your eyes.",
        "Neal draws maps entirely from memory, weaving geography, history, and listening skills into a show that invites students to join in.",
        "Talk about a geography show",
        [
            "World & regional maps",
            "Programs for K–12",
            "Adaptable to your lessons",
        ],
    ),
    (
        "BIOLOGY / CONCEPT PREVIEW",
        "Get to know the wild world around us.",
        "Imagine a live drawing journey into habitats, living things, and the connections between them. This is a proposed new direction; topics, ages, and show formats will be developed with Neal.",
        "Ask about the biology concept",
        [
            "Living things & habitats",
            "Observation & discovery",
            "Format in development",
        ],
    ),
    (
        "ART / CONCEPT PREVIEW",
        "Every great idea starts with a mark.",
        "Building on Neal’s background in illustration and drawing classes, this proposed show explores how artists see. Think line, perspective, texture, and the confidence to create. The new show format is still taking shape.",
        "Ask about the art concept",
        [
            "Drawing & visual thinking",
            "Perspective & texture",
            "New show format in development",
        ],
    ),
];

#[component]
fn ShowDetails(selected: RwSignal<usize>) -> impl IntoView {
    view! {
        <div class="show-details" id="show-details">
            <div class="show-tabs" role="tablist" aria-label="Explore show details">
                { ["Geography", "Biology", "Art"].into_iter().enumerate().map(|(i, label)| view! {
                    <button id=format!("show-tab-{i}") role="tab" aria-controls="show-panel" aria-selected=move || (selected.get() == i).to_string() class:active=move || selected.get() == i
                        on:click=move |_| selected.set(i)
                        on:keydown=move |ev| { let next = match ev.key().as_str() { "ArrowRight" => Some((i + 1) % 3), "ArrowLeft" => Some((i + 2) % 3), "Home" => Some(0), "End" => Some(2), _ => None }; if let Some(next) = next { ev.prevent_default(); selected.set(next); if let Some(el) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.get_element_by_id(&format!("show-tab-{next}"))) { let _ = js_sys::Reflect::get(&el, &"focus".into()).ok().and_then(|f| { use leptos::wasm_bindgen::JsCast; f.dyn_into::<js_sys::Function>().ok() }).map(|f| f.call0(&el)); } } }>{label}</button>
                }).collect_view() }
                <span>"FOLLOW YOUR CURIOSITY ↗"</span>
            </div>
            <div class="show-panel" id="show-panel" role="tabpanel" tabindex="0" aria-labelledby=move || format!("show-tab-{}", selected.get())>
                <div><p class="eyebrow">{move || PROGRAMS[selected.get()].0}</p><h3>{move || PROGRAMS[selected.get()].1}</h3></div>
                <div><p>{move || PROGRAMS[selected.get()].2}</p><ul>{move || PROGRAMS[selected.get()].4.into_iter().map(|item| view! { <li><span>"↗"</span>{item}</li> }).collect_view()}</ul><a class="text-link" href="#booking">{move || PROGRAMS[selected.get()].3}<span>"→"</span></a></div>
            </div>
        </div>
    }
}

#[component]
fn Booking() -> impl IntoView {
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let school = RwSignal::new(String::new());
    let topic = RwSignal::new("Geography".to_string());
    let notes = RwSignal::new(String::new());
    let draft = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        draft.set(Some(format!("Hello Neal,\n\nI’m interested in {} for our group.\n\nName: {}\nEmail: {}\nSchool / organization: {}\n\nDetails:\n{}\n\nI’d love to hear about current options and availability.\nThank you!", topic.get(), name.get().trim(), email.get().trim(), school.get().trim(), notes.get().trim())));
    };
    view! {
        <section class="booking section-space" id="booking" aria-labelledby="booking-title"><div class="wrap booking-grid"><div class="booking-copy"><p class="eyebrow">"THE NEXT ADVENTURE STARTS HERE"</p><h2 id="booking-title">"Let’s make"<br/>"some room"<br/><span class="serif-line">"for wonder."</span></h2><p>"Tell Neal a little about your group. Together, you can find the right starting point for something memorable."</p><a class="email-link" href="mailto:geographygameshow@yahoo.com">"geographygameshow@yahoo.com"<span>"↗"</span></a><span class="booking-doodle" aria-hidden="true">"✳"</span></div>
            <form class="inquiry-form" on:submit=submit on:input=move |_| draft.set(None)>
                <h3>"Bring curiosity to your classroom."</h3><p>"Start a conversation. Find your show."</p>
                <div class="form-row"><label>"Your name"<input required maxlength="120" autocomplete="name" name="name" placeholder="Alex Morgan" on:input=move |ev| name.set(event_target_value(&ev))/></label><label>"Email address"<input required maxlength="180" type="email" autocomplete="email" name="email" placeholder="you@school.org" on:input=move |ev| email.set(event_target_value(&ev))/></label></div>
                <label>"School or organization"<input required maxlength="180" autocomplete="organization" name="organization" placeholder="Where should curiosity take us?" on:input=move |ev| school.set(event_target_value(&ev))/></label>
                <label>"What are you curious about?"<select name="topic" on:change=move |ev| { topic.set(event_target_value(&ev)); draft.set(None); }><option value="Geography">"The Geography Gameshow"</option><option value="the Biology concept">"Biology — concept inquiry"</option><option value="the Art concept">"Art — concept inquiry"</option><option value="a mix of subjects">"A little of everything"</option></select></label>
                <label>"A little about your group"<textarea required maxlength="2500" name="details" rows="3" placeholder="Ages, group size, location, possible dates…" on:input=move |ev| notes.set(event_target_value(&ev))></textarea></label>
                <button class="button" type="submit">"Prepare my inquiry"<span>"↗"</span></button><p class="form-note">"This prepares an email for you to review and send. Nothing is submitted or stored on this website."</p>
                <Show when=move || draft.get().is_some()><div class="draft-preview" role="status"><h4>"Your inquiry is ready to review."</h4><pre>{move || draft.get().unwrap_or_default()}</pre><a class="button" href=move || format!("mailto:geographygameshow@yahoo.com?subject={}&body={}", js_sys::encode_uri_component(&format!("Show inquiry: {}", topic.get())), js_sys::encode_uri_component(&draft.get().unwrap_or_default()))>"Open in my email app"<span>"↗"</span></a><p>"Or copy the draft above and email Neal directly. Your email app sends the message only when you choose Send."</p></div></Show>
            </form>
        </div></section>
    }
}
