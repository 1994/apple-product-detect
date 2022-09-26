extern crate core;

use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

use clokwerk::{Interval, Scheduler};

use crate::push::PushFactory;

mod monitor;
mod mapper;
mod push;
mod utils;
mod cli;
mod config;


fn main() -> anyhow::Result<()> {
    let request = cli::get_request()?;
    let r = request.1;
    println!("{:?}", r);
    let web = request.0;
    let a = Arc::new(web);
    let mut scheduler = Scheduler::new();
    scheduler.every(Interval::Seconds(10))
        .run(
            move || {
                let response = monitor::detect(&r);
                if response.is_err() {
                    println!("detect error, response:{:?}", response);
                    return;
                }
                let result = response.unwrap();
                if !result.is_available() {
                    println!("current:{:?} not available", SystemTime::now());
                    return;
                }
                let p = PushFactory::default();
                let content = String::from(result);
                let t = a.to_string();
                if !t.is_empty() {
                    let _ = p.push(content, t);
                }
            }
        );
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("current:{:?} not available", SystemTime::now());
    }
}