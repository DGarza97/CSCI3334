fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let nums = [12, 5, 9, 15, 22, 30, 3, 8, 11, 25];


    for &num in nums.iter() {


        if num % 3 == 0 && num % 5 == 0 {
            println!("{num}: FizzBuzz");
        } else if num % 3 == 0 {
            println!("{num}: Fizz");
        } else if num % 5 == 0 {
            println!("{num}: Buzz");
        } else if is_even(num) {
            println!("{num}: Even");
        } else {
            println!("{num}: Odd");
        }
        
    }

    let mut sum = 0;
    let mut index = 0;
    while index < nums.len() {
        sum += nums[index];
        index += 1;
    }
    println!("\nSum: {sum}");

    let mut max = nums[0];
    for &num in nums.iter() {
        if num > max {
            max = num;
        }
    }
    println!("Max number: {max}");
}
