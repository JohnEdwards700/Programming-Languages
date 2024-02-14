fn main() {
    println!("This is the Main Func");
    //using function in main func
    sum_of_squares(2, 3);

    //fuzzbuzz implementation
    for n in 1..101 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("fizzbuzz")
        } else if n % 3 == 0 {
            println!("fizz")
        } else if n % 5 == 0 {
            println!("buzz")
        } else {
            println!("{}", n)
        }
    }
}
//accepts two parameters and produces a string
fn sum_of_squares(x: i32, y: i32) {
    let s = x * x + y * y;
    println!("The sum of the squares is {}", s);
}
