
fn main() {
    // Integer addition & subtraction
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("3 << 6 is {}", 3u32 << 6);
    println!("0x90 >> 1 is 0x{:x}", 0x90u32 >> 1);

    // Improve readability with underscores
    println!("One million is written as {}", 1_000_000u32);
}