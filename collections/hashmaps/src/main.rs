use std::io;
// use std::collections::HashMap;

// fn main() {
//     let mut scores = HashMap::new();
//     let mut s = String::from("hello");
//     scores.insert(String::from("Blue"), 10);
//     scores.insert(s, 9);

    
//     let name = "Blue";
//     let score = dbg!(scores.get(name).copied());
//     dbg!(&scores);

//     for (key, value) in &scores {
//         dbg!(key);
//     }
//     //dbg!(s);
// }

fn main() {
    let v = vec![1,2,3]
    ;
    let mut s = String::new();
    io::stdin().read_line(&mut s);
    let s: usize = match s.trim().parse() {
        Ok(num) => num, 
        Err(_) => 0,
    };
    dbg!(s);
    v[s];
}