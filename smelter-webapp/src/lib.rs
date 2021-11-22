use std::sync::Arc;

use wasm_bindgen::prelude::*;

use smelter_ui::*;
use smelter_reflux::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let mut builder = DocumentBuilder::new();
    let context = &mut builder;
    Element::new(context).children(|context| {
        Element::new(context);
        Element::new(context).children(|context| {
            let onclick = Subscriber::new();
            let text_content = Publisher::new();
            onclick
                .map(|_| Some(format!("Clicked")))
                .bind(&text_content);
            Button::new(context)
                .text("Hello world!")
                .publish_onclick(&onclick)
                .subscribe_text(&text_content);
        });
    });
    let document = builder.build();
    let _ = document.instantiate();
    Ok(())
}
