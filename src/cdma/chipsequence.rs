
type Chips = [u128; 8];
pub struct Chipsequence {
        arr : Chips,
}

impl Chipsequence {
        pub fn new() -> Self {
                Self { arr: [0, 0, 0, 0, 0, 0, 0, 0] }
        }

        pub fn set_chip(&mut self, bit_index: usize) {
                if bit_index > 1022 {
                        return;
                }
                let arr_index: usize = bit_index / 128;
                let inner_bit_index: usize = bit_index - arr_index*128;

                self.arr[arr_index] |= 1 << inner_bit_index;
        }

        pub fn print_bitwise(&self) {
                for &value in &self.arr {
                    // Print each u128 value in binary format
                    println!("{:0128b}", value);
                }
        }
}
