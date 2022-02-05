fn main() {
    let mut s = String::from("xx yy");

    let word = first_word(&s);

    println!("{}", word);
}

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &s) in bytes.iter().enumerate() {
        if s == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}
