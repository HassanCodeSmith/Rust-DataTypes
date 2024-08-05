#![allow(unused)]

use std::io;
use colored::*;
fn main() {
    /* __________ Tuple (Compund Data Type) __________ */
    // create tuple
    // let tup: (i32, &str, f64) = (500, "Hassan", 3.4);

    // destructure tuple
    // let (x, name, cpga) = tup;

    // println!("{x}");
    // println!("{name}");
    // println!("{cpga}");

    // println!("{}", tup.0);
    // println!("{}", tup.1);
    // println!("{}", tup.2);

    /* __________ Array (Compund Data Type) __________ */
    // let monghts = [
    //     "January",
    //     "February",
    //     "March",
    //     "April",
    //     "May",
    //     "June",
    //     "July",
    //     "August",
    //     "September",
    //     "October",
    //     "November",
    //     "December",
    // ];

    // let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let b = [10; 10];
    // println!("{:?}", b);

    // println!("JAN: {}", monghts[0]);
    // println!("FEB: {}", monghts[1]);

    // __________ Panic Situation in Program
    let a = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("{}", "Please enter an array index.".blue());
    io::stdin().read_line(&mut index).expect("Failed to read line.");

    let index: usize = index.trim().parse().expect("Index entered was not a number.");

    if index < a.len() {
        let element = a[index];
        println!("{}", format!("The value of element at index {index} is: {element}").green());
    } else {
        eprintln!(
            "{}",
            format!("Index out of bounds. Plese enter a number between 0 and {}", a.len() - 1).red()
        );
    }
}
