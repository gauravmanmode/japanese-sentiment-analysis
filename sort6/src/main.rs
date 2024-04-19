
fn sort(v: &mut Vec<i32> ) {
    for i in 0..v.len() {
        if let Some(key) = v.pop() {

            let mut i = 0;
            while i < v.len() {
                 if key < v[i] {
                 v.insert(i, key);
                 break;
                 }
                 else {
                    v.push(key);
                    break;
                 }
                 i += 1;
                 
            }
        }
    }
    
}



fn main() {
    let mut v: Vec<i32> = vec![4,5,6,7,9,8,2,1,3];
     sort(&mut v);
     for i in &mut v {
        println!("{i}");
     }
    
 }

