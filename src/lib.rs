use wasm_bindgen::prelude::*;
use web_sys::console;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let image = web_sys::HtmlImageElement::new().unwrap();
    image.set_src("Idle (1).png");
    
    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}
