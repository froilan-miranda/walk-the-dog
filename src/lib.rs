use wasm_bindgen::prelude::*;
use web_sys::console;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let image = web_sys::HtmlImageElement::new().unwrap();
    let callback = Closure::once(|| {
        web_sys::console::log_1(&JsValue::from_str("loaded"));
    });
    image.set_onload(Some(callback.as_ref().unchecked_ref()));
    callback.forget();
    image.set_src("Idle (1).png");
    context.draw_image_with_html_image_element(&image, 0.0, 0.0);

    console::log_1(&JsValue::from_str("Made it to the end!"));

    Ok(())
}
