use std::io;

use maze_solver::*;

fn main() {
    let mut buf = String::new();
    let mut point_str :Vec<&str>;

    let mut start = Point::new();
    let mut end = Point::new();

    println!("Enter start point (0-based) in the form: x y ");

    io::stdin().read_line(&mut buf).unwrap();
    point_str = buf.split(' ').collect();
    start.x = point_str[0].trim().parse().expect("Please enter valid unsigned integers!");
    start.y = point_str[1].trim().parse().expect("Please enter valid unsigned integers!");

    buf = String::new();


    println!("Enter end point (0-based) in the form: x y ");

    io::stdin().read_line(&mut buf).unwrap();
    point_str = buf.split(' ').collect();
    end.x = point_str[0].trim().parse().expect("Please enter valid unsigned integers!");
    end.y = point_str[1].trim().parse().expect("Please enter valid unsigned integers!");

    buf = String::new();

    
    println!("Enter maze (use spaces for open cells and # for walls):");

    let mut maze :Vec<Vec<char>> = Vec::new();

    loop {
        if let Err(_) = io::stdin().read_line(&mut buf) {
            break;
        }

        if buf == "\n" {
            break;
        }

        let mut row :Vec<char> = Vec::new();

        for c in buf.chars() {
            if c != '\n' {
                row.push(c);
            }
        }

        maze.push(row);
        buf = String::new();
    }

    println!("Solution:");
    println!("{}", maze_solver::solve_maze(&mut maze, start, end));

}
