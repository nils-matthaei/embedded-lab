
type Chips = [u128; 8];

#[derive(Debug)]
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

        pub fn get_chip(&self, chip_index: usize) -> Result<i32, &str> {
                if chip_index > 1022 {
                        return Err("chip_index must be in range 0..1023");
                }

                let arr_index: usize = chip_index / 128;
                let inner_bit_index: usize = chip_index - arr_index*128;

                let sub_sequence: u128 = self.arr[arr_index];
                let chip: i32 = (sub_sequence >> inner_bit_index & 0x1) as i32; // precedence of >> stronger than that of &
                if chip == 1 {
                        return Ok(1);
                } else if chip == 0 {
                        return Ok( -1 );
                } else {
                        return Err("what in tarnation?");
                }
        }

        #[allow(dead_code)]
        pub fn print_bitwise(&self) {
                for &value in &self.arr {
                    // Print each u128 value in binary format
                    println!("{:0128b}", value);
                }
        }

        pub fn correlation_product_with_signal(&self, signal: &Vec<i32>, delta: usize) -> Result<i32, &str>{
                if signal.len() < 1023 { return Err("Signal must contain at least 1023 values."); }

                let shifted_signal: Vec<i32> = {
                        let mut signal_copy = signal.clone();
                        signal_copy.rotate_left(delta); // evil O(n) operation
                        signal_copy
                };

                let mut result: i32 = 0;

                for i in 0..1022 {
                        match self.get_chip(1022 - i) {
                            Ok(chip) => result += shifted_signal[i] * chip,
                            Err(msg) => return Err(msg)
                        }
                }

                return Ok(result);
        }

        pub fn cross_correlate_with_signal(&self, signal: &Vec<i32>) -> Option<(i32,usize)> {

                for delta in 0..1023 {

                        match self.correlation_product_with_signal(&signal, delta) {
                                Ok(product) => {
                                        if product.abs() < 1000 { continue; }
                                        if product < 0 {
                                                return Some((0,delta));
                                        } else {
                                            return Some((1, delta));
                                        }
                                }
                                Err(msg) => print!("Error: {}\n", msg)
                        }

                }

                return None;
        }
}
