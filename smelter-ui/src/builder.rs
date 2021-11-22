use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

use crate::{DOMDocument, DOMElement, DOMElementType};

pub trait DOMContext {
    fn enter_children(&mut self, element: &Arc<DOMElement>);
    fn leave_children(&mut self, element: &Arc<DOMElement>);

    fn create_element(&mut self, tp: DOMElementType) -> Arc<DOMElement>;
}

pub struct DocumentBuilder {
    root_elements: Vec<Arc<DOMElement>>,
    parent_stack: Vec<Arc<DOMElement>>,
    current_element: Option<Arc<DOMElement>>,
}

impl DocumentBuilder {
    pub fn new() -> Self {
        Self {
            root_elements: vec![],
            parent_stack: vec![],
            current_element: None,
        }
    }

    pub fn build(self) -> Arc<DOMDocument> {
        Arc::new(DOMDocument::new(self.root_elements))
    }
}

impl DOMContext for DocumentBuilder {
    fn enter_children(&mut self, element: &Arc<DOMElement>) {
        if let Some(element) = self.current_element.take() {
            self.parent_stack.push(element);
        }
        self.current_element = Some(Arc::clone(element));
    }

    fn leave_children(&mut self, element: &Arc<DOMElement>) {
        if let Some(current_element) = self.current_element.as_ref() {
            assert!(Arc::ptr_eq(element, current_element));
        }
        self.current_element = self.parent_stack.pop();
    }

    fn create_element<'a>(&'a mut self, tp: DOMElementType) -> Arc<DOMElement> {
        let element = DOMElement::new(tp);
        if let Some(current_element) = self.current_element.as_ref() {
            current_element.push_child(&element);
        } else {
            self.root_elements.push(Arc::clone(&element));
        }
        element
    }
}
