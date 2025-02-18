use rand::Rng;
use std::cmp::Ordering;
use std::io; //  io library we are introducing //ranf lib

fn main() {
    // let mut x = 8; // use mut to declare a mutable variable
    // let y = 9; // non mutable
    // println!("Guess the number!");
    // x = 9;
    // print!("x : {x}");

    //contants are by default immutable and are to be declared with the keyword const and with the datatype
    const MAX: u32 = 100; //u32 === uint32

    //shadowing

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
