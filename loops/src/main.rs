fn main() {


    'loop1: for a in 1..3 {
        println!("{a}");
        break 'loop1;
    }

    let cond = true;
    let number = if cond {3 } else {7} ;
    println!("{number}");

    
}