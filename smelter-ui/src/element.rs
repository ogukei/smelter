use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

use smelter_reflux::{Property, Publisher, Subscriber};

pub enum DOMElementType {
    Div,
    Button,
    Paragraph,
}

pub struct DOMElement {
    element_type: DOMElementType,
    pub(crate) state: Mutex<DOMElementState>,
}

impl DOMElement {
    pub(crate) fn new(element_type: DOMElementType) -> Arc<Self> {
        let state = DOMElementState::new(&element_type);
        let element = Self {
            element_type,
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

    pub(crate) fn element_type(&self) -> &DOMElementType {
        &self.element_type
    }

    pub(crate) fn text_property(&self) -> Option<Arc<Property<Option<String>>>> {
        self.state.lock()
            .ok()
            .map(|v| v.text_property().clone())
            .flatten()
    }

    pub(crate) fn onclick_publisher(&self) -> Option<Arc<Publisher<()>>> {
        self.state.lock()
            .ok()
            .map(|v| v.onclick_publisher().clone())
            .flatten()
    }
}

pub struct DOMElementState {
    pub(crate) children: Vec<Arc<DOMElement>>,
    pub(crate) text_property: Option<Arc<Property<Option<String>>>>,
    pub(crate) onclick_publisher: Option<Arc<Publisher<()>>>,
}

impl DOMElementState {
    fn push_child(&mut self, element: &Arc<DOMElement>) {
        self.children.push(Arc::clone(element));
    }
}

impl DOMElementState {
    fn new(tp: &DOMElementType) -> Self {
        Self {
            children: vec![],
            text_property: tp.has_text().then(|| Property::new(None)),
            onclick_publisher: tp.has_onclick().then(|| Publisher::new())
        }
    }

    fn text_property(&self) -> &Option<Arc<Property<Option<String>>>> {
        &self.text_property
    }

    fn onclick_publisher(&self) -> &Option<Arc<Publisher<()>>> {
        &self.onclick_publisher
    }
}

impl DOMElementType {
    fn has_text(&self) -> bool {
        match &self {
            &DOMElementType::Div => true,
            &DOMElementType::Button => true,
            &DOMElementType::Paragraph => true,
        }
    }

    fn has_onclick(&self) -> bool {
        match &self {
            &DOMElementType::Div => true,
            &DOMElementType::Button => true,
            &DOMElementType::Paragraph => true,
        }
    }
}
