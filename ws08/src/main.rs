use rand::Rng;

use std::time::Instant;

struct ParallelIterator {
    iter: Vec<i32>,
    num_threads: usize,
}

impl ParallelIterator {
    fn find(&self, search_for: i32) -> Option<usize> {
        let res = std::thread::scope(|scope| {
            let chunk_size = self.iter.len() / self.num_threads;
            let mut handles = vec![];
            for (cno, chunk) in self.iter.chunks(chunk_size).into_iter().enumerate() {
                let handle = scope.spawn(move || {
                    for (i, value) in chunk.iter().enumerate() {
                        if *value == search_for {
                            return Some(cno * chunk_size + i);
                        }
                    }
                    None
                });
                handles.push(handle);
            }

            for handle in handles {
                let result = handle.join().expect("thread paniced");
                if let Some(idx) = result {
                    return Some(idx);
                }
            }

            None
        });
        res
    }

    fn find_all(&self, search_for: i32) -> Vec<usize> {
        std::thread::scope(|scope| {
            let chunk_size = self.iter.len() / self.num_threads;
            let mut handles = vec![];
            for (cno, chunk) in self.iter.chunks(chunk_size).into_iter().enumerate() {
                let handle = scope.spawn(move || {
                    let mut positions = vec![];
                    for (i, value) in chunk.iter().enumerate() {
                        if *value == search_for {
                            positions.push(cno * chunk_size + i);
                        }
                    }
                    positions
                });
                handles.push(handle);
            }

            let mut indices = vec![];
            for handle in handles {
                let result = handle.join().expect("thread paniced");
                for r in result {
                    indices.push(r);
                }
            }
            indices
        })
    }

    fn map(&self /*you will need to add the argument here*/) -> Vec<i32> {
        // TODO
        vec![]
    }
}

trait IntoParIter {
    fn into_par_iter(self) -> ParallelIterator;
}

impl IntoParIter for Vec<i32> {
    fn into_par_iter(self) -> ParallelIterator {
        ParallelIterator {
            iter: self,
            num_threads: 9,
        }
    }
}

fn time<T>(test_name: &str, data: T, f: impl Fn(T) -> ()) {
    let before = Instant::now();
    f(data);
    println!("Test {test_name}: {:.2?}", before.elapsed());
}

fn main() {
    let test_length = 1_000_000;
    let find_index = 750_001;

    let mut nums: Vec<i32> = vec![0, 1].repeat(test_length / 2);
    nums[find_index] = 2;
    nums[find_index + 100] = 2;
    nums[find_index + 1_000] = 2;

    // time("find_normal", nums.clone(), |nums| {
    //     let mut iter = nums.into_iter();
    //     assert_eq!(iter.position(|i| i == 2), Some(find_index));
    // });

    // time("find_parallel", nums.clone(), |nums| {
    //     let iter = nums.into_par_iter();
    //     assert_eq!(iter.find(2), Some(find_index));
    // });

    time("find_all_normal", nums.clone(), |nums| {
        let single_threaded = nums
            .into_iter()
            .enumerate()
            .filter_map(|(idx, value)| if value == 2 { Some(idx) } else { None })
            .collect::<Vec<usize>>();

        assert_eq!(
            single_threaded,
            Vec::<usize>::from([find_index, find_index + 100, find_index + 1_000])
        );
    });

    time("find_parallel", nums.clone(), |nums| {
        let iter = nums.into_par_iter();
        assert_eq!(
            iter.find_all(2),
            Vec::<usize>::from([find_index, find_index + 100, find_index + 1_000])
        );
    });
}
