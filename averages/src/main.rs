mod utils;

use std::io;

fn main() {
    println!("Enter your numbers (space separated integers):");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error! Input could not be read!");

    let mut user_arr :Vec<i32> = Vec::new();

    for num in &mut user_input.split_whitespace() {
        let num :i32 = num.parse().expect("Type conversion did not work!");
        user_arr.push(num);
    }

    let user_arr = utils::merge_sort(user_arr);
    let median :f32 = utils::get_median(&user_arr);
    let mode :Vec<i32> = utils::get_mode(&user_arr);
    let mean :f32 = utils::get_mean(&user_arr);

    println!("Soreted array: {:?}", user_arr);
    println!("Median: {:?}", median);
    println!("Mode: {:?}", mode);
    println!("Mean: {:?}", mean);
}