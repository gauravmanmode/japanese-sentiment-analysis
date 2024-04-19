fn main() {
    let mut s = String::from("hello");
    //change(&mut s);
    let r1 = &s;
    let r2 = &mut s;
    println!("{}", *r1);


    let unknown = dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", hello");
}

fn dangle () -> &String {
    let s= String::from("hello");
    &s
}
