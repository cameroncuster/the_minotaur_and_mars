use crossbeam_queue::ArrayQueue;
use rand::seq::SliceRandom;
use std::sync::Arc;

mod concurrent_ordered_linked_list;
use concurrent_ordered_linked_list::ConcurrentOrderedLinkedList;

fn main() {
    const THREADS: usize = 4;
    const GIFTS: usize = 500_000;

    let unordered_bag = Arc::new(ArrayQueue::new(GIFTS));

    let mut gifts = (0..GIFTS).collect::<Vec<_>>();
    gifts.shuffle(&mut rand::thread_rng());
    gifts.iter().for_each(|gift| {
        unordered_bag.push(*gift).unwrap();
    });

    let ordered_bag = Arc::new(ConcurrentOrderedLinkedList::new());

    let mut threads = Vec::with_capacity(THREADS);
    for id in 0..THREADS {
        let unordered_bag = unordered_bag.clone();
        let ordered_bag = ordered_bag.clone();

        threads.push(std::thread::spawn(move || {
            while !unordered_bag.is_empty() || !ordered_bag.is_empty() {
                // we never grab a random gift from the bag... (3)

                if let Some(gift) = unordered_bag.pop() {
                    ordered_bag.insert(gift);
                }

                if let Some(gift) = ordered_bag.pop() {
                    println!("Servent {} is thankful for the gift with tag #{}", id, gift);
                }
            }
        }));
    }

    threads.into_iter().for_each(|thread| {
        thread.join().unwrap();
    });
}
