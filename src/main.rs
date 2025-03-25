use cdma::{chipsequence::Chipsequence, gold_codes::{GoldCodeGenerator, REGISTER_SUMS}};
use std::fs;

mod cdma;
fn main() {
        let regsum = REGISTER_SUMS[1];
        let mut gen: GoldCodeGenerator = GoldCodeGenerator::new(regsum);

        let seq: Chipsequence = gen.generate();

        // seq.print_bitwise();

        let sig_string = fs::read_to_string("./signal")
        .expect("Could not read file :(");

        let signal: Vec<i32> = sig_string
            .split(" ")
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        for delta in 0..1024 {
                let res = seq.correlation_product_with_signal(&signal, delta);

                match res {
                    Ok(product) => print!("correlation product with delta of {} is {}\n", delta, product),
                    Err(msg) => print!("Error: {}\n", msg)
                }
        }
}
