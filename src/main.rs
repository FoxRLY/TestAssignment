use multithreading::std_flavour;
use std::{sync::mpsc::channel, time::Duration};

fn main() {
    let a: Vec<_> = (0..100).collect();
    let b: Vec<_> = (200..300).collect();

    let (rx, tx) = channel();
    std_flavour::parallel_func(a, move |x| {
        std::thread::sleep(Duration::from_secs(2));
        rx.clone().send(format!("{x}")).unwrap();
    });
    while let Ok(val) = tx.try_recv() {
        println!("{val}");
    }

    let result = std_flavour::parallel_func(b, std_flavour::test_func);
    result.iter().for_each(|x| println!("{x}"));
}

