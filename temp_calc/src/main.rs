use std::io;

fn main() {
    println!("Convert from Fahrenheit to Celsius!");

    loop {
        println!("Enter temperature in Fahrenheit:");
        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Couldn't read input!");

        if user_input.trim() == "quit" || user_input.trim() == "exit" {
            break;
        }

        let user_input :f32 = match user_input
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a number!");
                    continue;
                },
            };

        let result = fahr_to_cels(user_input);

        println!("Temperature in Celsius: {result}");

    }
}

fn fahr_to_cels(num :f32) -> f32 {
    return (num - 32.0) * (5.0/9.0)
}
