use keccak_hash::keccak256;

/// Seeded ChaCha RNG
///
/// RNG is seeded by variable length bytes. These bytes are hashed using keccak256
/// and that hash is used as the seed to the ChaCha RNG. The ChaCha counter is initialized
/// at 0.
pub struct ChaChaRng {
    state: [u32; 16],
    seed_state: [u32; 16],
    pub round_count: usize, // 8, 12, 20, etc
    buffer: Vec<u8>,
    buffer_offset: usize,
}

impl ChaChaRng {
    pub fn from_seed(seed: &[u8]) -> Self {
        let mut bytes = seed.to_vec();
        keccak256(&mut bytes);
        let mut state = [0u32; 16];
        // nothing up my sleeve values
        state[0] = 1634760805;
        state[1] = 857760878;
        state[2] = 2036477234;
        state[3] = 1797285236;
        // key values
        for i in 0..8 {
            state[4 + i] = u32::from_le_bytes([
                bytes[i * 4],
                bytes[i * 4 + 1],
                bytes[i * 4 + 2],
                bytes[i * 4 + 3],
            ]);
        }
        // counter values
        state[12] = 0;
        state[13] = 0;
        // nonce values
        state[14] = 0;
        state[15] = 0;
        ChaChaRng {
            state,
            seed_state: state,
            round_count: 12,
            buffer: vec![],
            buffer_offset: 0,
        }
    }

    /// get len bytes from the chacha stream
    pub fn get_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; len];
        for i in 0..len {
            bytes[i] = self.get_byte();
        }
        bytes
    }

    pub fn get_byte(&mut self) -> u8 {
        if self.buffer.len() == 0 {
            self.round();
            return self.get_byte();
        }
        if self.buffer_offset == self.buffer.len() {
            self.buffer = vec![];
            self.buffer_offset = 0;
            self.round();
            return self.get_byte();
        }
        let byte = self.buffer[self.buffer_offset];
        self.buffer_offset += 1;
        byte
    }

    /// Execute a single round of chacha
    ///
    /// The resulting state is the random number
    fn round(&mut self) {
        #[cfg(debug_assertions)]
        assert_eq!(self.round_count % 2, 0, "round count must be even");
        for _ in 0..self.round_count / 2 {
            // odd rounds
            self.quarter_round([0, 4, 8, 12]);
            self.quarter_round([1, 5, 9, 13]);
            self.quarter_round([2, 6, 10, 14]);
            self.quarter_round([3, 7, 11, 15]);
            // even rounds
            self.quarter_round([0, 5, 10, 15]);
            self.quarter_round([1, 6, 11, 12]);
            self.quarter_round([2, 7, 8, 13]);
            self.quarter_round([3, 4, 9, 14]);
        }
        for i in 0..16 {
            self.buffer.append(
                &mut self.seed_state[i]
                    .wrapping_add(self.state[i])
                    .to_le_bytes()
                    .to_vec(),
            );
        }
    }

    fn quarter_round(&mut self, indices: [usize; 4]) {
        // a += b; d ^= a; d <<<= 16;
        // c += d; b ^= c; b <<<= 12;
        // a += b; d ^= a; d <<<=  8;
        // c += d; b ^= c; b <<<=  7;
        let mut a = self.state[indices[0]];
        let mut b = self.state[indices[1]];
        let mut c = self.state[indices[2]];
        let mut d = self.state[indices[3]];

        a = a.wrapping_add(b);
        d = (d ^ a).rotate_left(16);

        c = c.wrapping_add(d);
        b = (b ^ c).rotate_left(12);

        a = a.wrapping_add(b);
        d = (a ^ d).rotate_left(8);

        c = c.wrapping_add(d);
        b = (b ^ c).rotate_left(7);

        self.state[indices[0]] = a;
        self.state[indices[1]] = b;
        self.state[indices[2]] = c;
        self.state[indices[3]] = d;
    }
}

impl rand::RngCore for ChaChaRng {
    fn next_u32(&mut self) -> u32 {
        let mut b = [0u8; 4];
        for i in 0..4 {
            b[i] = self.get_byte();
        }
        u32::from_le_bytes(b)
    }

    fn next_u64(&mut self) -> u64 {
        let mut b = [0u8; 8];
        for i in 0..8 {
            b[i] = self.get_byte();
        }
        u64::from_le_bytes(b)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for i in 0..dest.len() {
            dest[i] = self.get_byte();
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chacha_rng() {
        let seed = [0u8; 32];
        let mut rng = ChaChaRng::from_seed(&seed);
        let sample = rng.get_bytes(33);
        println!("{:?}", sample);
    }
}
