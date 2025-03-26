use cdma::{chipsequence::Chipsequence, gold_codes::{GoldCodeGenerator, REGISTER_SUMS}};
use std::fs;
use std::collections::HashMap;

mod cdma;
fn main() {
        // let regsum = REGISTER_SUMS[1];
        // let mut gen: GoldCodeGenerator = GoldCodeGenerator::new(regsum);
        // let seq: Chipsequence = gen.generate();

        // seq.print_bitwise();

        let sig_string = fs::read_to_string("./signal")
        .expect("Could not read file :(");

        let signal: Vec<i32> = sig_string
            .split(" ")
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        let mut prod_map: HashMap<i32, i32> = HashMap::new();

        for regsum in REGISTER_SUMS {
                let mut gen: GoldCodeGenerator = GoldCodeGenerator::new(regsum);
                let seq: Chipsequence = gen.generate();
                for delta in 0..1023 {
                        let res = seq.correlation_product_with_signal(&signal, delta);

                        match res {
                            Ok(product) => {
                                *prod_map.entry(product).or_insert(0) += 1;
                            }
                            Err(msg) => print!("Error: {}\n", msg)
                        }
                }
        }
        for (key, value) in &prod_map {
                println!("Correlation Product: {}, Count: {}", key, value);
        }
}
