// constants

// constants are variable that aren't allowed to change.
// 'mut' keyword can't be used with constants.

// constant is declared by using a 'const' keyword & type of the value must be annotated.

// constants can be declared in any scope.

// constants can only be set to a constant expression.
// not to the result of a function or any value that is calculated at runtime.

// constants are valid for entire time a program runs(within the scope they are
// declared in).

fn main(){

    const DAYS_IN_WEEK: u8 = 7;
    const SECONDS_IN_A_DAY: u32 = 86_400;

    println!("There are {} days in a week",DAYS_IN_WEEK);
    println!("There are {} seconds in a day",SECONDS_IN_A_DAY);

}