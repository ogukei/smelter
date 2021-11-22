
use std::{borrow::BorrowMut, sync::{Arc, Mutex}};

use smelter_reflux::{Publisher, Subscriber, Publish};

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
    fn publish_onclick(self, subscriber: &Arc<Subscriber<()>>) -> Self;
    fn subscribe_text(self, publisher: &Arc<Publisher<Option<String>>>) -> Self;
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

impl<T, Ctx> DeclareTextManipulate for T where T: DeclareElement<Context = Ctx>, Ctx: DOMContext {
    fn text<S>(self, text: S) -> Self where S: Into<String> {
        let element = self.element();
        if let Some(text_property) = element.text_property() {
            text_property.accept(&Some(text.into()));
        }
        self
    }

    fn publish_onclick(self, subscriber: &Arc<Subscriber<()>>) -> Self {
        let element = self.element();
        if let Some(onclick_publisher) = element.onclick_publisher() {
            onclick_publisher.receive_subscriber(subscriber);
        }
        self
    }

    fn subscribe_text(self, publisher: &Arc<Publisher<Option<String>>>) -> Self {
        let element = self.element();
        if let Some(text_property) = element.text_property() {
            publisher.receive_subscriber(text_property.subscriber())
        }
        self
    }
}

mod division;
pub use crate::proto::division::*;

mod button;
pub use crate::proto::button::*;

mod paragraph;
pub use crate::proto::paragraph::*;
