use wasm_bindgen::prelude::*;

use smelter_ui::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let mut builder = DocumentBuilder::new();
    let context = &mut builder;
    Element::new(context).children(|context| {
        Element::new(context);
        Element::new(context).children(|context| {
            Element::new(context);
        });
    });
    let document = builder.build();
    let _ = document.instantiate();
    Ok(())
}
