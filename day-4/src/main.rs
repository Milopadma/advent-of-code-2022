fn main() {
    // get the input
    let input = std::fs::read_to_string("input.txt").expect("Error reading input file");
    // split the input to a new vector for every \n
    let input: Vec<&str> = input.split('\n').collect();
    // create a new vector for every string in the input vector
    let input: Vec<Vec<&str>> = input
        .iter()
        // this is to remove the empty string at the end of the input
        .map(|x| x.split(' ').collect())
        .collect();
    


}