use fastrand;
use std::{
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
    thread,
    time::Instant,
};

const L: usize = 6;
const BALLS: [&str; L] = ["r", "g", "g", "b", "b", "b"];

fn main() {
    let mut args = std::env::args();
    let mut simulations = args
        .nth(1)
        .map_or(1_000_000, |x| x.parse::<usize>().unwrap());
    let threads = args.next().map_or(8, |x| x.parse::<usize>().unwrap());

    println!("running {simulations} sims with {threads} threads");

    if simulations % threads != 0 {
        simulations += threads - simulations % threads;
        println!("changed simulations to {simulations}, as it was not a multiple of [threads]({threads})");
    }

    let start = Instant::now();
    let mut handles = vec![];
    let total = Arc::new(AtomicI64::new(0));

    for _ in 0..threads {
        let total = Arc::clone(&total);
        let mut a = 0;
        let t = thread::spawn(move || {
            let mut rng = fastrand::Rng::new();
            for _ in 0..(simulations / threads) {
                let mut g = 0;
                let mut b = 0;

                loop {
                    let i = rng.usize(0..L);
                    match BALLS[i] {
                        "r" => break,
                        "g" => g += 1,
                        "b" => b += 1,
                        _ => panic!(),
                    }
                }

                a += g * b;
            }

            total.fetch_add(a, Ordering::Relaxed);
        });

        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let area = total.load(Ordering::Relaxed) as f32 / simulations as f32;
    let elapsed = start.elapsed().as_secs_f64();

    println!("{area}");
    println!("{}ms", elapsed * 1000f64);
    println!("{} simulations/s", simulations as f64 / elapsed);
}
