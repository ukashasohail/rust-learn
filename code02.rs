// Variables

// Variables are immutable by default in Rust, however you can make them mutable.
// immutable means you can't change the value of variable unless you redefine it.

// variables are defined with 'let' keyword.

fn main(){

    //
    // let x = 10;
    // x = 20;
    // println!("The value of x is: {}",x);
    //

    // ^^ Upper block of code will give you compile time error because variables are immutable
    // by default and you can't edit them.

    // we can make above variable mutable by adding 'mut' keyword.

    //
    let mut x = 10;
    x = 20;
    println!("The value of x is: {}",x);

}
