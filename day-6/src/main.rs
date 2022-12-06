fn main() {
    let test_input = "zzaabecswssw";
    // getting the input txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // using the moving window algorithm, increment the window by 1, and check if the 4 characters in the window are different from each other
    let mut count = 0;
    for i in 0..input.len() - 3 {
        let window = &input[i..i + 4];
        // to find out if the 4 characters are unique
        let mut unique = true;
        for j in 0..window.len() {
            for k in 0..window.len() {
                if j != k && window.chars().nth(j) == window.chars().nth(k) {
                    unique = false;
                }
            }
        }
        // println
        if unique {
            println!("{} is unique", window);
            // print the number of chars it has been since the first char
            println!("{} chars since the first char", i);
            println!("{} chars received", i + 4);
        }
    }
    println!("{}", count);
}