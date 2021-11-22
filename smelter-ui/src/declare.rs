use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

use crate::{DOMElement, DOMContext, DOMElementType};

pub trait DeclareElement {
    type Context;

    fn context_mut(&mut self) -> &mut Self::Context;
    fn element(&self) -> &Arc<DOMElement>;
}

pub trait DeclareTraverse {
    type Context;

    fn children<F>(self, f: F) -> Self where F: FnOnce(&mut Self::Context);
}

pub trait DeclareTextManipulate {
    fn text<S>(self, text: S) -> Self where S: Into<String>;
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

impl<'a, Ctx> DeclareElement for Element<'a, Ctx> where Ctx: DOMContext {
    type Context = Ctx;

    fn context_mut(&mut self) -> &mut Self::Context {
        self.context
    }

    fn element(&self) -> &Arc<DOMElement> {
        &self.element
    }
}

impl<T, Ctx> DeclareTraverse for T where T: DeclareElement<Context = Ctx>, Ctx: DOMContext {
    type Context = Ctx;

    fn children<F>(mut self, f: F) -> Self where F: FnOnce(&mut Self::Context) {
        let element = Arc::clone(self.element());
        let context = self.context_mut();
        context.enter_children(&element);
        f(context);
        context.leave_children(&element);
        self
    }
}
