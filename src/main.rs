use cdma::{chipsequence::Chipsequence, gold_codes::{GoldCodeGenerator, REGISTER_SUMS}};

mod cdma;
fn main() {
        let regsum = REGISTER_SUMS[1];
        let mut gen: GoldCodeGenerator = GoldCodeGenerator::new(regsum);

        let seq: Chipsequence = gen.generate();

        seq.print_bitwise();
}
