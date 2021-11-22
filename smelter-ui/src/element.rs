use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

pub enum DOMElementType {
    Div,
}

pub struct DOMElement {
    pub(crate) state: Mutex<DOMElementState>,
}

impl DOMElement {
    pub(crate) fn new(tp: DOMElementType) -> Arc<Self> {
        let state = DOMElementState::new();
        let element = Self {
            state: Mutex::new(state),
        };
        Arc::new(element)
    }

    pub(crate) fn push_child(&self, element: &Arc<DOMElement>) {
        if let Ok(mut guard) = self.state.lock() {
            let state = guard.borrow_mut();
            state.push_child(element);
        }
    }
}

pub struct DOMElementState {
    pub(crate) children: Vec<Arc<DOMElement>>,
}

impl DOMElementState {
    fn push_child(&mut self, element: &Arc<DOMElement>) {
        self.children.push(Arc::clone(element));
    }
}

impl DOMElementState {
    fn new() -> Self {
        Self { children: vec![] }
    }
}
