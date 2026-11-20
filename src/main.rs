use leptos::prelude::*;

mod coast;
mod gallery;
mod inquiry;
mod interactions;
mod pages;
mod planning;
mod programs;
mod quiz;
mod references;
mod screen;
mod videos;

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Programs,
    Geography,
    Art,
    Photos(usize),
    Videos(usize),
    InAction,
    MeetNeal,
    Challenge,
    Planning,
    References,
    Booking,
}

impl Page {
    fn current() -> Self {
        let path = web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        match path.rsplit('/').next().unwrap_or_default() {
            "programs.html" => Self::Programs,
            "geography.html" => Self::Geography,
            "art.html" => Self::Art,
            "geography-photos.html" => Self::Photos(0),
            "art-photos.html" => Self::Photos(1),
            "geography-videos.html" => Self::Videos(0),
            "art-videos.html" => Self::Videos(1),
            "in-action.html" => Self::InAction,
            "meet-neal.html" => Self::MeetNeal,
            "challenge.html" => Self::Challenge,
            "planning.html" => Self::Planning,
            "references.html" => Self::References,
            "booking.html" => Self::Booking,
            _ => Self::Home,
        }
    }
    fn is_program(self) -> bool {
        matches!(
            self,
            Self::Programs | Self::Geography | Self::Art | Self::Photos(_) | Self::Videos(_)
        )
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let page = Page::current();
    let menu_open = RwSignal::new(false);
    let storage = || web_sys::window().and_then(|w| w.session_storage().ok().flatten());
    let paused = storage()
        .and_then(|s| s.get_item("ocean-paused").ok().flatten())
        .is_some_and(|v| v == "true");
    let motion_paused = RwSignal::new(paused);
    let toggle_motion = move |_| {
        motion_paused.update(|v| *v = !*v);
        if let Some(s) = storage() {
            let _ = s.set_item("ocean-paused", &motion_paused.get().to_string());
        }
    };
    view! {
        <div class="site-shell" class:motion-paused=move || motion_paused.get() class:coastal-shell=page == Page::Home on:pointermove=interactions::follow_pointer on:pointerdown=interactions::follow_pointer>
            <div class="ocean-backdrop" aria-hidden="true"><div class="ocean-texture"></div><div class="ocean-reflection"></div><div class="ocean-light"></div></div>
            <a class="skip-link" href="#main">"Skip to content"</a>
            <header class="site-header wrap">
                <a class="brand heritage-brand" href="index.html" aria-label="The Geography Gameshow home"><img src="public/geography-gameshow-logo-header.webp" alt="The Geography Gameshow" width="400" height="267"/></a>
                <a class="button mobile-book" href="booking.html">"Request a Show"</a>
                <button class="menu-toggle" aria-controls="main-nav" aria-expanded=move || menu_open.get().to_string() on:click=move |_| menu_open.update(|v| *v = !*v)>{move || if menu_open.get() { "Close −" } else { "Menu +" }}</button>
                <nav id="main-nav" class:open=move || menu_open.get() aria-label="Main navigation">
                    <a href="index.html" aria-current=(page == Page::Home).then_some("page")>"Home"</a>
                    <details class="explore-menu program-menu" name="site-navigation"><summary class:active=page.is_program()>"Programs"<span aria-hidden="true">"⌄"</span></summary><div class="explore-links">
                        <a href="geography.html" aria-current=(page == Page::Geography).then_some("page")>"Geography"</a>
                        <a href="art.html" aria-current=(page == Page::Art).then_some("page")>"Art"</a>
                    </div></details>
                    <a href="meet-neal.html" aria-current=(page == Page::MeetNeal).then_some("page")>"Meet Neal"</a>
                    <a href="references.html" aria-current=(page == Page::References).then_some("page")>"References"</a>
                    <details class="explore-menu" name="site-navigation"><summary class:active=matches!(page, Page::InAction | Page::Challenge | Page::Planning)>"Explore"<span aria-hidden="true">"⌄"</span></summary><div class="explore-links">
                        <a href="in-action.html" aria-current=(page == Page::InAction).then_some("page")>"The show in action"<span>"↗"</span></a>
                        <a href="challenge.html" aria-current=(page == Page::Challenge).then_some("page")>"Try a challenge"<span>"↗"</span></a>
                        <a href="planning.html" aria-current=(page == Page::Planning).then_some("page")>"Plan a visit"<span>"↗"</span></a>
                    </div></details>
                    <a class="button button-small" href="booking.html" aria-current=(page == Page::Booking).then_some("page")>"Request a Show"</a>
                </nav>
            </header>
            <main id="main" tabindex="-1" class=if page == Page::Home { "home-page" } else { "interior-page" }>
                {match page {
                    Page::Home => view! { <pages::Home/> }.into_any(),
                    Page::Programs => view! { <programs::Programs/> }.into_any(),
                    Page::Geography => view! { <programs::ProgramPage index=0/> }.into_any(),
                    Page::Art => view! { <programs::ProgramPage index=1/> }.into_any(),
                    Page::Photos(index) => view! { <programs::PhotoPage index=index/> }.into_any(),
                    Page::Videos(index) => view! { <videos::VideoPage index=index/> }.into_any(),
                    Page::InAction => view! { <pages::InAction/> }.into_any(),
                    Page::MeetNeal => view! { <pages::MeetNeal/> }.into_any(),
                    Page::Challenge => view! { <quiz::Quiz/> }.into_any(),
                    Page::Planning => view! { <planning::Planning/> }.into_any(),
                    Page::References => view! { <references::References/> }.into_any(),
                    Page::Booking => view! { <inquiry::Inquiry/> }.into_any(),
                }}
            </main>
            <footer class="wrap compact-footer">
                <p class="site-copyright"><span>"(c) 1992 Neal Nichols Jr."</span>" "<span>"geographygameshow.com"</span></p>
                <div class="footer-main"><p>"A whole world of curiosity."</p><nav aria-label="Footer navigation"><a href="programs.html">"Programs"</a><a href="references.html">"References"</a><a href="planning.html">"Plan a visit"</a><a href="booking.html">"Request a Show"</a></nav></div>
                <div class="footer-motion"><button class="motion-toggle" aria-pressed=move || motion_paused.get().to_string() aria-label=move || if motion_paused.get() { "Resume animations" } else { "Pause animations" } on:click=toggle_motion>{move || if motion_paused.get() { "Resume motion" } else { "Pause motion" }}</button></div>
            </footer>
        </div>
    }
}
