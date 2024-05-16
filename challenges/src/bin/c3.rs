// Topic: Flow control lusing if..else
//
// Program requirements:
// * Display a message based on the value of a boolean variable 
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
//
// Notes:
// * Use a variable set to either true or false
// * Use am if..else block to determine which message to display
// * Use the println macro to display messages to the terminal 

fn main() {
    // * Use a variable set to either true or false
    let bit = false;
    // * Use am if..else block to determine which message to display
    if bit == true {
        // * Use the println macro to display messages to the terminal 
        println!("hello");
    } else {
        // * Use the println macro to display messages to the terminal 
        println!{"goodbye"};
    }
}