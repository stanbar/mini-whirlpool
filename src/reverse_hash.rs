use rayon::prelude::*;
use std::time::Instant;

const CHARS: &[u8] =
    b"qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890!@#%^-_=+([{<)]}>";

const HASHES: &[(usize, [u8; 16])] = &[
    (
        2,
        [
            0xBD, 0x84, 0xE6, 0xFB, 0xC0, 0x6A, 0x36, 0x73, 0x5D, 0xBC, 0xBD, 0x54, 0x96, 0x31,
            0x7A, 0xB2,
        ],
    ),
    (
        3,
        [
            0x4C, 0xC4, 0x06, 0x23, 0x63, 0x55, 0xC4, 0xC3, 0x2E, 0x4D, 0xB5, 0x86, 0x77, 0x84,
            0x52, 0xA1,
        ],
    ),
    (
        4,
        [
            0x42, 0xD6, 0x05, 0x85, 0x44, 0xD9, 0xC0, 0x0B, 0x32, 0xB7, 0x33, 0x2E, 0x0C, 0xA7,
            0x95, 0x5A,
        ],
    ),
    (
        5,
        [
            0x01, 0x3E, 0x59, 0x9F, 0xB1, 0x4C, 0x10, 0xBA, 0x0C, 0xCF, 0x23, 0x26, 0x6F, 0x4B,
            0x4D, 0x0E,
        ],
    ),
    (
        6,
        [
            0x3C, 0x53, 0xFF, 0xDD, 0x7D, 0xBD, 0xF2, 0xA1, 0xE4, 0xD3, 0xC9, 0x8F, 0x55, 0x9E,
            0xC0, 0x7C,
        ],
    ),
];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let reverse_order = match args.get(1) {
        Some(x) => x == "--reverse",
        None => false,
    };

    HASHES
        .iter()
        .for_each(|x| brute_force(reverse_order, x.0, x.1));
}

fn brute_force(reverse_order: bool, chars_count: usize, expected: [u8; 16]) {
    let start = Instant::now();
    let mut chars = [0u8; 79];
    chars.copy_from_slice(&CHARS[..]);
    if reverse_order {
        chars.reverse();
    }

    let p = permutations(&chars[..], chars_count)
        .par_bridge()
        .find_any(|p| {
            if expected == whirlpool::core::hash(p.clone()) {
                true
            } else {
                false
            }
        });
    match p {
        None => panic!("Did not find any input"),
        Some(x) => {
            let duration = start.elapsed();
            println!(
                "Found the preimage for hash {:?}. It is {:?}. Took {:?}",
                expected,
                String::from_utf8(x.clone()),
                duration
            );
        }
    }
}

struct PermutationIterator<'a, T: 'a> {
    universe: &'a [T],
    size: usize,
    prev: Option<Vec<usize>>,
}

fn permutations<T>(universe: &[T], size: usize) -> PermutationIterator<T> {
    PermutationIterator {
        universe,
        size,
        prev: None,
    }
}

fn map<T>(values: &[T], ixs: &[usize]) -> Vec<T>
where
    T: Clone,
{
    ixs.iter().map(|&i| values[i].clone()).collect()
}

impl<'a, T> Iterator for PermutationIterator<'a, T>
where
    T: Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        let n = self.universe.len();

        if n == 0 {
            return None;
        }

        match self.prev {
            None => {
                let zeroes: Vec<usize> = std::iter::repeat(0).take(self.size).collect();
                let result = Some(map(self.universe, &zeroes[..]));
                self.prev = Some(zeroes);
                result
            }
            Some(ref mut indexes) => match indexes.iter().position(|&i| i + 1 < n) {
                None => None,
                Some(position) => {
                    for index in indexes.iter_mut().take(position) {
                        *index = 0;
                    }
                    indexes[position] += 1;
                    Some(map(self.universe, &indexes[..]))
                }
            },
        }
    }
}
