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
    let mut head: (isize, isize) = (10, 10);
    let mut tail: (isize, isize) = (10, 10);

    println!("example input: {:?}", example_input);

    // split the input by \n
    let split_input: Vec<&str> = example_input.split("\n").collect();
    // to remove the empty string at the start of each line
    let split_input: Vec<&str> = split_input
        .iter()
        .map(|x| x.trim())
        .collect();
    println!("split input: {:?}", split_input);

    // move the head and tail according to the input
    let mut first_move = true;
    for line in split_input {
        let (direction, distance) = line.split_at(1);
        // since the tail follows the head, the tail will always be one step behind the head
        // offset the tail by one step in the direction of the head
        println!("Direction: {}, Distance: {}", direction, distance);
        let mut distance = distance.trim().parse::<isize>().unwrap();

        // now move the head and tail normally according to the input
        while distance > 0 {
            // move the head in the direction specified
            head = match direction {
                "U" => (head.0 - 1, head.1),
                "D" => (head.0 + 1, head.1),
                "L" => (head.0, head.1 - 1),
                "R" => (head.0, head.1 + 1),
                _ => panic!("Invalid direction"),
            };

            // update the grid
            grid[head.0 as usize][head.1 as usize] += 1;

            distance -= 1;
        }
        // only start updating the tails location after the head has moved once
        // After each step, you'll need to update the position of the tail if the step means the head is no longer adjacent to the tail
        // Check if the head is two steps away from the tail in any direction

        if (head.0 - tail.0).abs() == 2 || (head.1 - tail.1).abs() == 2 {
            // Move the tail in the same direction as the head
            tail = match direction {
                "U" => (tail.0 - 1, tail.1),
                "D" => (tail.0 + 1, tail.1),
                "L" => (tail.0, tail.1 - 1),
                "R" => (tail.0, tail.1 + 1),
                _ => panic!("Invalid direction"),
            };
        } else if (head.0 - tail.0).abs() == 1 && (head.1 - tail.1).abs() == 1 {
            // Move the tail in the same direction as the head
            tail = match direction {
                "U" => (tail.0 - 1, tail.1),
                "D" => (tail.0 + 1, tail.1),
                "L" => (tail.0, tail.1 - 1),
                "R" => (tail.0, tail.1 + 1),
                _ => panic!("Invalid direction"),
            };
        }

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