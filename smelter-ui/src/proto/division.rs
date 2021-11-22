use std::sync::Arc;

use crate::{DOMContext, DOMElement, DOMElementType, DeclareElement};

pub struct Division<'a, Ctx> {
    context: &'a mut Ctx,
    element: Arc<DOMElement>,
}

impl<'a, Ctx> Division<'a, Ctx> where Ctx: DOMContext {
    pub fn new(context: &'a mut Ctx) -> Self {
        let element = context.create_element(DOMElementType::Div);
        Self { context: context, element: element, }
    }
}

impl<'a, Ctx> DeclareElement for Division<'a, Ctx> where Ctx: DOMContext {
    type Context = Ctx;

    fn context_mut(&mut self) -> &mut Self::Context {
        self.context
    }

    fn element(&self) -> &Arc<DOMElement> {
        &self.element
    }
}
