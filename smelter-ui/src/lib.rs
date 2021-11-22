use std::{borrow::{Borrow, BorrowMut}, marker::PhantomData, sync::{Arc, Mutex}, thread::current};

pub trait DOMContext {
    fn enter_children(&mut self, element: &Arc<DOMElement>);
    fn leave_children(&mut self, element: &Arc<DOMElement>);

    fn create_element(&mut self, tp: DOMElementType) -> Arc<DOMElement>;
}

pub struct DOMElement {
    state: Mutex<DOMElementState>,
}

impl DOMElement {
    fn new(tp: DOMElementType) -> Arc<Self> {
        let state = DOMElementState::new();
        let element = Self {
            state: Mutex::new(state),
        };
        Arc::new(element)
    }

    fn push_child(&self, element: &Arc<DOMElement>) {
        if let Ok(mut guard) = self.state.lock() {
            let state = guard.borrow_mut();
            state.push_child(element);
        }
    }
}

pub struct DOMElementState {
    children: Vec<Arc<DOMElement>>,
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

pub enum DOMElementType {
    Div,
}

pub trait Visit {
    type Context;

    fn context_mut(&mut self) -> &mut Self::Context;
    fn element(&self) -> &Arc<DOMElement>;
}

pub trait Manipulate {
    type Context;

    fn children<F>(self, f: F) -> Self where F: Fn(&mut Self::Context);
}

pub struct Element<'a, Ctx> {
    context: &'a mut Ctx,
    element: Arc<DOMElement>,
}

impl<'a, Ctx> Element<'a, Ctx> where Ctx: DOMContext {
    pub fn new(context: &'a mut Ctx) -> Self {
        let element = context.create_element(DOMElementType::Div);
        Self { context: context, element: element, }
    }
}

impl<T, Ctx> Manipulate for T where T: Visit<Context = Ctx>, Ctx: DOMContext {
    type Context = Ctx;

    fn children<F>(mut self, f: F) -> Self where F: Fn(&mut Self::Context) {
        let element = Arc::clone(self.element());
        let context = self.context_mut();
        context.enter_children(&element);
        f(context);
        context.leave_children(&element);
        self
    }
}

impl<'a, Ctx> Visit for Element<'a, Ctx> where Ctx: DOMContext {
    type Context = Ctx;

    fn context_mut(&mut self) -> &mut Self::Context {
        self.context
    }

    fn element(&self) -> &Arc<DOMElement> {
        &self.element
    }
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

pub struct DOMDocument {
    elements: Vec<Arc<DOMElement>>,
}

impl DOMDocument {
    fn new(elements: Vec<Arc<DOMElement>>) -> Self {
        Self { elements }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut builder = DocumentBuilder::new();
        let context = &mut builder;
        Element::new(context).children(|context| {
            Element::new(context).children(|_| {
            });
        });
        let document = builder.build();
        assert_eq!(document.elements.len(), 1);
    }
}

// import dom manipulations
mod instantiate;
pub use crate::instantiate::*; 
