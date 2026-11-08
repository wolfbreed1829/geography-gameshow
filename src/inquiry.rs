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
        Some("anatomy") => "Anatomy — concept inquiry",
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
    let draft = RwSignal::new(None::<String>);
    let submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        draft.set(Some(format!(
            "School: {}\nAdministrator / teacher: {}\nEmail: {}\nPhone: {}\nSchool address: {}\nCountry / territory: {}\nProgram: {}\n\nAdditional details:\n{}",
            school.get().trim(), name.get().trim(), email.get().trim(), phone.get().trim(),
            address.get().trim(), country.get().trim(), topic.get(),
            if notes.get().trim().is_empty() { "Not provided".to_string() } else { notes.get().trim().to_string() }
        )));
    };
    view! {
        <section class="booking section-space" aria-labelledby="booking-title"><div class="wrap booking-grid">
            <div class="booking-copy"><p class="eyebrow">"FROM YOUR CLASSROOM TO THE WORLD"</p><h1 id="booking-title">"Bring curiosity"<br/>"to your"<br/><span class="serif-line">"corner of the world."</span></h1><p>"A school down the road or an ocean away. Neal travels internationally to bring live learning to curious students everywhere."</p><div class="worldwide-note"><span aria-hidden="true">"↗"</span><div><strong>"International inquiries welcome"</strong><p>"Tell us where you are and what you have in mind."</p></div></div><a class="text-link" href="planning.html">"Read the school visit information"<span>"→"</span></a></div>
            <form class="inquiry-form" on:submit=submit on:input=move |_| draft.set(None)>
                <p class="eyebrow">"LET’S START WITH YOUR SCHOOL"</p><h2>"A new adventure starts here."</h2><p id="inquiry-note">"Preview form: review your details below. Nothing is sent or saved. All fields are required except additional details."</p>
                <label>"School name"<input required maxlength="180" autocomplete="organization" name="school" on:input=move |ev| school.set(event_target_value(&ev))/></label>
                <label>"Administrator or teacher name"<input required maxlength="160" autocomplete="name" name="name" on:input=move |ev| name.set(event_target_value(&ev))/></label>
                <div class="form-row"><label>"Email address"<input required maxlength="254" type="email" autocomplete="email" name="email" on:input=move |ev| email.set(event_target_value(&ev))/></label><label>"Phone number"<input required maxlength="80" type="tel" autocomplete="tel" name="phone" aria-describedby="phone-help" placeholder="+ country code and number" on:input=move |ev| phone.set(event_target_value(&ev))/><span class="field-help" id="phone-help">"Include your country code. Any international format."</span></label></div>
                <label>"School address"<textarea required maxlength="1000" autocomplete="street-address" name="address" rows="3" aria-describedby="address-help" placeholder="Street, city, region, and postal code (if used)" on:input=move |ev| address.set(event_target_value(&ev))></textarea><span class="field-help" id="address-help">"Use the full address in your local format."</span></label>
                <div class="form-row"><label>"Country or territory"<input required maxlength="120" autocomplete="country-name" name="country" on:input=move |ev| country.set(event_target_value(&ev))/></label><label>"Program of interest"<select name="topic" prop:value=move || topic.get() on:change=move |ev| {topic.set(event_target_value(&ev)); draft.set(None); }><option>"Not sure yet"</option><option>"The Geography Gameshow"</option><option>"Anatomy — concept inquiry"</option><option>"Art — concept inquiry"</option><option>"Several programs"</option></select></label></div>
                <label>"Additional details (optional)"<textarea maxlength="3000" name="details" rows="3" placeholder="Student ages, group size, possible dates, or anything you’d like Neal to know…" on:input=move |ev| notes.set(event_target_value(&ev))></textarea></label>
                <button class="button" type="submit" aria-describedby="inquiry-note">"Review inquiry"<span>"→"</span></button>
                <Show when=move || draft.get().is_some()><div class="draft-preview" role="status" aria-live="polite"><h3>"Your inquiry preview"</h3><p>"These details have not been sent or saved. You can edit the fields above to update your preview."</p><pre>{move || draft.get().unwrap_or_default()}</pre></div></Show>
            </form>
        </div></section>
    }
}
