// const THREE_MINUTES_IN_SECONDS: u32 = 3 * 60;
// const SHRT: u32 = THREE_MINUTES_IN_SECONDS;
// fn main() {
//     let x = 5;

//     {
//         let x = x + 1;
//         println!("The value of inner x is {x}");
//     }

//     println!("The value of x is {x}");
//     println!("{SHRT}");

//     let spaces = "     ";
//     let spaces: u32 = spaces.len();

//     //println!("{spaces} {}", spaces.len());
//     println!("{}", spaces);

// }

// fn main() {
//     let tup = (3,4,5);
//     let (x, y, z) = tup;
//     let unit: () = ();
//     println!(" {x}");
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}