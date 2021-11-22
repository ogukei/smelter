
use std::{borrow::BorrowMut, sync::Arc};

use crate::{DOMDocument, DOMElement, DOMElementState, DOMElementType, element};
use smelter_reflux::Publish;
use wasm_bindgen::{JsCast, prelude::*};
use web_sys::{Document, HtmlElement};

impl DOMDocument {
    pub fn instantiate(self: &Arc<Self>) -> Arc<DOMDocumentInstance> {
        let window = web_sys::window()
            .expect("no global `window` exists");
        let document = window.document()
            .expect("should have a document on window");
        let body = document.body()
            .expect("document should have a body");
        DOMDocumentInstance::new(self, &document, &body)
    }
}

impl DOMElement {
    fn instantiate(self: &Arc<Self>, document: &web_sys::Document, element: &web_sys::Element) -> Arc<DOMElementInstance> {
        let children = if let Ok(mut guard) = self.state.lock() {
            let state = guard.borrow_mut();
            state.children.iter()
                .cloned()
                .collect()
        } else {
            vec![]
        };
        DOMElementInstance::new(self, children, document, element)
    }
}

pub struct DOMDocumentInstance {
    reference_element: Arc<DOMDocument>,
    elements: Vec<Arc<DOMElementInstance>>,
}

impl DOMDocumentInstance {
    fn new(reference: &Arc<DOMDocument>, document: &web_sys::Document, element: &web_sys::Element) -> Arc<Self> {
        let elements: Vec<_> = reference.elements.iter()
            .map(|v| v.instantiate(document, &element))
            .collect();
        let instance = Self {
            reference_element: Arc::clone(reference),
            elements,
        };
        Arc::new(instance)
    }
}

pub struct DOMElementInstance {
    reference_element: Arc<DOMElement>,
    reference_children: Vec<Arc<DOMElement>>,
    element: web_sys::Element,
    children: Vec<Arc<DOMElementInstance>>,
    binding: DOMElementInstanceBinding,
}

impl DOMElementInstance {
    fn new(
        reference: &Arc<DOMElement>,
        reference_children: Vec<Arc<DOMElement>>,
        document: &web_sys::Document,
        parent: &web_sys::Element,
    ) -> Arc<Self> {
        let element_type = reference.element_type();
        let element_name = element_type.name();
        let element = document.create_element(element_name.as_str()).unwrap();
        parent.append_child(&element);
        let children: Vec<_> = reference_children.iter()
            .map(|child| child.instantiate(document, &element))
            .collect();
        let html_element = element
            .dyn_ref::<HtmlElement>()
            .unwrap();
        let binding = DOMElementInstanceBinding::new(reference, html_element);
        let instance = Self {
            reference_element: Arc::clone(reference),
            reference_children,
            element,
            children,
            binding,
        };
        let instance = Arc::new(instance);
        instance
    }
}

impl DOMElementType {
    fn name(&self) -> String {
        match &self {
            &DOMElementType::Div => "div".into(),
            &DOMElementType::Button => "button".into(),
            &DOMElementType::Paragraph => "p".into(),
        }
    }
}

struct DOMElementInstanceBinding {

}

impl DOMElementInstanceBinding {
    fn new(reference: &Arc<DOMElement>, element: &web_sys::HtmlElement) -> Self {
        // onclick
        if let Some(onclick_publisher) = reference.onclick_publisher() {
            let onclick = Closure::wrap(Box::new(move || {
                // web_sys::console::log_1(&JsValue::from(format!("clicked")));
                onclick_publisher.send_value(&());
            }) as Box<dyn FnMut()>);
            element.set_onclick(Some(onclick.as_ref().unchecked_ref()));
            onclick.forget();
        }
        // text_property
        if let Some(text_property) = reference.text_property() {
            let text = text_property.value().flatten();
            if let Some(text) = text {
                element.set_text_content(Some(text.as_str()));
            }
            let element = element.clone();
            text_property.subscriber()
                .sink(move |v| {
                    element.set_text_content(v.as_ref().map(|v| v.as_str()));
                });
        }
        let binding = Self {
        };
        binding
    }
}
