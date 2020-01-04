use std::{iter, sync::Barrier, time};

use crossbeam_utils::{thread::scope, CachePadded};

fn main() {
    let options = Options {
        n_threads: 32,
        n_locks: 1000,
        n_rounds: 10_000,
    };
    println!("{:#?}\n", options);

    bench::<mutexes::Std>(&options);
    bench::<mutexes::ParkingLot>(&options);
    bench::<mutexes::Spin>(&options);
}

fn bench<M: Mutex>(options: &Options) {
    let time = run_bench::<M>(options);
    println!("{:<20} {:?}", M::LABEL, time)
}

#[derive(Debug)]
struct Options {
    n_threads: u32,
    n_locks: u32,
    n_rounds: u32,
}

fn random_numbers(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;
    iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

trait Mutex: Sync + Send + Default {
    const LABEL: &'static str;
    fn with_lock(&self, f: impl FnOnce(&mut u32));
}

fn run_bench<M: Mutex>(options: &Options) -> time::Duration {
    let locks = &(0..options.n_locks)
        .map(|_| CachePadded::new(M::default()))
        .collect::<Vec<_>>();

    let start_barrier = &Barrier::new(options.n_threads as usize + 1);
    let end_barrier = &Barrier::new(options.n_threads as usize + 1);

    let elapsed = scope(|scope| {
        let thread_seeds = random_numbers(0x6F4A955E).scan(0x9BA2BF27, |state, n| {
            *state ^= n;
            Some(*state)
        });
        for thread_seed in thread_seeds.take(options.n_threads as usize) {
            scope.spawn(move |_| {
                start_barrier.wait();
                let indexes = random_numbers(thread_seed)
                    .map(|it| it % options.n_locks)
                    .map(|it| it as usize)
                    .take(options.n_rounds as usize);
                for idx in indexes {
                    locks[idx].with_lock(|cnt| *cnt += 1);
                }
                end_barrier.wait();
            });
        }

        std::thread::sleep(time::Duration::from_millis(100));
        start_barrier.wait();
        let start = time::Instant::now();
        end_barrier.wait();
        let elapsed = start.elapsed();

        let mut total = 0;
        for lock in locks.iter() {
            lock.with_lock(|cnt| total += *cnt);
        }
        assert_eq!(total, options.n_threads * options.n_rounds);

        elapsed
    })
    .unwrap();
    elapsed
}

mod mutexes {
    use super::Mutex;

    pub(crate) type Std = std::sync::Mutex<u32>;
    impl Mutex for Std {
        const LABEL: &'static str = "std::sync::Mutex";
        fn with_lock(&self, f: impl FnOnce(&mut u32)) {
            let mut guard = self.lock().unwrap();
            f(&mut guard)
        }
    }

    pub(crate) type ParkingLot = parking_lot::Mutex<u32>;
    impl Mutex for ParkingLot {
        const LABEL: &'static str = "parking_lot::Mutex";
        fn with_lock(&self, f: impl FnOnce(&mut u32)) {
            let mut guard = self.lock();
            f(&mut guard)
        }
    }

    pub(crate) type Spin = spin::Mutex<u32>;
    impl Mutex for Spin {
        const LABEL: &'static str = "spin::Mutex";
        fn with_lock(&self, f: impl FnOnce(&mut u32)) {
            let mut guard = self.lock();
            f(&mut guard)
        }
    }
}
