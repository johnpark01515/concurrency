use anyhow::Result;
use std::{
    sync::mpsc::{self, Sender},
    thread,
    time::Duration,
};

const THREAD_NUM: u8 = 10;

fn main() -> Result<()> {
    let (s, r) = mpsc::channel();
    let mut s_list = Vec::new();
    for i in 1..=THREAD_NUM {
        let sender = s.clone();
        let t = thread::spawn(move || gen_num(sender, i));
        s_list.push(t);
    }
    drop(s);
    s_list.push(thread::spawn(move || {
        for msg in r.iter() {
            println!("get msg :{}, index:{}", msg.msg, msg.index);
        }
    }));
    for sj in s_list {
        sj.join().unwrap();
    }
    Ok(())
}

#[derive(Debug)]
struct Msg {
    msg: String,
    index: u8,
}

impl Msg {
    fn new(index: u8) -> Self {
        Msg {
            msg: format!("the num is {}", rand::random::<u8>()),
            index,
        }
    }
}

fn gen_num(s: Sender<Msg>, i: u8) {
    for _ in 1..10 {
        let msg = Msg::new(i);
        _ = s.send(msg);
        thread::sleep(Duration::from_millis(100))
    }
}
