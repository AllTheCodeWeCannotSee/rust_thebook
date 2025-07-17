use std::io;

fn main() {
    let mut num = String::new();

    println!("type a num");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    let num: u32 = num.trim().parse().expect("please type a number");
    let res = fib(num);

    println!("fib num is {res}");
}

// 1 1 2 3 5
fn fib(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
