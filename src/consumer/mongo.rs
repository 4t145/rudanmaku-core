use bilive_danmaku::event::Event;
use tokio::{sync::broadcast::Receiver};

use super::Consumer;


pub struct MongoConsumer {
    receiver: Receiver<Event>
}



impl Consumer for MongoConsumer {
    type Out = mongodb::Collection<Event>;
    fn launch(receiver: Receiver<Event>) -> Self {
        return Self {
            receiver
        };
    }
    fn accept(&self, collection: Self::Out) {
        let mut receiver = self.receiver.resubscribe();
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                match collection.insert_one(event, None).await {
                    Ok(_) => {}
                    Err(_e) => {
                        // log error
                    }
                }
            }
        });
    }
}

