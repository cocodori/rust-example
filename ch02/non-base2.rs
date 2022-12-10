fn main() {
    let three = 0b11;
    let thirsty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {} {} {}", three, thirsty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirsty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirsty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirsty, three_hundred);
}