// advent of code day 1 the calorie problem

fn main() {
    // open up the input.txt file and load it into an array, where every double \n is a new array
    let inputraw = std::fs::read_to_string("input.txt").unwrap();
    let input_split_per_elf = inputraw.split("\n").collect::<Vec<&str>>();
    // now for every \n in the array, split it into a new seperate array
    let mut input_split_per_number = Vec::new();
    for i in &input_split_per_elf {
        input_split_per_number.push(i.split("\n").collect::<Vec<&str>>());
    }
    // turn every string that is a number into an integer, leave the empty strings alone
    let mut input_integers = Vec::new(); //new vector array
    for i in &input_split_per_number {
        let mut temp = Vec::new();
        for j in i {
            if j != &"" {
                temp.push(j.parse::<i32>().unwrap());
            }
        }
        input_integers.push(temp);
    }
    // for every array in the array, add it to a new array and create a new array everytime an empty array is found
    let mut input_integers_2 = Vec::new(); //new vector array
    let mut temp = Vec::new();
    for i in &input_integers {
        if i.len() == 0 {
            input_integers_2.push(temp);
            temp = Vec::new();
        } else {
            temp.push(i[0]);
        }
    }
    input_integers_2.push(temp);

    // for every array in inputintegers2, pair it with a key of "elf-number"
    let mut input_integers_3 = Vec::new(); //new vector array
    for i in 1..input_integers_2.len() {
        input_integers_3.push(("elf-".to_string() + &i.to_string(), input_integers_2[i].clone()));
    }
    // println!("{:?}", input_integers_3);

    // now time to find the elf with the most calories, for part 1
    let mostcalories: (String, i32) = find_most_calories(&input_integers_3);
    println!("The elf with the most calories is: {:?}", mostcalories);

    // for part 2, finding the total calories of the top three elves
    let top3calories = top_three_calorie_elfs(&input_integers_3);
    // to add the calories together, we need to use a fold
    let totalcalories = top3calories.iter().fold(0, |acc, x| acc + x.1);
    println!("The top three elves have a total of {:?} calories", totalcalories);
}

// for part 1
fn find_most_calories(input: &Vec<(String, Vec<i32>)>) -> (String, i32) {
    let mut most_calories = 0;
    let mut most_calories_elf = String::new();
    input.iter().for_each(|i| {
        let mut calories = 0;
        for j in &i.1 {
            calories += j;
        }
        if calories > most_calories {
            most_calories = calories;
            most_calories_elf = i.0.clone();
        }
    });
    (most_calories_elf, most_calories)
}

// for part2
fn top_three_calorie_elfs(input: &Vec<(String, Vec<i32>)>) -> Vec<(String, i32)> {
    let mut top_three_calorie_elfs = Vec::new(); //new vector array
    input.iter().for_each(|i| {
        // for every elf in the passed in vector,
        let mut calories = 0; // have a caloric counter
        for j in &i.1 {
            calories += j;
        }
        top_three_calorie_elfs.push((i.0.clone(), calories)); // add the elf and their calories to the vector, to keep track of each of them
    });
    top_three_calorie_elfs.sort_by(|a, b| b.1.cmp(&a.1)); // sort the vector by the calories
    top_three_calorie_elfs.truncate(3); // truncate the vector to only the top 3
    top_three_calorie_elfs // return the vector
}