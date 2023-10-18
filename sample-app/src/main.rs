mod domains;
mod infras;
use async_trait::async_trait;
use domains::app_date::AppDateService;

use infras::app_date::boosted::BoostedAppDateService;
use infras::app_date::system::SystemAppDateService;

use futures::executor;
use std::{thread, time};

fn main() {
    let r = Runner {};
    executor::block_on(r.run());
}

struct Runner {}

#[async_trait]
pub trait Runable {
    async fn run(&self);
}

#[async_trait]
impl Runable for Runner {
    async fn run(&self) {
        let system = SystemAppDateService::new();
        let boosted = BoostedAppDateService::new("", 0, 48);

        loop {
            let delay = time::Duration::from_secs(3);

            let date = system.now().await.display();
            let date2 = boosted.now().await.display();
            println!("system:{date}, boosted:{date2}");

            thread::sleep(delay);
        }
    }
}
