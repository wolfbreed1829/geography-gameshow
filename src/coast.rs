use leptos::prelude::*;

fn remember_arrival() {
    if let Some(storage) = web_sys::window().and_then(|w| w.session_storage().ok().flatten()) {
        let _ = storage.set_item("wellfleet-arrival-seen", "true");
    }
}

#[component]
pub fn CoastalHome(children: Children) -> impl IntoView {
    let seen = web_sys::window()
        .and_then(|w| w.session_storage().ok().flatten())
        .and_then(|s| s.get_item("wellfleet-arrival-seen").ok().flatten())
        .is_some_and(|value| value == "true");
    let ready = RwSignal::new(false);
    let finished = RwSignal::new(seen);
    let failed = RwSignal::new(false);
    let replay = RwSignal::new(0_u32);
    let finish = move || {
        finished.set(true);
        remember_arrival();
    };

    view! {
        <div class="coastal-home" class:coast-ready=move || ready.get() class:coast-done=move || finished.get() class:coast-failed=move || failed.get()>
            <div class="coastal-backdrop" aria-hidden="true">
                <For each=move || vec![replay.get()] key=|run| *run children=move |_| view! {
                    <img class="coastal-pan" src="public/wellfleet-coast.webp" alt="" width="887" height="1774" fetchpriority="high" decoding="async"
                        on:load=move |_| ready.set(true)
                        on:error=move |_| { failed.set(true); finished.set(true); }
                        on:animationend=move |_| finish()
                    />
                }/>
                <div class="coastal-shade"></div>
            </div>
            <div class="coastal-welcome wrap" aria-hidden="true">
                <p class="eyebrow">"EVERY JOURNEY STARTS SOMEWHERE"</p>
                <p class="coastal-title">"From Wellfleet,"<br/><em>"to the world."</em></p>
            </div>
            <div class="coastal-content" on:focusin=move |_| finish()>{children()}</div>
            <div class="coastal-controls wrap">
                <div class="coastal-place"><span class="coastal-marker" aria-hidden="true">"◉"</span><div><strong>"Wellfleet, Massachusetts"</strong><span>"Neal’s home on Cape Cod"</span></div></div>
                <div class="coastal-progress" aria-hidden="true"><div><span>"Horizon"</span><span>"Harbor"</span><span>"Water"</span><span>"Shore"</span></div><i><b></b></i></div>
                <button type="button" class="coastal-action" on:click=move |_| {
                    if finished.get() {
                        ready.set(false);
                        finished.set(false);
                        replay.update(|run| *run += 1);
                    } else { finish(); }
                }>{move || if finished.get() { "Replay coast" } else { "Skip intro" }}<span aria-hidden="true">{move || if finished.get() { "↺" } else { "↗" }}</span></button>
            </div>
        </div>
    }
}
