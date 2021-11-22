use std::sync::Arc;

use smelter_reflux::{Publish, Publisher, Subscriber};

use crate::{DOMContext, DOMElement, DOMElementType, DeclareElement};


pub struct Button<'a, Ctx> {
    context: &'a mut Ctx,
    element: Arc<DOMElement>,
}

impl<'a, Ctx> Button<'a, Ctx> where Ctx: DOMContext {
    pub fn new(context: &'a mut Ctx) -> Self {
        let element = context.create_element(DOMElementType::Button);
        Self { context: context, element: element, }
    }
}

impl<'a, Ctx> DeclareElement for Button<'a, Ctx> where Ctx: DOMContext {
    type Context = Ctx;

    fn context_mut(&mut self) -> &mut Self::Context {
        self.context
    }

    fn element(&self) -> &Arc<DOMElement> {
        &self.element
    }
}

impl<'a, Ctx> Button<'a, Ctx> where Ctx: DOMContext {
    pub fn text<S>(self, text: S) -> Self where S: Into<String> {
        let element = self.element();
        if let Some(text_property) = element.text_property() {
            text_property.accept(&Some(text.into()));
        }
        self
    }

    pub fn publish_onclick(self, subscriber: &Arc<Subscriber<()>>) -> Self {
        let element = self.element();
        if let Some(onclick_publisher) = element.onclick_publisher() {
            onclick_publisher.receive_subscriber(subscriber);
        }
        self
    }

    pub fn subscribe_text(self, publisher: &Arc<Publisher<Option<String>>>) -> Self {
        let element = self.element();
        if let Some(text_property) = element.text_property() {
            publisher.receive_subscriber(text_property.subscriber())
        }
        self
    }
}
