use leptos::{ev::PointerEvent, wasm_bindgen::JsCast};

// Delegate pointer tracking so controls in tabs and pagination react too.
pub fn follow_pointer(event: PointerEvent) {
    let Some(target) = event
        .target()
        .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
    else {
        return;
    };
    let Ok(Some(control)) = target.closest("button, .button, .text-link, #main-nav > a, .explore-menu summary, .explore-links a, .program-subnav a, .screen-links a") else {
        return;
    };
    if control.has_attribute("disabled") {
        return;
    }
    let bounds = control.get_bounding_client_rect();
    if bounds.width() == 0.0 || bounds.height() == 0.0 {
        return;
    }
    let x = ((f64::from(event.client_x()) - bounds.left()) / bounds.width()).clamp(0.0, 1.0);
    let y = ((f64::from(event.client_y()) - bounds.top()) / bounds.height()).clamp(0.0, 1.0);
    if let Some(control) = control.dyn_ref::<web_sys::HtmlElement>() {
        let style = control.style();
        let _ = style.set_property("--pointer-x", &format!("{:.1}%", x * 100.0));
        let _ = style.set_property("--pointer-y", &format!("{:.1}%", y * 100.0));
        let _ = style.set_property("--tilt-x", &format!("{:.2}deg", (0.5 - y) * 6.0));
        let _ = style.set_property("--tilt-y", &format!("{:.2}deg", (x - 0.5) * 8.0));
    }
}
