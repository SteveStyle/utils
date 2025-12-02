mod bit_array;

use bit_array::BitArray;

fn main() {
    println!("Hello, world!  fredfred");

    match test_bit_array() {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

fn test_bit_array() -> bit_array::Result<()> {
    let mut ba = BitArray::<1000>::default();

    ba.set_bit_repeating(1, 3)?;
    dbg!(ba);
    Ok(())
}
