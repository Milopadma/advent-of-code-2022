// Advent of Code Day 7

// disable the snake case warnings
#![allow(non_snake_case)]

// disable mut warnings
#![allow(unused_mut)]

fn main() {
    // load up input.txt
    let rawInput = std::fs::read_to_string("input.txt").unwrap();

    // create a new vector where every \n indicates a new element
    let mut splittedInput: Vec<&str> = rawInput.split("\n").collect();

    // println
    println!("{:?}", splittedInput);

    // stage 1: get all the Files and their sizes in a vector
    let mut fileVec: Vec<File> = fileFinder(&splittedInput);
    println!("{:?}", fileVec);

    // need to find the total size of directories with a total size of 100000, and summed of their total sizes
    let mut sumOfDir100ksize: i32 = sumOfDir(&splittedInput, 100000);

    // println
    println!("Sum of directories with a size of 100000: {}", sumOfDir100ksize);
}

// a struct of filename, filesize, and filedir
// for easier debugging
#[derive(Debug)]
struct File {
    filename: String,
    filesize: i32,
    filedir: String,
}

// function that goes through the whole vector and finds the strings with numbers and returns a filevec struct instance
fn fileFinder<'a>(splitted_input: &'a Vec<&'a str>) -> Vec<File> {
    // create a new instance of vector file
    let mut vectorOfFiles: Vec<File> = Vec::new();

    // to find out where in the directory we are
    let mut currentDir: String = String::new();

    // loop through every element in the vector
    for x in splitted_input {
        // early returns to weed out non-files
        if
            x.contains("ls") ||
            x.contains("dir") ||
            x.contains("..")
        {
            //println
            println!("skipping: {}", x);
            // move to the next x
            continue;
        }

        // init a new filevec struct instance
        let mut aSingularFile = File {
            filename: String::new(),
            filesize: 0,
            filedir: String::new(),
        };

        // if the string has "$ cd" in it
        if x.contains("$ cd") {
            // this means we moved into a new directory and need to update the currentDir, by taking it from the string
            let mut temp: Vec<&str> = x.split(" ").collect();
            currentDir = temp[2].to_string();
            //println
            println!("currentDir: {}", currentDir);
        }

        // if the string has a number in it
        if x.contains(char::is_numeric) {
            // split the string into a vector of strings where every space is a new element
            let mut temp: Vec<&str> = x.split(" ").collect();
            // find the filename, and concat it to the filename string
            aSingularFile.filename = aSingularFile.filename.to_string() + &temp[1].to_string();
            // find the filesize, and concat it to the filesize string
            aSingularFile.filesize = temp[0].parse::<i32>().unwrap();
            // find the filedir, and concat it to the filedir string
            aSingularFile.filedir = currentDir.to_string();
        }

        // add this singular file to the vector of files
        vectorOfFiles.push(aSingularFile);
    }
    return vectorOfFiles;
}

fn sumOfDir(splitted_input: &Vec<&str>, arg: i32) -> i32 {
    let mut sumOfDir100ksize: i32 = 0;

    // return the sumOfDir
    return sumOfDir100ksize;
}