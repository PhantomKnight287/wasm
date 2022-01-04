use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("Global window object not present, Probably due to SSR");
    let document = window
        .document()
        .expect("Global document object not present, Probably due to SSR");
    let body = document.body().expect("document should have a body");
    let val = document.create_element("h1")?;
    val.set_text_content(Some("Dom Manipulation Using Rust and Wasm"));

    body.append_child(&val)?;
    Ok(())
}
