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
    let mut s = String::from("hello"); // requests for the memory allocation
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
    let s2 = s1; // different action - s1 was moved into s2
    // information about s1 are located in Stack but value (content) on HEAP
    // why Rust throws error ? s1 is no longer valid
    // 'double free' error
    // 'shallow' copy is called as a 'move' in RUST
    println!("{s1}, world");
}

fn data_clone() {

    let s1 = String::from("hello");
    let s2 = s1.clone(); // copy data, it can be expensive operation

    println!("s1 = {s1}, s2 = {s2}");

}

fn stack_only_data() {
    let x = 5;
    let y = x;
    // these types are know in time of compiling
    // copying these values (types) is cheap
    // COPY trait is implemented by any types
    println!("x = {x} ,y = {y}");
}

// ownership and functions
fn function_ownership() {
    let s = String::from("hello");

    take_ownership(s);  // s's value moves to function
                        // no longer valid
                        // trying to use s here occurring compile-time error

    let x = 5;

    makes_copy(x)  // value moves to function
                   // i32 is copied and still available to use afterward
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// return values and scope
fn return_values_scope() {
    let s1 = gives_ownership();
    let s2  = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // returned and moves out to the calling function
}

// Main concepts:
// - assigning a value to another variable moves it


// Returning multiple values
fn simulate_main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_something(s1);

    println!("The length of {s2} is {len}")
}

fn calculate_something(a_string: String) {
    let length = a_string.len();

    (s, length)
}

