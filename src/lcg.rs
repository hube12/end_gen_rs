pub mod lcg_const {
    pub const A: u64 = 0x5DEECE66D;
    pub const C: u64 = 0xB;
}

// Constants used to reverse operations
pub mod lcg_const_extra {
    pub const INV_A: u64 = 0xdfe05bcb1365;
    pub const INV_A_1: u64 = 18698324575379;
    pub const INV__INV_A__1: u64 = 192407907957609;
}


pub const fn mask(n: u8) -> u64 {
    (1 << n) - 1
}

#[derive(Copy, Clone, Debug)]
pub struct Random {
    seed: u64,
}

impl Random {
    pub fn with_seed(s: u64) -> Random {
        let mut r = Random { seed: 0 };
        r.set_seed(s);
        r
    }

    pub fn with_raw_seed(s: u64) -> Random {
        let mut r = Random { seed: 0 };
        r.set_raw_seed(s);
        r
    }

    pub fn set_seed(&mut self, s: u64) {
        self.seed = s ^ lcg_const::A;
    }

    pub fn set_raw_seed(&mut self, s: u64) {
        self.seed = s;
    }

    pub fn get_seed(&self) -> u64 {
        (self.seed ^ lcg_const::A) & mask(48)
    }

    pub fn get_raw_seed(&self) -> u64 {
        self.seed & mask(48)
    }

    pub fn next(&mut self, bits: u8) -> i32 {
        self.seed = Self::next_state(self.seed);
        ((self.seed & mask(48)) >> (48 - bits)) as i32
    }

    // s * A + C
    pub fn next_state(s: u64) -> u64 {
        s.wrapping_mul(lcg_const::A).wrapping_add(lcg_const::C)
    }

    // Returns the same as the last call to next
    pub fn last_next(&self, bits: u8) -> i32 {
        ((self.seed & mask(48)) >> (48 - bits)) as i32
    }

    pub fn next_int(&mut self) -> i32 {
        self.next(32)
    }

    pub fn next_int_n(&mut self, n: i32) -> i32 {
        if n == 10 {
            return self.next_int_n_10();
        }
        if !(n > 0) {
            panic!("In JavaRng::next_int_n, n should be greater than zero.");
        }
        // If n is a power of 2
        if (n & -n) == n {
            return (((n as i64) * (self.next(31) as i64)) >> 31) as i32;
        }

        let mut bits;
        let mut val;
        loop {
            bits = self.next(31);
            val = bits % n;
            // Check for modulo bias
            if bits.wrapping_sub(val).wrapping_add(n - 1) >= 0 {
                break;
            }
        }

        val
    }

    pub fn next_int_n_10(&mut self) -> i32 {
        let mut bits;
        loop {
            bits = self.next(31);
            // Check for modulo bias
            let limit = (1u32 << 31) / 10 * 10; // last multiple of 10 < 2^31
            if bits < limit as i32 {
                break;
            }
        }
        bits % 10
    }

    pub fn next_long(&mut self) -> i64 {
        ((self.next_int() as i64) << 32) + (self.next_int() as i64)
    }

    pub fn next_boolean(&mut self) -> bool {
        self.next(1) != 0
    }

    pub fn next_float(&mut self) -> f32 {
        self.next(24) as f32 / (1 << 24) as f32
    }

    pub fn next_double(&mut self) -> f64 {
        let hi = (self.next(26) as i64) << 27;
        let lo = self.next(27) as i64;

        (hi + lo) as f64 / ((1u64 << 53) as f64)
    }

    // The inverse of next()
    pub fn previous(&mut self) {
        //self.seed = (self.seed.wrapping_sub(lcg_const::C)).wrapping_mul(lcg_const_extra::INV_A);
        self.seed = Self::previous_state(self.seed);
    }

    // The previous internal state of the prng, not the seed
    pub fn previous_state(s: u64) -> u64 {
        (s.wrapping_sub(lcg_const::C)).wrapping_mul(lcg_const_extra::INV_A) & mask(48)
    }
}