
type Chips = [u128; 8];
pub struct Chipsequence {
        arr : Chips,
}

impl Chipsequence {
        fn new() -> Self {
                Self { arr: [0, 0, 0, 0, 0, 0, 0, 0] }
        }

        // technically this flips a bit but we will only use it once per bit to set it
        fn set_bit(&self, bit_index: usize) {
                if bit_index > 1022 {
                        return;
                }
                let arr_index: usize = bit_index / 128;
                let inner_bit_index: usize = bit_index - arr_index*128;
                let mask: u128 = 1 << inner_bit_index;
                self.arr[arr_index] & mask;
        }
}
