// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable is 
//  > 5, < 5, or == 5, respectively
//
// Notees:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    // Use a variable set to any integer value
    let my_variable = 5;

    // Display ">5", "<5", or "=5" based on the value of a variable is 
    //  > 5, < 5, or == 5, respectively
    if my_variable == 5 {
        println!("=5");
    } else if my_variable < 5 {
        println!("<5");
    }  else {
        println!(">5");
    }
 }