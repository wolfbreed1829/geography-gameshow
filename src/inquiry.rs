use leptos::prelude::*;

fn initial_program() -> &'static str {
    let query = web_sys::window()
        .and_then(|w| w.location().search().ok())
        .unwrap_or_default();
    match query
        .trim_start_matches('?')
        .split('&')
        .find_map(|part| part.strip_prefix("program="))
    {
        Some("art") => "Art — concept inquiry",
        Some("geography") => "The Geography Gameshow",
        _ => "Not sure yet",
    }
}

#[component]
pub fn Inquiry() -> impl IntoView {
    let school = RwSignal::new(String::new());
    let name = RwSignal::new(String::new());
    let email = RwSignal::new(String::new());
    let phone = RwSignal::new(String::new());
    let address = RwSignal::new(String::new());
    let country = RwSignal::new(String::new());
    let topic = RwSignal::new(initial_program().to_string());
    let notes = RwSignal::new(String::new());
    let step = RwSignal::new(0usize);
    let draft = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if step.get() < 3 {
            step.update(|s| *s += 1);
        } else {
            draft.set(Some(format!("School: {}\nAdministrator / teacher: {}\nEmail: {}\nPhone: {}\nSchool address: {}\nCountry / territory: {}\nProgram: {}\n\nAdditional details:\n{}",
                school.get().trim(),name.get().trim(),email.get().trim(),phone.get().trim(),address.get().trim(),country.get().trim(),topic.get(),
                if notes.get().trim().is_empty() {"Not provided".to_string()} else {notes.get().trim().to_string()})));
        }
    };
    let panels=vec![
        ("Your inquiry",view!{
            <Show when=move ||draft.get().is_some() fallback=move ||view!{
                <form class="step-form" on:submit=submit>
                    <p class="step-label" role="status">{move ||format!("Step {} of 4 · {}",step.get()+1,["School","Contact","Location","Program"][step.get()])}</p>
                    <div class="step-fields">
                        <Show when=move ||step.get()==0>
                            <label>"School name"<input required maxlength="180" autocomplete="organization" name="school" prop:value=move ||school.get() on:input=move |e|school.set(event_target_value(&e))/></label>
                            <label>"Administrator or teacher name"<input required maxlength="160" autocomplete="name" name="name" prop:value=move ||name.get() on:input=move |e|name.set(event_target_value(&e))/></label>
                        </Show>
                        <Show when=move ||step.get()==1>
                            <label>"Email address"<input required maxlength="254" type="email" autocomplete="email" name="email" prop:value=move ||email.get() on:input=move |e|email.set(event_target_value(&e))/></label>
                            <label>"Phone number"<input required maxlength="80" type="tel" autocomplete="tel" name="phone" placeholder="+ country code and number" prop:value=move ||phone.get() on:input=move |e|phone.set(event_target_value(&e))/></label>
                            <p class="field-help">"Include your country code. Any international format."</p>
                        </Show>
                        <Show when=move ||step.get()==2>
                            <label>"School address"<textarea required maxlength="1000" autocomplete="street-address" name="address" rows="2" placeholder="Street, city, region, and postal code (if used)" prop:value=move ||address.get() on:input=move |e|address.set(event_target_value(&e))></textarea></label>
                            <label>"Country or territory"<input required maxlength="120" autocomplete="country-name" name="country" prop:value=move ||country.get() on:input=move |e|country.set(event_target_value(&e))/></label>
                        </Show>
                        <Show when=move ||step.get()==3>
                            <label>"Program of interest"<select name="topic" prop:value=move ||topic.get() on:change=move |e|topic.set(event_target_value(&e))><option>"Not sure yet"</option><option>"The Geography Gameshow"</option><option>"Art — concept inquiry"</option><option>"Several programs"</option></select></label>
                            <label>"Additional details (optional)"<textarea maxlength="3000" name="details" rows="2" placeholder="Student ages, group size, possible dates…" prop:value=move ||notes.get() on:input=move |e|notes.set(event_target_value(&e))></textarea></label>
                        </Show>
                    </div>
                    <p class="inquiry-note" id="inquiry-note">"Preview only. Nothing is sent or saved."</p>
                    <nav class="reader-pagination" aria-label="Inquiry steps">
                        <button type="button" disabled=move ||step.get()==0 on:click=move |_|step.update(|s|*s=s.saturating_sub(1))>"Back"</button>
                        <button type="submit" class="button" aria-describedby="inquiry-note">{move ||if step.get()==3 {"Review inquiry"} else {"Continue"}}</button>
                    </nav>
                </form>
            }>
                <div class="draft-preview contained-draft"><h2>"Your inquiry preview"</h2><p>"These details have not been sent or saved."</p>
                    {move ||view!{<crate::screen::Reader text=draft.get().unwrap_or_default()/>}}
                    <button type="button" on:click=move |_|{draft.set(None);step.set(0); }>"Edit inquiry"</button>
                </div>
            </Show>
        }.into_any()),
        ("Scheduling",view!{<crate::screen::Reader text="Neal personally confirms availability and schedules each show. An inquiry is a request, not a confirmed booking.\n\nA school down the road or an ocean away. Neal travels internationally to bring live learning to curious students everywhere.\n\nInternational inquiries welcome. Tell us where you are and what you have in mind.\n\nAll fields are required except additional details. Use your school’s full address in your local format.".to_string()/><div class="screen-links"><a href="planning.html">"Read the school visit information"</a></div>}.into_any())
    ];
    view! {<section class="screen-page wrap"><h1>"Request a show"</h1><crate::screen::Deck panels=panels/></section>}
}
