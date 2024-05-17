## Rust

* Statically typed language

### Troubleshooting

* Missing `Cargo.toml` file. It's not possible to build project `cargo build` when this file
is missing. You can generate it by command `cargo init`.

### Start
* Creating new executable project `cargo new <name - will be project directory>`
* Files are ending with extension ```.rs```
* Compile class by ```rustc``` command
* ```main()``` function is entry of each program
    * can have arguments
* lines/execution parts are ending with parenthesis ```;```
* functions with ```!``` are __macros__

### Cargo
* Helps to create new project
* Create ```src``` and ```toml``` files to manage project
* ```cargo build```
  * It's default build with ```target/debug``` build
* ```cargo run``` - compile and run program
* ```cargo check``` - compile and check errors
* ```cargo build --released``` - compile optimized version
* ```cargo update``` - updating dependencies

### Dependencies
* ```https://crates.io/```
* ```cargo doc --open``` - build docs dependencies of your program

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


### Cargo.lock file
```cargo update```: ignore lock file and trying to find out the newest version of packages

### Variables 
> https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

### Main concepts
* variables by defaults are **immutable!**
* but you can make variables mutable if you want! `mut`
  * keyword `mut` don't allow to assign another type value to variable
* constants are immutable also `const`
* Shadowing variables - declares with the same name
  * Allow to redeclare with other value or type

### Data types [#doc](https://doc.rust-lang.org/book/ch03-02-data-types.html)

Data types subsets:
* Scalar - represents a single value
  * integers 
      ```
      example: u32 or i32
    
      u -> unsigned (not allows negative values)
      i -> signed (allow negative values)
    
      i8 -> from -(2**7) to 2**7-1 (from -128 to 128)
      but:
      u8 -> from 0 to 2**8-1 (from 0 to 255)
      ``` 
      
      ```
      Integer overflow
      example: 
        i32 -> and the program willing to assing integer 256
      
      * In --release mode it change value to **1**
      * In debug programs **interupt** with panic mode
      ```
      
      ```
      Use underscore _ to make integers more readable => 100_000
      ```

  * floating-points
      ```
      Two primitive values f32 and f64. f64 is default and more usable because
      is more precise and not much slower than f32. All floating points are signed.
    
      let x = 2.0 // f64 - double precision float
      let y: f32 = 1.6 // f32 - single precision float
    
      Implementing standard IEEE-754
      ```
  * numbers
  * Boolean
    ```
    Two possible states true/false
    
    let x = true;
    let y: bool = false;
    ```
  * characters
    ```
    let x = 'Marcin'
    let y: char = 'developer'
    
    Single qotes '' => char literals - four bytes in size, can represent Unicode chars
    Dobule quotes "" => string literal
    
    ```
  
* Compound types (multiple values in one type)
  
  * Tuples
    ```
    Like in Python cannot shrink or grow once time created tuple
    
    let tup = (20, 10.0)
    
    let (x, y) = tup // can use destructuring, like in Python/JS
    
    let y: (u32, f64) = (20, 1.0)
    
    let z = y.0 // access by index
    let a = y.1
    
    Empty created tuple is called UNIT. Usable to return nothing.
    ```
  
  * Array type
  ```
    * All values stored in array MUST have the same type
    * Length is fixed - not changeable
    * If you want to use changeable data structure probably you can use a Vector
  
    let x = [1, 2, 3, 4]
    
    let y: [u32, 3] = [1, 2, 3]
  
    let z: [3, 2]; // is the same like let z: u32 = [3, 3]
    
    access to elements by indexes starting from 0 
    let one = z[0]
    let second = z[1]
  ```

### Functions
* snake_case
* entry point of program contains function `main()`
* types declarations are required
* Statements - perform actions but not return value 
  - declare variable
  ```let x = 12```
   - function definition
* Expressions - return values
  - execute function

### Ownership 
* Unique for Rust language
* Guarantees safety memory without a garbage collector
* Stack and heap
* Allocation of memory determines why 
  ```
  String is mutable but str not
  ```
