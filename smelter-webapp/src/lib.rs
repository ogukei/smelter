use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!!!"));

    body.append_child(&val)?;

    Ok(())
}

/*
// Draft

let builder = DocumentBuilder::new();
let context = &builder;
Element::new(context).children(|context| {
    Label::new(context)
        .title("Foo");
});
let document = builder.build();
*/
