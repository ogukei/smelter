
use std::{borrow::BorrowMut, sync::Arc};

use crate::{DOMDocument, DOMElement, DOMElementState};
use wasm_bindgen::prelude::*;
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
}

impl DOMElementInstance {
    fn new(
        reference: &Arc<DOMElement>,
        reference_children: Vec<Arc<DOMElement>>,
        document: &web_sys::Document,
        parent: &web_sys::Element,
    ) -> Arc<Self> {
        let element = document.create_element("div").unwrap();
        parent.append_child(&element);
        let children: Vec<_> = reference_children.iter()
            .map(|child| child.instantiate(document, &element))
            .collect();
        let instance = Self {
            reference_element: Arc::clone(reference),
            reference_children,
            element,
            children,
        };
        let instance = Arc::new(instance);
        instance
    }
}
