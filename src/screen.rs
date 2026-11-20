use leptos::prelude::*;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use std::rc::Rc;

#[component]
pub fn Deck(
    panels: Vec<(&'static str, AnyView)>,
    #[prop(optional)] selected: Option<RwSignal<usize>>,
) -> impl IntoView {
    let active = selected.unwrap_or_else(|| RwSignal::new(0));
    let labels = panels.iter().map(|(label, _)| *label).collect::<Vec<_>>();
    let options = labels.clone();
    let many = labels.len() > 4;
    view! {
        <div class="screen-deck" class:many-sections=many>
            <div class="screen-tabs" role="group" aria-label="Page sections">
                {labels.into_iter().enumerate().map(|(i,label)| view! {
                    <button type="button" aria-pressed=move || (active.get()==i).to_string() on:click=move |_| active.set(i)>{label}</button>
                }).collect_view()}
            </div>
            <label class="screen-section-select"><span class="sr-only">"Page section"</span><select prop:value=move || active.get().to_string() on:change=move |e| { if let Ok(i)=event_target_value(&e).parse::<usize>() {active.set(i);} }>
                {options.into_iter().enumerate().map(|(i,label)| view! {<option value=i.to_string()>{label}</option>}).collect_view()}
            </select></label>
            <div class="screen-panels">
                {panels.into_iter().enumerate().map(|(i,(_,content))| view! {<section class="screen-panel" hidden=move || active.get()!=i>{content}</section>}).collect_view()}
            </div>
        </div>
    }
}

// Measure real rendered text at the current size. Page boundaries always fall at
// whitespace, retaining every character, paragraph, and source credit.
#[component]
pub fn Reader(text: String) -> impl IntoView {
    let body = NodeRef::<leptos::html::Div>::new();
    let probe = NodeRef::<leptos::html::P>::new();
    let pages = RwSignal::new(vec![String::new()]);
    let current = RwSignal::new(0usize);
    let source = Rc::new(text);
    Effect::new(move |_| {
        if let (Some(body), Some(probe)) = (body.get(), probe.get()) {
            let source = source.clone();
            let observed_body = body.clone();
            let measure = Rc::new(move || {
                if pages.is_disposed() {
                    return;
                }
                let height = body.client_height();
                if height < 24 || body.client_width() < 40 {
                    return;
                }
                let boundaries = source
                    .char_indices()
                    .map(|(i, _)| i)
                    .chain(std::iter::once(source.len()))
                    .collect::<Vec<_>>();
                let mut result = Vec::new();
                let mut start = 0;
                while start + 1 < boundaries.len() {
                    let mut low = start + 1;
                    let mut high = boundaries.len() - 1;
                    let mut end = low;
                    while low <= high {
                        let mid = (low + high) / 2;
                        probe.set_text_content(Some(&source[boundaries[start]..boundaries[mid]]));
                        if probe.scroll_height() <= height - 2 {
                            end = mid;
                            low = mid + 1;
                        } else {
                            high = mid - 1;
                        }
                    }
                    // Prefer a word boundary; split an oversized word only when
                    // necessary so URLs and long user-entered text remain reachable.
                    if end + 1 < boundaries.len() {
                        if let Some((offset, ch)) = source[boundaries[start]..boundaries[end]]
                            .char_indices()
                            .rev()
                            .find(|(_, c)| c.is_whitespace())
                        {
                            let byte = boundaries[start] + offset + ch.len_utf8();
                            if byte > boundaries[start] {
                                end = boundaries.binary_search(&byte).unwrap_or(end);
                            }
                        }
                    }
                    result.push(source[boundaries[start]..boundaries[end]].to_string());
                    start = end;
                }
                if result.is_empty() {
                    result.push(String::new());
                }
                let last = result.len() - 1;
                if current.get_untracked() > last {
                    current.set(last);
                }
                pages.set(result);
            });
            let callback_measure = measure.clone();
            let callback =
                Closure::<dyn FnMut(js_sys::Array, web_sys::ResizeObserver)>::new(move |_, _| {
                    callback_measure()
                });
            if let Ok(observer) = web_sys::ResizeObserver::new(callback.as_ref().unchecked_ref()) {
                observer.observe(&observed_body);
                let _callback = StoredValue::new_local(callback);
                on_cleanup(move || observer.disconnect());
            }
            leptos::task::spawn_local(async move {
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    if let Ok(ready) = document.fonts().ready() {
                        let _ = js_sys::futures::JsFuture::from(ready).await;
                    }
                }
                measure();
            });
        }
    });
    view! {
        <div class="screen-reader">
            <div class="reader-body" node_ref=body>
                <p class="reader-text" aria-live="polite">{move || pages.with(|p| p.get(current.get()).cloned().unwrap_or_default())}</p>
                <p class="reader-text reader-probe" node_ref=probe aria-hidden="true"></p>
            </div>
            <nav class="reader-pagination" aria-label="Reading pages">
                <button type="button" disabled=move || current.get()==0 on:click=move |_| current.update(|i| *i=i.saturating_sub(1))>"Previous"</button>
                <span role="status">{move || format!("{} / {}", current.get()+1,pages.with(Vec::len))}</span>
                <button type="button" disabled={move || current.get()+1>=pages.with(Vec::len)} on:click=move |_| current.update(|i| *i+=1)>"Next"</button>
            </nav>
        </div>
    }
}
