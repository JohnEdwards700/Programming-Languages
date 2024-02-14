fn main() {
    println!("This is the Main Func");

    let result = fizz_buzz(101);
    println!("{}", result)
}
//fuzzbuzz implementation
fn fizz_buzz(n: i32) -> String {
    let mut result = String::new();

    for x in 1..n {
        let mut answer = String::new();
        fizzbuzz(&mut answer, x);
        fizz(&mut answer, x);
        buzz(&mut answer, x);
        baz(&mut answer, x);

        if answer.is_empty() {
            answer = x.to_string();
        }

        result.push_str(&answer);
        result.push('\n');
    }
    result
}
fn fizzbuzz(answer: &mut String, num: i32) {
    if num % 3 == 0 && num % 5 == 0 {
        answer.push_str("FizzBuzz");
    }
}

fn fizz(answer: &mut String, num: i32) {
    if num % 3 == 0 {
        answer.push_str("Fizz");
    }
}

fn buzz(answer: &mut String, num: i32) {
    if num % 5 == 0 {
        answer.push_str("Buzz");
    }
}

fn baz(answer: &mut String, num: i32) {
    if num % 7 == 0 {
        answer.push_str("Bazz");
    }
}