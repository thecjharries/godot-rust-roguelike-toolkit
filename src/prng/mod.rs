// Copyright 2025 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

#[derive(Debug)]
pub struct Prng {
    seed: u64,
    rng: Pcg64,
}

impl Prng {
    pub fn new(seed: u64) -> Self {
        let rng = Pcg64::seed_from_u64(seed);
        Self { seed, rng }
    }

    pub fn roll(&mut self, sides: u64) -> u64 {
        self.rng.random_range(1..=sides)
    }
}

#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;

    #[test]
    fn test_new_prng() {
        let mut prng = Prng::new(0);
        assert_eq!(0, prng.seed);
        assert_eq!(5, prng.rng.random_range(1..6));
    }

    #[test]
    fn test_roll() {
        let mut prng = Prng::new(0);
        assert_eq!(1, prng.roll(1));
        assert_eq!(1, prng.roll(2));
        assert_eq!(3, prng.roll(3));
        assert_eq!(1, prng.roll(4));
        assert_eq!(5, prng.roll(5));
        assert_eq!(1, prng.roll(6));
    }
}
