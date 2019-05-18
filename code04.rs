// Shadowing

// shadowing is declaring that variable again or replacing that variable using
// 'let' keyword.

fn main(){
    let x = 10;
    println!("Value of x is: {}",x);

    let x = 20; // previous x is shadowed here

    let x = x*2; // previous x is shadowed here

    let x = x+5; // previous x is shadowed here

    println!("Value of x is: {}",x);

    let x = "Hello";
    let x = x.len(); // changing the type of variable from string to integer

    println!("{}",x);
}

// shadowing vs mut
// by using let we can have few transformations and then we can still keep our variable
// immutable after that.println!
// + we can also change the type of variable here 

