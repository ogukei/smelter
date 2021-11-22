use std::{marker::PhantomData, sync::{Arc, Mutex, Weak}};

pub trait Subscribe {
    type Input;

    fn receive_subscription(&self, subscription: Arc<Subscription<Self::Input>>);
    fn receive_value(&self, v: &Self::Input) -> Demand;
}

pub trait Publish {
    type Output;

    fn receive_subscriber(&self, subscriber: &Arc<Subscriber<Self::Output>>);
    fn send_value(&self, v: Self::Output);
}

pub struct Subscriber<T> {
    state: Mutex<SubscriberState<T>>,
}

impl<T> Subscriber<T> {
    pub fn new() -> Arc<Self> {
        let subscriber = Self {
            state: Mutex::new(SubscriberState::new()),
        };
        Arc::new(subscriber)
    }

    pub fn sink<F>(&self, f: F) where F: FnMut(&T), F: 'static {
        if let Ok(mut guard) = self.state.lock() {
            guard.sink(f);
        }
    }
}

impl<T> Subscribe for Subscriber<T> {
    type Input = T;

    fn receive_subscription(&self, subscription: Arc<Subscription<Self::Input>>) {
        if let Ok(mut guard) = self.state.lock() {
            guard.receive_subscription(subscription);
        }
    }

    fn receive_value(&self, v: &Self::Input) -> Demand {
        if let Ok(mut guard) = self.state.lock() {
            guard.receive_value(v)
        } else {
            Demand::nothing()
        }
    }
}

struct SubscriberState<T> {
    v: PhantomData<T>,
    sinks: Vec<Box<dyn FnMut(&T)>>,
}

impl<T> SubscriberState<T> {
    fn new() -> Self {
        Self {
            v: PhantomData,
            sinks: vec![],
        }
    }

    fn receive_subscription(&mut self, subscription: Arc<Subscription<T>>) {
        // TODO: automatic reference synced disposal 
        //self.subscription = Some(subscription);
    }

    fn receive_value(&mut self, v: &T) -> Demand {
        for sink in self.sinks.iter_mut() {
            let sink = sink.as_mut();
            sink(v);
        }
        Demand::unlimited()
    }

    fn sink<F>(&mut self, f: F) where F: FnMut(&T), F: 'static {
        self.sinks.push(Box::new(f));
    }
}

pub struct Publisher<T> {
    state: Mutex<PublisherState<T>>,
}

impl<T> Publisher<T> {
    fn new() -> Arc<Self> {
        let publisher = Self {
            state: Mutex::new(PublisherState::new()),
        };
        let publisher = Arc::new(publisher);
        // pass weak reference
        if let Ok(mut guard) = publisher.state.lock() {
            guard.initialize(&publisher);
        }
        publisher
    }
}

impl<T> Publish for Publisher<T> {
    type Output = T;

    fn receive_subscriber(&self, subscriber: &Arc<Subscriber<T>>) {
        if let Ok(mut guard) = self.state.lock() {
            guard.receive_subscriber(subscriber);
        }
    }

    fn send_value(&self, v: Self::Output) {
        if let Ok(mut guard) = self.state.lock() {
            guard.send_value(v);
        }
    }
}

struct PublisherState<T> {
    v: PhantomData<T>,
    publisher: Option<Weak<Publisher<T>>>,
    subscriptions: Vec<Arc<Subscription<T>>>,
}

impl<T> PublisherState<T> {
    fn new() -> Self {
        Self {
            v: PhantomData,
            publisher: None,
            subscriptions: vec![],
        }
    }

    fn initialize(&mut self, publisher: &Arc<Publisher<T>>) {
        self.publisher = Some(Arc::downgrade(publisher));
    }

    fn receive_subscriber(&mut self, subscriber: &Arc<Subscriber<T>>) {
        let subscription = Subscription::new(subscriber);
        self.subscriptions.push(Arc::clone(&subscription));
        subscriber.receive_subscription(subscription);
    }

    fn send_value(&mut self, v: T) {
        let subscriptions: Vec<_> = self.subscriptions
            .iter()
            .cloned()
            .filter_map(|subscription| {
                let demand = subscription.receive_value(&v);
                demand
                    .consumed(0)
                    .map(|_| subscription)
            })
            .collect();
        self.subscriptions = subscriptions;
    }
}

pub struct Demand {
    count: u64,
}

impl Demand {
    pub fn nothing() -> Self {
        Self { count: 0 }
    }

    pub fn unlimited() -> Self {
        Self { count: u64::MAX }
    }

    pub fn consumed(self, count: u64) -> Option<Self> {
        if self.count == u64::MAX {
            Some(self)
        } else if self.count >= count {
            Some(Self { count: self.count - count })
        } else {
            None
        }
    }
}

pub struct Subscription<T> {
    state: Mutex<SubscriptionState<T>>,
}

impl<T> Subscription<T> {
    pub fn new(subscriber: &Arc<Subscriber<T>>) -> Arc<Self> {
        let subscription = Self {
            state: Mutex::new(SubscriptionState::new(subscriber)),
        };
        Arc::new(subscription)
    }

    pub fn receive_value(&self, v: &T) -> Demand {
        if let Ok(mut guard) = self.state.lock() {
            guard.receive_value(v)
        } else {
            Demand::nothing()
        }
    }
}

struct SubscriptionState<T> {
    demand: Demand,
    subscriber: Weak<Subscriber<T>>,
}

impl<T> SubscriptionState<T> {
    pub fn new(subscriber: &Arc<Subscriber<T>>) -> Self {
        Self {
            demand: Demand::unlimited(),
            subscriber: Arc::downgrade(&subscriber),
        }
    }

    pub fn receive_value(&mut self, v: &T) -> Demand {
        println!("receive_valuereceive_valuereceive_value");
        if let Some(subscriber) = self.subscriber.upgrade() {
            println!("1");
            subscriber.receive_value(v)
        } else {
            println!("2");
            Demand::nothing()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let publisher: Arc<Publisher<u64>> = Publisher::new();
        let subscriber: Arc<Subscriber<u64>> = Subscriber::new();
        let x: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        let r = Arc::clone(&x);
        subscriber
            .sink(move |v| *r.lock().unwrap() = *v);
        publisher.receive_subscriber(&subscriber);
        publisher.send_value(100);
        assert_eq!(*x.lock().unwrap(), 100);
    }
}
