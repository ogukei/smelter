use std::sync::Arc;

use wasm_bindgen::prelude::*;

use smelter_ui::*;
use smelter_reflux::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let mut builder = DocumentBuilder::new();
    let context = &mut builder;
    Division::new(context).children(|context| {
        let text_content = Publisher::new();
        Paragraph::new(context)
            .text("Smelter")
            .subscribe_text(&text_content);
        Division::new(context).children(|context| {
            let onclick = Subscriber::new();
            onclick
                .map(|_| Some(format!("Clicked")))
                .bind(&text_content);
            Button::new(context)
                .text("Hello world!")
                .publish_onclick(&onclick);
        });
    });
    let document = builder.build();
    let _ = document.instantiate();
    Ok(())
}
