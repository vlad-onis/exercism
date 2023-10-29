use std::{cmp::min, collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();

    if input.len() == 0 {
        return counts;
    }

    let input = input.join("");

    if input.len() == 0 {
        return counts;
    }

    let real_worker_count = min(input.len(), worker_count);
    let mut thread_pool = Vec::with_capacity(worker_count);
    let mut work_length = (input.len() / real_worker_count).max(1);
    if work_length * real_worker_count < input.len() {
        work_length += 1;
    }

    let mut churn = input.chars();

    for _ in 0..real_worker_count {
        let chunk = churn.by_ref().take(work_length).collect::<String>();
        let my_thread = thread::spawn(move || {
            let mut answer: HashMap<char, usize> = HashMap::new();

            chunk
                .chars()
                .filter(|chr| chr.is_alphabetic())
                .for_each(|chr| {
                    *answer.entry(chr.to_ascii_lowercase()).or_default() += 1;
                });
            answer
        });

        thread_pool.push(my_thread);
    }

    for thd in thread_pool {
        let answer = thd.join().unwrap();
        for (key, val) in answer {
            *counts.entry(key).or_default() += val;
        }
    }

    counts
}
