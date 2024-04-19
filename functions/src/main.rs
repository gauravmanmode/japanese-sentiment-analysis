fn main() {
    let y = 5;
    let mut x = y;
    x = plusone(x);
    println!("Hello, world!{x}{y}");

    measurement(5, 'h');
}

fn measurement(value: i32, label: char)
{
    println!("value of x is {value}{label}");
}

fn plusone(x: i32) -> i32 {
    x+1
}