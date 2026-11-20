use leptos::prelude::*;

#[component]
pub fn CoastalHome(children: Children) -> impl IntoView {
    view! {
        <div class="coastal-home">
            <div class="coastal-backdrop" aria-hidden="true">
                <img class="coastal-pan" src="public/wellfleet-coast.webp" alt="" width="887" height="1774" fetchpriority="high" decoding="async"/>
                <div class="coastal-shade"></div>
            </div>
            <div class="coastal-content">{children()}</div>
        </div>
    }
}
