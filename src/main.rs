mod cdma;

use cdma::{chipsequence::{self, Chipsequence}, gold_codes::{GoldCodeGenerator, REGISTER_SUMS}};
use std::fs;

struct DecodedSignal {
        sattelite_id: usize,
        bit: i32,
        delta: usize
}
fn main() {
        // read signal-file and write values to vector
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            eprintln!("Usage: {} <path_to_signal_file>", args[0]);
            std::process::exit(1);
        }
        let sig_string = fs::read_to_string(&args[1])
            .expect("Could not read file :(");

        let signal: Vec<i32> = sig_string
            .split(" ")
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        // generate chipsequences
        let mut chipsequences: Vec<Chipsequence> = Vec::new();
        for regsum in REGISTER_SUMS {
                let mut gen: GoldCodeGenerator = GoldCodeGenerator::new(regsum);
                chipsequences.push(gen.generate());

        }

        // decode signal
        let mut decoded_signals: Vec<DecodedSignal> = Vec::new();
        for (index,sequence) in chipsequences.iter().enumerate() {
                if let Some(result) = sequence.cross_correlate_with_signal(&signal) {
                                decoded_signals.push(DecodedSignal {
                                                sattelite_id: index + 1,
                                                bit: result.0,
                                                delta: result.1
                                });
                }
        }

        // print results
        for dsig in decoded_signals {
                println!("Sattelite {} has sent bit {} (delta = {})", dsig.sattelite_id, dsig.bit, dsig.delta);
        }
}
