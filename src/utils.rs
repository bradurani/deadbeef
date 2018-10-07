extern crate rand;

use utils::rand::rngs::SmallRng;
use utils::rand::{Rng, SeedableRng};

pub fn choose_random<T>(vec: &Vec<T>) -> &T {
    seeded_rng().choose(vec).unwrap()
}

pub fn random() -> u8 {
    seeded_rng().gen()
}

fn seeded_rng() -> SmallRng {
    let seed = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let mut rng = SmallRng::from_seed(seed);
    rng
}

#[cfg(test)]
mod tests {
    //     use test::Bencher;
    use utils::*;

    #[test]
    fn test_choose_random() {
        let vec = vec![23];
        assert_eq!(*choose_random(&vec), 23);
    }

    #[test]
    fn repeated_random() {
        let n = random();
        let m = random();
        assert_eq!(n, m);
    }

    //     #[bench]
    //     fn bench_choose_random10(b: &mut Bencher) {
    //         let vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    //         b.iter(|| choose_random(&vec))
    //     }
}
