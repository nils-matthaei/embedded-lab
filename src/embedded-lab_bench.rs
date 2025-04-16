mod cdma;

use criterion::{criterion_group, criterion_main, Criterion};
use cdma::{chipsequence::Chipsequence, gold_codes::{GoldCodeGenerator, REGISTER_SUMS}};

use std::fs;

struct DecodedSignal {
    sattelite_id: usize,
    bit: i32,
    delta: usize
}

fn setup() {
    let sig_string = fs::read_to_string("/home/schasch/Documents/HKA/06Semester/Embedded_Labor/embedded-lab/signals/signal5").expect("Could not read file :(");

    let signal: Vec<i32> = sig_string
        .split(" ")
        .filter_map(|s| s.trim().parse::<i32>().ok())
        .collect();

    // generate chipsequences
    let mut chipsequences: Vec<Chipsequence> = Vec::new();
    for regsum in REGISTER_SUMS {
        let mut genr: GoldCodeGenerator = GoldCodeGenerator::new(regsum);
        chipsequences.push(genr.generate());
    }

    let mut decoded_signals: Vec<DecodedSignal> = Vec::new();
    for (index, sequence) in chipsequences.iter().enumerate() {
        if let Some(result) = sequence.cross_correlate_with_signal(&signal) {
            decoded_signals.push(DecodedSignal {
                sattelite_id: index + 1,
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("decode signal", |b| b.iter(|| setup()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
