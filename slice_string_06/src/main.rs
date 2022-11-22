fn main() {
    let s = String::from("world,hello");
    let f_word = first_world(&s);

    // s.clear(); //
    println!("the first word is : {}", f_word);
}

fn first_world(s: &String) -> &str {
    &s[..1]
}
