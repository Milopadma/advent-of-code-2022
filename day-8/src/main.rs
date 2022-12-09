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
    print!("Input is: {:?}", tree);

    // at this point, the input is a 2D vector of numbers
    // we can now iterate over the vector and count the visible trees
    // Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.
    // A tree is visible if all of the other trees between it and an edge of the grid are shorter than it.
    // Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.
    // All of the trees around the edge of the grid are visible - since they are already on the edge,
    // trees are only visible if the trees between them and the edge are shorter.

    let mut visible_trees = 0;
    // vector to keep track the trees that are visible
    let mut visible: Vec<u32> = Vec::new();

    for row in 0..tree.len() {
        for col in 0..tree[row].len() {
            // for the edge checks
            if row == 0 || row == tree.len() - 1 || col == 0 || col == tree[row].len() - 1 {
                visible.push(tree[row][col]);
                visible_trees += 1;
                // skip the rest of the checks
                continue;
            }

            // for the cardinal direction checks
            let mut visible_from_left = true;
            let mut visible_from_right = true;
            let mut visible_from_top = true;
            let mut visible_from_bottom = true;
            // check if the tree is visible from the left
            for i in 0..col {
                if tree[row][i] >= tree[row][col] {
                    visible_from_left = false;
                    break;
                }
            }
            // check if the tree is visible from the right
            for i in col + 1..tree[row].len() {
                if tree[row][i] >= tree[row][col] {
                    visible_from_right = false;
                    break;
                }
            }
            // check if the tree is visible from the top
            for i in 0..row {
                if tree[i][col] >= tree[row][col] {
                    visible_from_top = false;
                    break;
                }
            }
            // check if the tree is visible from the bottom
            for i in row + 1..tree.len() {
                if tree[i][col] >= tree[row][col] {
                    visible_from_bottom = false;
                    break;
                }
            }
            // a tree is only not visible if it is not visible from all directions
            if visible_from_left && visible_from_right && visible_from_top && visible_from_bottom {
                // add it to the vector
                visible.push(tree[row][col]);

                visible_trees += 1;
            }
        }
    }

    println!("Visible trees: {}", visible_trees);
    println!("Visible trees: {:?}", visible);
}
// 1. 831, too low
// 2. 731, too low
// 3. 827, too low
// 4. 9272, incorrect