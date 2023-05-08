use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

pub struct EventManager {
    senders: Arc<Mutex<HashMap<(TypeId, TypeId), Vec<Box<dyn Any + Send>>>>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup<
        RequestType: std::marker::Send + Clone + 'static,
        ResponseType: std::marker::Send + Clone + 'static,
    >(
        &mut self,
    ) -> UnboundedSender<RequestType> {
        let (sender, mut receiver) = mpsc::unbounded_channel::<RequestType>();

        let senders = self.senders.clone();

        self.senders.lock().unwrap().insert(
            (TypeId::of::<RequestType>(), TypeId::of::<ResponseType>()),
            Vec::new(),
        );

        tokio::spawn(async move {
            while let Some(event) = receiver.recv().await {
                Self::broadcast(event, &senders.lock().unwrap());
            }
        });

        sender
    }

    pub fn get_receiver<
        RequestType: std::marker::Send + Clone + 'static,
        ResponseType: std::marker::Send + Clone + 'static,
    >(
        &mut self,
    ) -> UnboundedReceiver<ResponseType> {
        let (sender, receiver) = mpsc::unbounded_channel::<ResponseType>();

        self.senders
            .lock()
            .unwrap()
            .get_mut(&TypeId::of::<RequestType>())
            .unwrap()
            .push(Box::new(sender));

        receiver
    }

    // TODO: unsubscribe

    fn broadcast<ResponseType: std::marker::Send + Clone + 'static>(
        event: ResponseType,
        senders: &HashMap<TypeId, Vec<Box<dyn Any + Send>>>,
    ) {
        let event_type = &TypeId::of::<ResponseType>();

        for sender in senders.get(event_type).unwrap() {
            sender
                .downcast_ref::<UnboundedSender<ResponseType>>()
                .unwrap()
                .send(event.clone());
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            senders: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
