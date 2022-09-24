extern crate core;

use std::sync::Arc;
use std::thread;
use std::time::Duration;
use clokwerk::{Interval, Scheduler};
use crate::push::PushFactory;

mod monitor;
mod mapper;
mod push;
mod utils;
mod cli;


fn main() -> anyhow::Result<()> {
    let request = cli::get_request()?;
    let r = request.1;
    println!("{:?}", r);
    let web = request.0;
    let a = Arc::new(web);
    let mut scheduler = Scheduler::new();
    scheduler.every(Interval::Seconds(30))
        .run(
            move || {
                let response = monitor::detect(&r).expect("detect error");
                if response.is_available() {
                    let p = PushFactory::default();
                    let content = String::from(response);
                    let t = a.to_string();
                    if !t.is_empty() {
                        let _ = p.push(content, t);
                    }
                } else { println!("current unavailable") }
            }
        );
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
