// Regular comment
/* Block comment */

/*
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

*/

use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

fn main() {
    /*
    println!("Hello World!");
    println!("I'm a Rustacean!");
    */

    /*
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    

    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");
    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);
    println!("Base 16 (hexadecimal): {:X}", 69420);
    */
    
    /*
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=2);
    println!("{number:0>width$}", number=3, width=6);
    println!("My name is {0}, {0} {1}", "James", "Bond");
    */

    /*
    #[allow(dead_code)]
    struct Structure(i32);
    println!("This struct `{}` won't print...", Structure(3));
    */

    /*
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    */

    /*
    struct UnPrintable(i32);
    println!("{}", UnPrintable(5));
    
    #[derive(Debug)]
    struct DebugPrintable(i32);
    println!("{:?}", DebugPrintable(9));
    */

    /*
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slator", "Christian", actor="actor's");
    println!("Now {:?} will print!", Structure(6));
    println!("Now {:?} will print!", Deep(Structure(7)));
    */

    /*
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
    */

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small is {small}", small=small_range, big=big_range);
    // println!("The big range is {:?} and the small is {:?}", big_range, small_range);

}