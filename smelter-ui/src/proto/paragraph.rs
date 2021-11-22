use std::sync::Arc;

use crate::{DOMContext, DOMElement, DOMElementType, DeclareElement};

pub struct Paragraph<'a, Ctx> {
    context: &'a mut Ctx,
    element: Arc<DOMElement>,
}

impl<'a, Ctx> Paragraph<'a, Ctx> where Ctx: DOMContext {
    pub fn new(context: &'a mut Ctx) -> Self {
        let element = context.create_element(DOMElementType::Paragraph);
        Self { context: context, element: element, }
    }
}

impl<'a, Ctx> DeclareElement for Paragraph<'a, Ctx> where Ctx: DOMContext {
    type Context = Ctx;

    fn context_mut(&mut self) -> &mut Self::Context {
        self.context
    }

    fn element(&self) -> &Arc<DOMElement> {
        &self.element
    }
}
