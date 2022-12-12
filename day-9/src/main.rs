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
    let mut grid = [[0; 10]; 10];
    for row in &grid {
        println!("{:?}", row);
    }
}