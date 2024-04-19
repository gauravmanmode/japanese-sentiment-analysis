
fn main() {
    let s = String::new();
    let s = "gaurav".to_string();
    let mut s = String::from("'gaurav");
    let s2 ="hello";
    let s1 = String::from(" helloj");
    //s.push_str(s2);

    dbg!(s2);


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}+{s2}+{s3}");
    dbg!(s);
    
    let s = s1 + &s2 + &s3;

    for i in s.chars() {
        println!("{i}");
    }

    for i in s.bytes() {
        println!("{i}");
    }

}
