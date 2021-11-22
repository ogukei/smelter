use std::{borrow::{Borrow, BorrowMut}, marker::PhantomData, sync::{Arc, Mutex}, thread::current};

mod element;
pub use crate::element::*;

mod document;
pub use crate::document::*;

mod declare;
pub use crate::declare::*;

mod builder;
pub use crate::builder::*;

mod manipulation;
pub use crate::manipulation::*; 

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
