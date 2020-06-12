extern crate jemallocator;
extern crate jemalloc_ctl;

use std::thread;
use std::time::Duration;
use jemalloc_ctl::{stats, epoch};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

static NTHREADS: i64 = 8000000000;

fn main() {
    let e = epoch::mib().unwrap();
    let allocated = stats::allocated::mib().unwrap();
    let resident = stats::resident::mib().unwrap();
    let mut children = vec![];

    for i in 0..NTHREADS {
        if i % 10000 == 0 && i != 0 {
            e.advance().unwrap();

            let allocated = allocated.read().unwrap() / 1_000_000;
            let resident = resident.read().unwrap() / 1_000_000;
            println!("{} threads: {:.4} MB allocated/{:.4} MB resident", i, allocated, resident);
            thread::sleep(Duration::from_secs(2));
        }

        children.push(thread::spawn(move || {
            let _simple_calc = 1337 + i;
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
