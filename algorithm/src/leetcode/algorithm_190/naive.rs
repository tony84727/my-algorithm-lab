pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        fn get_bits(bytes: [u8; 4], i: usize) -> u8 {
            let byte_index = i / 8;
            let bit_index = i % 8;
            if bytes[byte_index] & (1 << bit_index) != 0 {
                1
            } else {
                0
            }
        }
        let bytes = x.to_ne_bytes();
        let mut output = 0_u32;
        for bit in 0..32 {
            output |= (get_bits(bytes, 31 - bit) as u32) << bit;
        }
        output
    }
}
