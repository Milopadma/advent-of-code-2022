fn main() {
    // input (tree heights)
    let raw_input = std::include_str!("test_input.txt");

    // each number in the input is a tree height, being 0 the shortest, and 9 the tallest
    // a tree is visible if all the other trees between it and an edge of the grid are shorter than it,
    // only considering trees of the same column or row.
    // each new line is a new row, and each number is a new column

    // the input is a string, so we need to split it into lines
    let lines = raw_input.split('\n').collect::<Vec<&str>>();

    // we need to convert the strings into numbers
    let mut tree = Vec::new();
    for line in lines {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(match c.to_digit(10) {
                Some(_) => { c.to_digit(10).unwrap() }
                None => { 0 }
            });
        }
        tree.push(row);
    }
    println!("Input is: {:?}", tree);

    // at this point, the input is a 2D vector of numbers
    // we can now iterate over the vector and count the visible trees
    // Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
    // A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
    // Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
    // All of the trees around the edge of the grid are visible - since they are already on the edge,
    // trees are only visible if the trees between them and the edge are shorter.

    // lets count the total number of trees first
    let total_trees = count_total_trees(&tree);
    println!("Total trees: {}", total_trees);

    // now lets count the number of trees on the edges
    let edge_trees = count_edge_trees(&tree);
    println!("Edge trees: {}", edge_trees);

    // now lets count the number of visible trees
    let total_visible = count_visible_numbers(&tree);
    println!("Visible trees: {}", total_visible);

    // for part 2, need to count the "scenic score"
    let all_tree_scenic_score = count_scenic_score(&tree);
    // print out the vector after each element has been multiplied by its 4 cardinal directions
    println!(
        "Scenic scores: {:?}",
        all_tree_scenic_score.tree_scenic_scores
            .iter()
            .map(|x| x.north * x.south * x.east * x.west)
            .collect::<Vec<u32>>()
    );

    println!(
        "Highest scenic score: {}",
        all_tree_scenic_score.tree_scenic_scores
            .iter()
            .max_by_key(|x| x.north + x.south + x.east + x.west)
            .unwrap().north +
            all_tree_scenic_score.tree_scenic_scores
                .iter()
                .max_by_key(|x| x.north + x.south + x.east + x.west)
                .unwrap().south +
            all_tree_scenic_score.tree_scenic_scores
                .iter()
                .max_by_key(|x| x.north + x.south + x.east + x.west)
                .unwrap().east +
            all_tree_scenic_score.tree_scenic_scores
                .iter()
                .max_by_key(|x| x.north + x.south + x.east + x.west)
                .unwrap().west
    );
    println!("Scenic scores: {:?}", all_tree_scenic_score);
    // 1. 2765952 is too high
    // 2. 1420978 is too high
    // 3. 145 is too low
}

#[derive(Debug)]
struct TreeScenicScore {
    tree_height: u32,
    north: u32,
    south: u32,
    east: u32,
    west: u32,
}

#[derive(Debug)]
struct AllTreeScenicScores {
    tree_height: Vec<u32>,
    tree_scenic_scores: Vec<TreeScenicScore>,
}

fn count_scenic_score(tree: &Vec<Vec<u32>>) -> AllTreeScenicScores {
    // init new struct of TreeScenicScore
    let mut tree_scenic_score = TreeScenicScore {
        tree_height: 0,
        north: 0,
        south: 0,
        east: 0,
        west: 0,
    };

    // init new struct of AllTreeScenicScores
    let mut all_tree_scenic_scores = AllTreeScenicScores {
        tree_height: Vec::new(),
        tree_scenic_scores: Vec::new(),
    };

    // iterate through the matrix
    for row in 0..tree.len() {
        for column in 0..tree[row].len() {
            let current_number = tree[row][column];

            // Find the coordinates of the number in the matrix
            let mut coords = (0, 0);
            for i in 0..tree.len() {
                for j in 0..tree[i].len() {
                    if tree[i][j] == current_number {
                        coords = (i, j);
                        break;
                    }
                }
            }

            // Measure the viewing distance by looking up
            for i in (0..coords.0).rev() {
                if tree[i][coords.1] == current_number || tree[i][coords.1] < current_number {
                    break;
                }
                tree_scenic_score.north += 1;
            }

            // then to the same for the other 3 cardinal directions
            // check down
            for k in row + 1..tree.len() {
                if tree[k][column] >= tree[row][column] {
                    tree_scenic_score.south += 1;
                    break;
                } else {
                    tree_scenic_score.south += 1;
                }
            }

            // check left
            for k in 0..column {
                if tree[row][k] >= tree[row][column] {
                    tree_scenic_score.east += 1;
                    break;
                } else {
                    tree_scenic_score.east += 1;
                }
            }

            // check right
            for k in column + 1..tree[row].len() {
                if tree[row][k] >= tree[row][column] {
                    tree_scenic_score.west += 1;
                    break;
                } else {
                    tree_scenic_score.west += 1;
                }
            }

            tree_scenic_score.tree_height = tree[row][column];

            // add the total to the vector
            all_tree_scenic_scores.tree_scenic_scores.push(tree_scenic_score);
            // add the tree height
            all_tree_scenic_scores.tree_height.push(tree[row][column]);
            // and reset the tree_scenic_score
            tree_scenic_score = TreeScenicScore {
                tree_height: 0,
                north: 0,
                south: 0,
                east: 0,
                west: 0,
            };
        }
    }

    // return the vector
    all_tree_scenic_scores
}

fn count_visible_numbers(tree: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;

    // Go through each element in the matrix
    for row in 0..tree.len() {
        for column in 0..tree[row].len() {
            // Check if the current element is visible by looking up
            let mut visible = true;
            for k in 0..row {
                if tree[k][column] >= tree[row][column] {
                    visible = false;
                    break;
                }
            }

            // If the current element is not visible by looking up,
            // check if it is visible by looking down
            if !visible {
                visible = true;
                for k in row + 1..tree.len() {
                    if tree[k][column] >= tree[row][column] {
                        visible = false;
                        break;
                    }
                }
            }

            // If the current element is not visible by looking up or down,
            // check if it is visible by looking left
            if !visible {
                visible = true;
                for k in 0..column {
                    if tree[row][k] >= tree[row][column] {
                        visible = false;
                        break;
                    }
                }
            }

            // If the current element is not visible by looking up, down, or left,
            // check if it is visible by looking right
            if !visible {
                visible = true;
                for k in column + 1..tree[row].len() {
                    if tree[row][k] >= tree[row][column] {
                        visible = false;
                        break;
                    }
                }
            }

            // If the current element is visible, increment the count
            if visible {
                count += 1;
            }
        }
    }

    count
}

fn count_edge_trees(tree: &Vec<Vec<u32>>) -> u32 {
    let mut edge_trees = 0;
    // make sure not to count the trees that are on the edges twice, by offsetting the loop by 1, respectively
    // or.. just count how many entries are on the first and last rows
    for row in 0..tree.len() {
        for col in 0..tree[row].len() {
            if row == 0 || row == tree.len() - 1 || col == 0 || col == tree[row].len() - 1 {
                edge_trees += 1;
            }
        }
    }
    edge_trees
}

fn count_total_trees(tree: &Vec<Vec<u32>>) -> u32 {
    let mut total_trees = 0;
    for row in 0..tree.len() {
        for col in 0..tree[row].len() {
            if tree[row][col] != 0 {
                total_trees += 1;
            }
        }
    }
    total_trees
}
// 1. 831, too low
// 2. 731, too low
// 3. 827, too low
// 4. 9272, incorrect
// 5. 1693, correct