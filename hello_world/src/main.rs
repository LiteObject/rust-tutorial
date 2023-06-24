// https://doc.rust-lang.org/book/ch01-02-hello-world.html

/* 
> rustc main.rs
> .\main.exe
Hello, world!
*/

/*
The main function is special: it is always the first code that runs in every executable Rust program. 
Here, the first line declares a function named main that has no parameters and returns nothing. If there 
were parameters, they would go inside the parentheses ().
*/

#![allow(unused_variables)]

fn main() {
    // Rust style is to indent with four spaces, not a tab.
    let unused_var: u32 = 100; 
    println!("Hello, world!");

    // arrary
    let location: [f32; 2] = [1.0, 2.0];

    // tuple
    let my_tuple_location: (&str, f64, f64) = ("Dallas", 1.0, 2.0);
    println!("my_tuple_location: {:?}", my_tuple_location);
}