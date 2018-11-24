use rand::rngs::SmallRng;
#[allow(unused_imports)]
use rand::{Rng, RngCore, SeedableRng};
use std::collections::HashMap;
use std::fs::*;
use std::hash::{BuildHasherDefault, Hash};
use std::io::prelude::*;
use twox_hash::XxHash;

pub fn choose_random<'a, T, R: Rng>(rng: &mut R, vec: &'a Vec<T>) -> &'a T {
    debug_assert!(vec.len() > 0);
    rng.next_u32();
    rng.choose(&vec).expect("no random to choose")
}

pub fn seeded_rng(rng_seed: u8) -> SmallRng {
    let seeds = [
        rng_seed, 2, rng_seed, 3, rng_seed, 4, rng_seed, 5, rng_seed, 6, rng_seed, 7, rng_seed, 8,
        rng_seed, 9,
    ];
    SmallRng::from_seed(seeds)
}

// Creates a HashMap that will be iterated in the same order for every program run (NOT the insert
// order however) if given the same elements. Allows deterministic execution and tests
pub fn deterministic_hash_map<K: Hash + Eq, V>() -> HashMap<K, V, BuildHasherDefault<XxHash>> {
    let hash: HashMap<K, V, BuildHasherDefault<XxHash>> = Default::default();
    hash
}

pub fn file_to_string(filename: &'static str) -> String {
    let mut file = File::open(filename).expect("could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("could not read file to string");
    contents
}

#[cfg(test)]
mod tests {
    //     use test::Bencher;
    use utils::*;

    #[test]
    fn test_choose_random_1_element() {
        let mut rng = seeded_rng(1);
        let vec = vec![23];
        assert_eq!(*choose_random(&mut rng, &vec), 23);
    }

    #[test]
    fn test_choose_random() {
        let mut rng = seeded_rng(37);
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(2, *choose_random(&mut rng, &vec));
        assert_eq!(4, *choose_random(&mut rng, &vec));
    }

    #[test]
    fn repeated_random() {
        let mut rng = seeded_rng(1);
        let a: u32 = rng.next_u32();
        let b: u32 = rng.next_u32();
        assert_eq!(35328554, a);
        assert_eq!(255278948, b);
    }

    //     #[bench]
    //     fn bench_choose_random10(b: &mut Bencher) {
    //         let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    //         b.iter(|| choose_random(&vec))
    //     }
}
