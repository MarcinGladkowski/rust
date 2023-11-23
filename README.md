### Rust


#### Start
* Files are ending with extension ```.rs```
* Compile class by ```rustc``` command
* ```main()``` function is entry of each program
    * can have arguments
* lines/execution parts are ending with parenthesis ```;```
* functions with ```!``` are __macros__

#### Cargo
* Helps to create new project
* Create ```src``` and ```toml``` files to manage project
* ```cargo build```
  * It's default build with ```target/debug``` build
* ```cargo run``` - compile and run program
* ```cargo check``` - compile and check errors
* ```cargo build --released``` - compile optimized version
* ```cargo update``` - updating dependencies

#### Dependencies
* ```https://crates.io/```
* ```cargo doc --open``` - build docs dependencies of your program

https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


#### Cargo.lock file
```cargo update```: ignore lock file and trying to find out the newest version of packages

#### Variables 
> https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

##### Main concepts
* variables by defaults are **immutable!**
* but you can make variables mutable if you want!
