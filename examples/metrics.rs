use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let map = Metrics::new();
    for idx in 0..5 {
        task_worker(idx, map.clone());
    }

    for _ in 0..6 {
        request_worker(map.clone())?;
    }
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("test:here{:?}", map.snapshot());
    }
}

fn task_worker(idx: usize, map: Metrics<i32>) {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(10..500)));
        map.inc(format!("thread_worker: {idx}"), 1).unwrap();
    });
}

fn request_worker(map: Metrics<i32>) -> Result<()> {
    thread::spawn(move || loop {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(50..100)));
        let page = rng.gen_range(1..5);
        map.inc(format!("request worker got page: {page}"), 1)
            .unwrap();
    });
    Ok(())
}
