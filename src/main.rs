use std::fs;


fn main() {
    let result_day_one = day_one();
    // let result_day_two: i32 = day_two();
    let day_two_result: i32 = day_two_try_two();
    println!("Day one answer: {:?}", result_day_one);
    println!("Day two answer: {:?}", day_two_result);

}

fn read_file_ret_vec() -> String {
    let contents = fs::read_to_string("src/calories.txt")
    .expect("The file is not readable.");

    return contents;
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

fn string_loop<'a>(s: Vec<&'a str>,top_3: &'a mut [i32]) -> &'a mut [i32] {
    // loop through the vector passed in and split each element into a vector of ints
    for i in s {
        let this_little_list: Vec<i32> = i.split("\n").filter_map(|x: &str| x.parse::<i32>().ok()).collect();


        let this_sum: i32 = this_little_list.iter().sum();
        println!("this sum{:?}", this_sum);
        for j in &mut *top_3 {
            if this_sum > *j {
                *j = this_sum;
                break;
            }
        }

        // println!("top 3: {:?}", top_3)
    }
    return top_3
}

fn sum_array(a:&mut [i32]) -> i32 {

    let mut this_sum = 0;
    for i in a.iter() {
        this_sum += i;
    }
    return this_sum
}

fn day_two() -> i32 {
    let file_contents = read_file_ret_vec();
    let content_vec: Vec<_> = file_contents.split("\n\n").collect();
    let mut top_three = [0, 0, 0];
    let mut new_three = string_loop(content_vec, &mut top_three);
    let result = sum_array(&mut new_three);

    println!("new three: {:?}", new_three);
    return result
}

fn day_two_try_two() -> i32 {

    // loop through the contents like before
    let file_contents: String = read_file_ret_vec();
    let content_vec: Vec<_> = file_contents.split("\n\n").collect();

    let mut numbers_vec: Vec<i32> = Vec::new();

    for i in content_vec {
        let this_little_list: Vec<i32> = i.split("\n").filter_map(|x: &str| x.parse::<i32>().ok()).collect();
        let this_sum: i32 = this_little_list.iter().sum();
        numbers_vec.push(this_sum);
    }

    numbers_vec.sort();
    numbers_vec.reverse();

    let top_three_sum = numbers_vec[0] + numbers_vec[1] + numbers_vec[2];

    return top_three_sum
}
