fn main() {
    let mut v = Vec::new();
    v.push(2);
    v.push(4);
    v.push(6);
    let mut v2 = vec![2, 4, 6, 8];
    for i in &mut v2 {
        *i += 1;
    }
    for j in &v2 {
        println!("{}", *j);
    }
    println!("Hello, world!");
}
