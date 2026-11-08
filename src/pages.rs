use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
    <section class="hero wrap" aria-labelledby="hero-title">
                    <div class="hero-copy">
                        <p class="eyebrow"><span class="live-dot"></span>"LIVE SHOWS. LIMITLESS CURIOSITY."</p>
                        <h1 id="hero-title">"Big ideas."<br/><span class="underlined">"Drawn live."<svg viewBox="0 0 490 24" preserveAspectRatio="none" aria-hidden="true"><path d="M5 17Q190-3 484 8M42 23Q280 5 457 17"/></svg></span></h1>
                        <p class="hero-description">"A map becomes an adventure. A question becomes a discovery. With Neal Nichols Jr., learning comes to life—one drawing at a time."</p>
                        <div class="hero-actions"><a class="button" href="programs.html">"Explore the programs"<span>"↗"</span></a><a class="text-link" href="in-action.html"><span class="play-icon">"▶"</span>"See the magic"</a></div>

                    </div>
                    <div class="hero-art brand-hero">
                        <p class="logo-motto">"Every child deserves the opportunity to experience"<br/><strong>"the Geography Game Show"</strong></p>
                        <img class="hero-brand-logo" src="public/geography-gameshow-logo.webp" alt="The Geography Gameshow: gold lettering across a globe with cobalt-blue oceans and earthy green continents" fetchpriority="high" width="1536" height="1024"/>
                    </div>
                </section>
        }
}

#[component]
pub fn InAction() -> impl IntoView {
    view! {
    <section class="in-action section-space" id="in-action" aria-labelledby="action-title">
                    <div class="wrap action-grid"><div class="action-photo"><img src="public/neal-live.webp" alt="Neal Nichols Jr. drawing the United States on a classroom whiteboard as students watch" width="700" height="497" loading="lazy"/><div class="photo-label"><span class="live-dot"></span>"REAL MAPS. REAL MOMENTS. REAL CURIOSITY."</div></div>
                        <div class="action-copy"><p class="eyebrow">"MORE THAN A SCHOOL ASSEMBLY"</p><h1 id="action-title">"You don’t just"<br/>"watch the show."<br/><em>"You’re part of it."</em></h1><p>"It starts with a blank page. As Neal draws, students follow clues, connect ideas, and put their knowledge to the test. Geography becomes something they experience together."</p><a class="button button-light" href="https://www.youtube.com/watch?v=YnSI6XrQm8E" target="_blank" rel="noopener noreferrer"><span class="play-icon">"▶"</span>"Watch the original show"<span>"↗"</span></a><span class="video-caption">"From the Geography Gameshow archive · Opens YouTube"</span></div>
                    </div>
                </section>
        }
}

#[component]
pub fn MeetNeal() -> impl IntoView {
    view! {
    <section class="meet wrap section-space" id="meet-neal" aria-labelledby="meet-title">
                    <div class="meet-art"><div class="artwork-frame"><img src="public/neal-art.webp" alt="Neal's detailed black-and-white drawing of Uncle Tim's Bridge in Wellfleet" loading="lazy" width="480" height="382"/><span>"Uncle Tim’s Bridge, Wellfleet · Artwork by Neal"</span></div><img class="neal-portrait" src="public/neal-portrait.webp" alt="Neal outdoors in the snow with a wolf" width="316" height="436" loading="lazy"/><span class="handwritten meet-note">"Artist. Explorer. Educator."</span></div>
                    <div class="meet-copy"><p class="eyebrow">"THE MIND BEHIND THE MAPS"</p><h1 id="meet-title">"Meet Neal."<br/><span class="serif-line">"Forever curious."</span></h1><p>"Raised in Wellfleet on Cape Cod, Neal Nichols Jr. grew up drawing the world around him. His love of art and travel became something bigger: a way to help students see the world for themselves."</p><p>"An artist, traveler, and the creator of The Geography Gameshow, Neal combines maps, stories, and audience participation to turn a lesson into a shared adventure."</p><p class="meet-next">"Now, the next chapter is taking shape—with new possibilities in anatomy and art."</p><a class="text-link" href="booking.html">"Start a conversation with Neal"<span>"↗"</span></a></div>
                </section>
        }
}
