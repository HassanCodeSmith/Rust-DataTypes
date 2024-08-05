#![allow(unused)]
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
    let monghts = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let b = [10; 10];
    // println!("{:?}", b);

    println!("JAN: {}", monghts[0]);
    println!("FEB: {}", monghts[1]);
}
