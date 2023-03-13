use std::fs;


fn main() {
    let result_day_one = day_one();
    println!("Day one answer: {:?}", result_day_one);
}

fn day_one() -> i32 {

    let mut largest_sum = 0;
    // take the list of numbers as strings
    let contents = fs::read_to_string("src/calories.txt")
        .expect("The file is not readable.");

    // split the list into smaller lists by space
    let content_vec: Vec<&str> = contents.split("\n\n").collect();

    // loop through big list, summing each smaller list
    for i in content_vec {
        // convert i to inst
        let little_list: Vec<i32> = i.split("\n").filter_map(|x| x.parse::<i32>().ok()).collect();

        let this_sum = little_list.iter().sum();

        if this_sum >= largest_sum {
            largest_sum = this_sum;
        }

    }
    // keep track of the current largest sum of small lists
    // if the current sum is greater than than the current largest some, replace the largest sum

    // // return the largest sum
    // return largest_sum
    return largest_sum
}
