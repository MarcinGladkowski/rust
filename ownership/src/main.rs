fn main() { // not valid
    let s = "Hello world"; // valid
    // do stuff with s

    let mut word = String::from("hello"); // mutable

    word.push_str(" world");

    print!("{}", word);

} // scope is over, no longer valid
