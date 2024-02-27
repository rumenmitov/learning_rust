use std::collections::HashSet;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    pub x :usize,
    pub y :usize,
}

impl Point {
    pub fn new() -> Point {
        return Point {
            x: 0,
            y: 0
        }
    }
}

pub fn solve_maze(maze :&mut Vec<Vec<char>>, start :Point, end :Point) -> String {
    let mut path :Vec<Rc<Point>> = Vec::new();
    let mut seen :HashSet<Rc<Point>> = HashSet::new();

    if walk(maze, start, Rc::new(end), &mut path, &mut seen) == false {
        return String::from("No path found!");
    } 

    for p in path.iter() {
        maze[p.y][p.x] = '*';
    }

    let mut maze_result = String::new();

    for row in maze.iter() {
        let mut buf = String::new();

        for c in row.iter() {
            buf.push(*c);
        }

        maze_result.push_str(&buf);
        maze_result.push('\n');
    }

    maze_result
}


fn walk(maze :&Vec<Vec<char>>, curr :Point, end :Rc<Point>, path :&mut Vec<Rc<Point>>, seen :&mut HashSet<Rc<Point>>) -> bool {
    if curr == *end {
        return true;
    }

    if curr.y >= maze.len() || curr.x >= maze[0].len() {
        return false;
    }

    for p in seen.iter() {
        if curr == *p.as_ref() {
            return false;
        }
    }

    if maze[curr.y][curr.x] == '#' {
        return false;
    }

    let directions = [
        [ 1, 0 ],
        [ 0, 1 ],
    ];

    let curr_ref = Rc::new(curr);

    seen.insert(Rc::clone(&curr_ref));
    path.push(Rc::clone(&curr_ref));

    for i in 0..2 {

        let new_curr = Point {
            x: curr_ref.x + directions[i][0],
            y: curr_ref.y + directions[i][1],
        };

        if walk(maze, new_curr, Rc::clone(&end), path, seen) == true {
            path.push(Rc::clone(&end));
            return true;
        }
    }

    path.pop().unwrap();
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_straight() {
        let mut maze = vec![
            vec!['#', '#', ' ', '#', '#'],
            vec!['#', '#', ' ', '#', '#'],
            vec!['#', '#', ' ', '#', '#'],
        ];

        let start = Point {
            x: 2,
            y: 0
        };

        let end = Point {
            x: 2,
            y: 2
        };

        let solution = String::from("##*##\n##*##\n##*##\n");

        assert_eq!(solve_maze(&mut maze, start, end), solution);
    }

    #[test]
    fn test_turn() {
        let mut maze = vec![
            vec!['#', '#', ' ', '#', '#'],
            vec!['#', '#', ' ', ' ', ' '],
            vec!['#', '#', '#', '#', '#'],
        ];

        let start = Point {
            x: 2,
            y: 0
        };

        let end = Point {
            x: 4,
            y: 1
        };

        let solution = String::from("##*##\n##***\n#####\n");

        assert_eq!(solve_maze(&mut maze, start, end), solution);
    }

    #[test]
    fn test_no_solution() {
        let mut maze = vec![
            vec!['#', '#', ' ', '#', '#'],
            vec!['#', '#', ' ', '#', ' '],
            vec!['#', '#', '#', '#', '#'],
        ];

        let start = Point {
            x: 2,
            y: 0
        };

        let end = Point {
            x: 4,
            y: 1
        };

        let solution = String::from("No path found!");

        assert_eq!(solve_maze(&mut maze, start, end), solution);
    }

    #[test]
    fn test_loop() {
        let mut maze = vec![
            vec!['#', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', ' ', '#'],
            vec!['#', ' ', '#', ' ', '#'],
            vec!['#', ' ', ' ', ' ', '#'],
            vec!['#', ' ', '#', '#', '#'],
        ];

        let start = Point {
            x: 1,
            y: 0
        };

        let end = Point {
            x: 1,
            y: 4
        };

        let solution = String::from("#*  #\n#*# #\n#*# #\n#*  #\n#*###\n");

        assert_eq!(solve_maze(&mut maze, start, end), solution);
    }
    
}
