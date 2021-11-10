// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}

fn print_result(res: i32) {
    println!("{:?}", res);
}

fn main() {
    print_result(sum(10, 20));
}
