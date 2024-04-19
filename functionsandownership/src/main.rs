fn main() {
    let s1 = String::from("hello");
    //takeownership(s1);
    //let s1 = takeandgiveback(s1);
    //println!("{s1}");

    //let x = 5;
    //makescopy(x);
    //println!("{x}");

    //let (s2, len) = calculate_length(s1);
    //println!("{s2}, {len}");

    //let s2 = s1.clone();
    //makestringcopy(s1.clone());
    //println!("{s1}");

    let mut s = String::from("Hello");
    s.push_str(", world");
    println!("{s}");
    



}


fn makescopy (x: i32) {
    
}

fn makestringcopy(s: String){

}

fn takeownership(s: String) {
    println!("\n taken taken {s} ");
}

fn takeandgiveback(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}