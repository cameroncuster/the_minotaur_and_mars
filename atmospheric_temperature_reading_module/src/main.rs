use rand::Rng;
use rayon::prelude::*;
use std::sync::atomic::AtomicUsize;

// returns a random number from -100F to 70F
fn read_temp() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-100..=70)
}

fn main() {
    const ITERATIONS: usize = 10;
    const THREADS: usize = 8;
    const DATA_POINTS: usize = 60;

    for iter in 0..ITERATIONS {
        let mut data = vec![vec![0; DATA_POINTS]; THREADS];

        let cnt = AtomicUsize::new(0);

        data.par_iter_mut().enumerate().for_each(|(i, data)| {
            for j in 0..DATA_POINTS {
                data[j] = read_temp();

                cnt.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                while cnt.load(std::sync::atomic::Ordering::Relaxed) < (i + 1) * THREADS {
                    std::thread::yield_now();
                }
            }
        });

        let mut nums = data.iter().flatten().copied().collect::<Vec<_>>();
        nums.sort_unstable();

        let smallest = nums[0..5].to_vec();
        let largest = nums[nums.len() - 5..].to_vec();

        let mut max_diff = 0;
        let mut idx_of_max_diff = 0;
        for data_row in data.iter() {
            for start in 0..DATA_POINTS - 10 {
                let diff = data_row[start..start + 10].iter().max().unwrap()
                    - data_row[start..start + 10].iter().min().unwrap();
                if diff > max_diff {
                    max_diff = diff;
                    idx_of_max_diff = start;
                }
            }
        }

        println!("Report for hour #{}", iter);
        println!("======");
        println!("Largest: {:?}", largest);
        println!("Smallest: {:?}", smallest);
        println!(
            "10 minute interval of time with largest temperature difference: [{:?}, {:?})",
            idx_of_max_diff,
            idx_of_max_diff + 10
        );
        println!();
    }
}
