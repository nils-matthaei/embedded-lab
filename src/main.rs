mod cdma;

use cdma::{
    chipsequence::Chipsequence,
    gold_codes::{GoldCodeGenerator, REGISTER_SUMS},
};
use std::fs;

struct DecodedSignal {
    sattelite_id: usize,
    bit: i32,
    delta: usize,
}

fn main() {
    // read signal-file and write values to vector
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_signal_file>", args[0]);
        std::process::exit(1);
    }
    let sig_string = fs::read_to_string(&args[1]).expect("Could not read file :(");

    let signal: [i32; 2046] = {
        let sig_array: [i32; 1023] = sig_string
            .split(" ")
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect::<Vec<_>>()
            .try_into()
            .expect("Signal length must be 1023");

        // Concatenate the signal array with itself to avoid slow modulus operations
        let mut concatenated = [0; 2046];
        concatenated[..1023].copy_from_slice(&sig_array);
        concatenated[1023..].copy_from_slice(&sig_array);
        concatenated
    };

    // generate chipsequences
    let chipsequences: [Chipsequence; REGISTER_SUMS.len()] = std::array::from_fn(|i| {
        let mut gen = GoldCodeGenerator::new(REGISTER_SUMS[i]);
        gen.generate()
    });

    // decode signal
    let mut decoded_signals: Vec<DecodedSignal> = Vec::new();
    for i in 0..chipsequences.len() {
        if let Some(result) = chipsequences[i].cross_correlate_with_signal(&signal) {
            decoded_signals.push(DecodedSignal {
                sattelite_id: i + 1,
                bit: result.0,
                delta: result.1,
            });
        }
    }

    // print results
    for dsig in decoded_signals {
        println!(
            "Sattelite {} has sent bit {} (delta = {})",
            dsig.sattelite_id, dsig.bit, dsig.delta
        );
    }
}
