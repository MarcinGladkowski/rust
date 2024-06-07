fn main() { // not valid
    let s = "Hello world"; // valid (in scope)
    // do stuff with s

    let mut word = String::from("hello"); // mutable

    word.push_str(" world");

    print!("{}", word);

} // scope is over, no longer valid


fn string_type() {
    // String type is more complex and good to explain memory
    // optimization/safe
    // String is useful when we do not know length while compiling the code
    let s = String::from("hello"); // requests for the memory allocation
    // :: <- allow to use function form String namespace
    // HEAP memory storage, not STACK as `str`
    s.push_str(", world"); // appends literals

    println!("{}", s); // Prints `Hello world`

}


fn free_memory() {
    let s = String::from("Hello"); // s is valid
    // do stuff
    // valid
} // no longer valid - out of scope - Rust run `drop` function - automatically


fn data_interaction() {
    let x = 5;
    let y = x; // copy value and set new variable to memory

    let s1 = String::from("hello");
    let s2 = s1; // different action
    // information about s1 are located in Stack but value (content) on HEAP
}