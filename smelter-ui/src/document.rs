use std::{sync::{Arc, Mutex}};

use crate::{DOMElement};

pub struct DOMDocument {
    pub(crate) elements: Vec<Arc<DOMElement>>,
}

impl DOMDocument {
    pub(crate) fn new(elements: Vec<Arc<DOMElement>>) -> Self {
        Self { elements }
    }
}
