//fn main() {
    // let greeting = String::from("hello world");
    // say_greeting2(greeting);
    // println!("{}", greeting);
    //}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    // s.clear();
    println!("{}", word);
}

fn first_word(s: &String) -> &str { // &str表示字符串切片类型
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[0..s.len()];
}

// fn say_greeting2(greeting: String) {
//     println!("{}", greeting);
// }

// fn sayGreeting(String& greeting) {
//     println!(greeting);
// }
