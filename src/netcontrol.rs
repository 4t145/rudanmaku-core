// control request frequency
use tokio::{sync::Mutex, time::{Duration, Instant, sleep_until}};
use std::sync::Arc;
#[derive(Clone, Debug)]
pub struct Cooldown {
    cooldown: Duration,
    next: Arc<Mutex<Instant>>
}

impl Cooldown {
    pub fn new(cooldown: Duration) -> Self {
        Self { cooldown, next: Arc::new(Mutex::new(Instant::now())) }
    }

    pub async fn cooldown(&self) {
        let next = {
            let mut next = self.next.lock().await;
            *next = Instant::now() + self.cooldown;
            next.clone()
        };
        sleep_until(next).await;
    }
}

