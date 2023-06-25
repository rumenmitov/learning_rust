use std::io;

fn main() {
    println!("Fibonacci number generator!");

    let mut n = String::new();
    println!("Enter the n-th term (positive integer):");
    io::stdin()
        .read_line(&mut n)
        .expect("Couldn't read input!");

    let n :u64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("Term {n} in Fibonacci is: {}", get_fib_num(n));

}

fn get_fib_num(num :u64) -> u64 {
    if num == 1 {
        return 0;
    } else if num == 2 {
        return 1;
    } else {
        return get_fib_num(num-1) + get_fib_num(num-2);
    }
}
