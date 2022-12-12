// Advent of Code day 9 - Rope bridge problem

fn main() {
    let mut example_input = "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    // 1. Objective: Count all the positions the tail visited at least once.

    // Part 1.

    // create a 2 dimensional array of 0s to simulate the grid
    let mut grid = [[0; 20]; 20];
    for row in &grid {
        println!("{:?}", row);
    }

    // the start of the Head H and Tail T is at the center of the grid
    let mut head = (10, 10);
    let mut tail = (10, 10);

    println!("example input: {:?}", example_input);

    // split the ionput by \n
    let split_input: Vec<&str> = example_input.split("\n").collect();
    // to remove the empty string at the start of each line
    let split_input: Vec<&str> = split_input
        .iter()
        .map(|x| x.trim())
        .collect();
    println!("split input: {:?}", split_input);

    // move the head and tail according to the input
    for line in split_input {
        let (direction, distance) = line.split_at(1);
        println!("Direction: {}, Distance: {}", direction, distance);
        let distance = distance.trim().parse::<usize>().unwrap();
        match direction {
            "R" => {
                for _ in 0..distance {
                    head.0 += 1;
                    grid[head.0][head.1] += 1;
                }
            }
            "L" => {
                for _ in 0..distance {
                    head.0 -= 1;
                    grid[head.0][head.1] += 1;
                }
            }
            "U" => {
                for _ in 0..distance {
                    head.1 += 1;
                    grid[head.0][head.1] += 1;
                }
            }
            "D" => {
                for _ in 0..distance {
                    head.1 -= 1;
                    grid[head.0][head.1] += 1;
                }
            }
            _ => println!("Invalid direction"),
        }
        // move the tail
        grid[tail.0][tail.1] += 1;
        tail = head;

        // print the grid
        for row in &grid {
            println!("{:?}", row);
        }
    }

    // count the number of positions the tail visited at least once
    let mut count = 0;
    for row in &grid {
        for col in row {
            if *col > 0 {
                count += 1;
            }
        }
    }

    println!("Number of positions the tail visited at least once: {}", count);
}