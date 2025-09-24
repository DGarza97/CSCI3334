// fn add(x:i32,y:i32) -> i32
// {
//     //return x+y; //statements have semicolons ;
//     //last line of code is treated as an expression
//     x + y
// }
// fn sub(x:i32,y:i32) -> u8// 0-255
// {
//     (x-y).try_into().unwrap()
// }
// fn main() 
// {
//     let res = add(5,15);
//     println!("{}", res);
//     let res = sub(25,15);
//     println!("{}", res);
// }
// fn pattern_match_simple(num:u32) -> String 
// {
//     match(num%3==0, num%5==0)
//     {
//         (true, true)    => "FizzBuzz".to_string(),
//         (true, false)   => "Fizz".to_string(),
//         (false, true)   => "Buzz".to_string(),
//         (false, false)  => num.to_string(),
//     }
// }
fn palindrome(mut x:i32) -> bool
{
    if x<0
    {
        return false;
    }
    let old_num = x;
    let mut new_num = 0;
    while x > 0
    {
       let last_digit = x % 10;
       new_num = new_num*10 + last_digit;
       x = x / 10;
    }
    if old_num == new_num
    {
        return true;
    }    
    else
    {
        return false;
    }
}
fn main ()
{
    // for num in 1..10
    // {
    //     println!("{}", pattern_match_simple(num));
    // }
    let x = 12234321;
    println!("Is {} a palindrome? {}", x, palindrome(x))
}
