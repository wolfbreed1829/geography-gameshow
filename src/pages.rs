use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
    <crate::coast::CoastalHome>
    <section class="landing-hero wrap" aria-labelledby="hero-title">
        <img class="landing-logo" src="public/geography-gameshow-logo.webp" alt="The Geography Gameshow" fetchpriority="high" width="1536" height="1024"/>
        <h1 id="hero-title" class="reflective-copy">"World maps illustrated from memory"</h1>
        <p class="reflective-copy">"Educational performances for all ages"</p>
        <div class="landing-actions">
            <a class="button" href="booking.html?program=geography">"Request a Show"</a>
            <a class="text-link" href="programs.html">"View Programs"</a>
        </div>
    </section>
    </crate::coast::CoastalHome>
        }
}

#[component]
pub fn ReferenceHighlights() -> impl IntoView {
    view! {
    <section class="home-evidence reference-highlights" aria-label="Experience the show and hear from schools">
        <div class="home-evidence-grid">
            <crate::videos::FeaturedVideo/>
            <div class="home-school-story">
                <p class="eyebrow">"FROM THE CLASSROOM"</p>
                <blockquote>"“It was a dynamic and fascinating way to learn more about the geography of the world.”"</blockquote>
                <p class="home-quote-credit">"Westbrook Hay Prep School"<span>"School visit report · April 2022"</span></p>
                <a class="text-link" href="#reference-search">"Read the school stories"</a>
            </div>
        </div>
        <div class="home-press"><span>"FEATURED IN"</span><p>"The Provincetown Independent"</p><p>"The Nome Nugget"</p><p>"Overdrive"</p><a href="#reference-search">"Explore the coverage"</a></div>
    </section>
        }
}

#[component]
pub fn InAction() -> impl IntoView {
    let panels=vec![
        ("The show",view!{<h2>"You don’t just watch the show. You’re part of it."</h2>
            <crate::screen::Reader text="It starts with a blank page. As Neal draws, students follow clues, connect ideas, and put their knowledge to the test. Geography becomes something they experience together.".to_string()/>
            <div class="screen-links"><a href="geography-videos.html">"Watch the original show"</a></div>
        }.into_any()),
        ("In the classroom",view!{<figure class="screen-media"><img src="public/photos/world-in-progress.webp" alt="Neal drawing Europe on a large hand-drawn world map" width="1290" height="708"/><figcaption>"Real maps. Real moments. Real curiosity."</figcaption></figure>}.into_any())
    ];
    view! {<section class="screen-page wrap"><h1>"The show in action"</h1><crate::screen::Deck panels=panels/></section>}
}

#[component]
pub fn MeetNeal() -> impl IntoView {
    let panels=vec![
        ("About Neal",view!{<crate::screen::Reader text="Raised in Wellfleet on Cape Cod, Neal Nichols Jr. grew up drawing the world around him. His love of art and travel became something bigger: a way to help students see the world for themselves.\n\nAn artist, traveler, and the creator of The Geography Gameshow, Neal combines maps, stories, and audience participation to turn a lesson into a shared adventure.\n\nNow, the next chapter is taking shape—with new possibilities in art.".to_string()/>}.into_any()),
        ("Artwork",view!{<figure class="screen-media"><img src="public/neal-art.webp" alt="Neal's detailed black-and-white drawing of Uncle Tim's Bridge in Wellfleet" width="480" height="382"/><figcaption>"Uncle Tim’s Bridge, Wellfleet · Artwork by Neal"</figcaption></figure>}.into_any()),
        ("Portrait",view!{<figure class="screen-media"><img src="public/neal-portrait.webp" alt="Neal outdoors in the snow with a wolf" width="316" height="436"/><figcaption>"Artist. Explorer. Educator."</figcaption></figure>}.into_any())
    ];
    view! {<section class="screen-page wrap"><h1>"Meet Neal"</h1><crate::screen::Deck panels=panels/></section>}
}
