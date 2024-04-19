
fn main() {
    let mut v: Vec<i32> = Vec::new();
    for i in 1..10 {
        v.push(i);
    }
    v.clear();
    let w = v.is_empty();
    println!("{:?}", w);
    
}
