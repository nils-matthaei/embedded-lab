use std::collections::VecDeque;

use super::chipsequence::Chipsequence;

// Constants
pub const REGISTER_SUMS: [(usize,usize); 24] = [
        (2,6),
        (3,7),
        (4,8),
        (5,9),
        (1,9),
        (2,10),
        (1,8),
        (2,9),
        (3,10),
        (2,3),
        (3,4),
        (5,6),
        (6,7),
        (7,8),
        (8,9),
        (9,10),
        (1,4),
        (2,5),
        (3,6),
        (4,7),
        (5,8),
        (6,9),
        (1,3),
        (4,6)
];

pub struct GoldCodeGenerator {
        register_top: VecDeque<u8>,
        register_bottom: VecDeque<u8>,
        register_sum: (usize,usize)
}

impl GoldCodeGenerator {
        pub fn new(register_sum: (usize, usize)) -> Self {
                Self {
                        register_top: VecDeque::from_iter(vec![1;10]),
                        register_bottom: VecDeque::from_iter(vec![1;10]),
                        register_sum
                }
        }

        fn shift_top_register(&mut self) -> u8 {
                let new: u8 = self.register_top[2] ^ self.register_top[9];
                let out: u8 = self.register_top.pop_back().unwrap();
                self.register_top.push_front(new);

                return out;
        }

        fn shift_bottom_register(&mut self) -> u8 {
                let new: u8 = self.register_bottom[1] ^ self.register_bottom[2] ^ self.register_bottom[5] ^ self.register_bottom[7] ^ self.register_bottom[8] ^ self.register_bottom[9];
                let out: u8 = self.register_bottom[self.register_sum.0 - 1] ^ self.register_bottom[self.register_sum.1 - 1];
                self.register_bottom.pop_back();
                self.register_bottom.push_front(new);

                return out;
        }

        pub fn generate(&mut self) -> Chipsequence {
                let mut sequence: Chipsequence = Chipsequence::new();

                for i in 0..1022 {
                        let out_top: u8 = self.shift_top_register();
                        let out_bot: u8 = self.shift_bottom_register();

                        let chip: u8 = out_top ^ out_bot;

                        if chip == 1 {
                                sequence.set_chip( 1022 - i);
                        }
                 }

                return sequence;
        }
}
