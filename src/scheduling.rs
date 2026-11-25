use crate::schedule_content::{Curriculum, VisitSchedules};
use leptos::prelude::*;

const CONTACT_EMAIL: &str = "geographygameshow@yahoo.com";

#[component]
pub fn Scheduling() -> impl IntoView {
    let panels = vec![
        ("Overview", view! { <Overview/> }.into_any()),
        ("Programs by Grade", view! { <Curriculum/> }.into_any()),
        (
            "Additional Programs",
            view! { <AdditionalPrograms/> }.into_any(),
        ),
        (
            "Visit Schedules",
            view! { <VisitSchedules/><BookingInvitation/> }.into_any(),
        ),
    ];
    let panels = panels
        .into_iter()
        .map(|(label, content)| {
            (label, view! {
            <div class="guide-pane" role="region" aria-label=label tabindex="0">{content}</div>
        }.into_any())
        })
        .collect();
    view! {
        <article class="screen-page wrap scheduling-guide">
            <h1>"Programs & Scheduling"</h1>
            <crate::screen::Deck panels=panels/>
        </article>
    }
}

#[component]
fn Overview() -> impl IntoView {
    view! {
        <section class="guide-intro" aria-labelledby="intro-heading">
            <div class="guide-intro-copy">
                <h2 id="intro-heading">"A World of Learning, "<em>"Illustrated from Memory"</em></h2>
                <p>"Neal combines illustrations drawn from memory with educational storytelling and audience participation to bring geography, history, and art to life. The Geography Gameshow turns a practical lesson into an interactive game show, with opportunities for students to win prizes from places around the world."</p>
                <p>"Programs are available for schools, libraries, camps, aftercare groups, tourist centers, homeschool assemblies, Montessori schools, and family fun night fundraisers."</p>
            </div>
            <img src="public/photos/world-in-progress.webp" alt="Neal drawing Europe on a large hand-drawn world map" width="1290" height="708"/>
        </section>
        <BookingInvitation/>
    }
}

#[component]
fn AdditionalPrograms() -> impl IntoView {
    view! {
        <section class="guide-section guide-additional" aria-labelledby="additional-heading">
            <div>
                <h2 id="additional-heading">"More Places. More Stories. More Ways to Learn."</h2>
                <ul class="additional-titles">
                    {crate::programs::ADDITIONAL_TITLES.into_iter().map(|title| view! { <li>{title}</li> }).collect_view()}
                </ul>
                <p>"Ask about additional titles and options for your audience."</p>
            </div>
            <figure>
                <img src="public/photos/mexico.webp" alt="Illustrated map of Mexico, Central America, and the Caribbean" width="837" height="583" loading="lazy"/>
                <figcaption>"Mexico and its neighbors, drawn by Neal."</figcaption>
            </figure>
        </section>
    }
}

#[component]
fn BookingInvitation() -> impl IntoView {
    view! {
        <section class="guide-invitation">
            <div>
                <h2>"Bring the World to Your Audience"</h2>
                <p>"Planning a school assembly, library program, homeschool event, or family fundraiser? Get in touch to discuss program choices and scheduling."</p>
            </div>
            <div class="guide-contact">
                <a class="button" href=format!("mailto:{CONTACT_EMAIL}")>"Ask About a Visit"</a>
                <span>{CONTACT_EMAIL}</span>
            </div>
        </section>
    }
}
