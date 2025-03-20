use cdma::chipsequence::Chipsequence;

mod cdma;
fn main() {
    let mut chip_sequence = Chipsequence::new();
    chip_sequence.set_bit(5);

    println!(
        "Satellite 8 has sent bit 0 (delta = 72)
Satellite 9 has sent bit 1 (delta = 449)
Satellite 18 has sent bit 0 (delta = 345)
Satellite 22 has sent bit 1 (delta = 157)"
    );
}
