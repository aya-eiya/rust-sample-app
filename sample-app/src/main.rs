mod domains;
mod infras;
use domains::app_date::AppDateService;

use infras::app_date::boosted::BoostedAppDateService;
use infras::app_date::system::SystemAppDateService;

use std::{thread, time};

fn main() {
    let system = SystemAppDateService::new();
    let boosted = BoostedAppDateService::new("", 0, 48);

    loop {
        let delay = time::Duration::from_secs(3);

        let date = system.now().display();
        let date2 = boosted.now().display();
        println!("system:{date}, boosted:{date2}");

        thread::sleep(delay);
    }
}
