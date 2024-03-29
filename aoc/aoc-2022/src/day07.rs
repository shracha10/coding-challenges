static A_MOST: i32 = 100000;
static B_REQ: i32 = 30000000;
static B_MAX: i32 = 70000000;

use std::fs;

pub fn a (input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).expect("Unable to read file.");

    let mut lines = input.split_terminator('\n').rev().collect::<Vec<&str>>();
    lines.pop();                         // remove "$ cd \" line

    let mut output = 0;

    find_size_a(&mut lines, &mut output);

    output
}

fn find_size_a (lines: &mut Vec<&str>, sum: &mut i32) -> i32 {
    let mut size_of_dir = 0;
    lines.pop();                         // remove "$ ls" line

    loop {
        let mut line = match (*lines).pop() {
                      Some(inner) => inner.split(' '),
                      None => break,            // in the case of the final dir
        };

        let mut temp = line.next().unwrap();    // get first string in the line

        if temp.eq("$") {                 // check if the line is a command
            temp = line.last().unwrap();  // getting dir for cd command

            if temp.eq("..") {            // go up a dir
                break;
            } else {                      // explore new dir
                size_of_dir += find_size_a(lines, sum);
            }
        } else if temp.eq("dir") {        // dir, do nothing
        } else {                          // file size
            size_of_dir += temp.parse::<i32>().unwrap();
        }
    }

    if size_of_dir <= A_MOST {
        *sum += size_of_dir;
    }
    size_of_dir
}

pub fn b (input_file: &str) -> i32 {
    let input = fs::read_to_string(input_file).expect("Unable to read file.");

    let mut lines = input.split_terminator('\n').rev().collect::<Vec<&str>>();
    lines.pop();                         // remove "$ cd \" line

    let space_required = B_REQ - (B_MAX - find_size_a(&mut lines, &mut 0)); // finding total size of fs, do not care about the part a sum

    let mut output = B_MAX;

    // re-populating `lines` for a second pass to find the dir to remove
    lines = input.split_terminator('\n').rev().collect::<Vec<&str>>();
    lines.pop();                         // remove "$ cd \" line

    find_size_b(&mut lines, & space_required, &mut output);

    output
}

fn find_size_b (lines: &mut Vec<&str>, space_required: & i32, dir_to_remove: &mut i32) -> i32 {
    let mut size_of_dir = 0;
    lines.pop();                         // remove "$ ls" line

    loop {
        let mut line = match (*lines).pop() {
                      Some(inner) => inner.split(' '),
                      None => break,            // in the case of the final dir
        };

        let mut temp = line.next().unwrap();    // get first string in the line

        if temp.eq("$") {                 // check if the line is a command
            temp = line.last().unwrap();  // getting dir for cd command

            if temp.eq("..") {            // go up a dir
                break;
            } else {                      // explore new dir
                size_of_dir += find_size_b(lines, space_required, dir_to_remove);
            }
        } else if temp.eq("dir") {        // dir, do nothing
        } else {                          // file size
            size_of_dir += temp.parse::<i32>().unwrap();
        }

    }

    if *space_required <= size_of_dir && size_of_dir < *dir_to_remove {
        *dir_to_remove = size_of_dir;
    }

    size_of_dir
}
