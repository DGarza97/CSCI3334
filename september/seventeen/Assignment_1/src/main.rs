
fn fahrenheit_to_celsius(f: f64) -> f64
{
    (f-32.0)*(5.0/9.0)
}
fn celsius_to_fahrenheit(c: f64) -> f64
{
    ((c*9.0)/5.0) + 32.0
}
fn assignment_1()
{
    println!("\nAssingment One: \n");

    //Here I Declare a constant for the freezing point of water in Fahrenheit (32°F).
    const WATER_FREEZING_POINT: f64 = 32.0;

    //Here I Declare a mutable variable with a temperature in Fahrenheit
    let mut fahrenheit:f64 = 33.0;

    //I use the 2nd function just to avoid the warning of not using it
    celsius_to_fahrenheit(fahrenheit_to_celsius(WATER_FREEZING_POINT));
    
    //I start with using the constant to not get a warning
    println!("Converting Fahrenheit: {}° to Celsius: {}°", WATER_FREEZING_POINT,((fahrenheit_to_celsius(WATER_FREEZING_POINT))* 100.0).round() / 100.0);
    
    //Here I Use a loop to convert and print the next 5 integer temperatures
    while fahrenheit < 38.0
    {
        //I print the results of the conversion
        println!("Converting Fahrenheit: {}° to Celsius: {}°", fahrenheit,((fahrenheit_to_celsius(fahrenheit))* 100.0).round() / 100.0);
        //I update the  variable
        fahrenheit = fahrenheit + 1.0;
    }
}
fn assignment_2()
{
    println!("\n Assignment 2: \n");

    //Here I Create an array of 10 integer numbers of your choice.
    let numbers = [1,2,3,4,5,6,7,8,9,10];
    
    //Here I Use a for loop to iterate through the array and for each number
    for n in 0..10
    {
        //If it's divisible by both 3 and 5, print "FizzBuzz"
        if numbers[n]%3 == 0 && n%5 == 0
        {
            println!("FizzBuzz");
        }
        //If the number is divisible by 3, print "Fizz" instead
        else if numbers[n]%3 == 0
        {
            println!("Fizz");
        }
        //If the number is divisible by 5, print "Buzz" instead
        else if numbers[n]%5 == 0
        {
            println!("Buzz");
        }
        //Print whether it's even or odd using your is_even function
        else
        {
            if is_even(numbers[n].try_into().unwrap())
            {
                println!("Even")
            }
            else
            {
                println!("Odd")
            }
        }
    }
    let mut x = 0;
    let mut y = 0;

    //Here I Use a while loop to find and print the sum of all numbers in the array.
    while y < 10
    {
        x = x + numbers[y];
        y = y + 1;
    }
    println!("\nThe sum of all numbers in the array is: {}", x);
    
    //Use a loop to find and print the largest number in the array.
    for numb in 0..9
    {
        x = numbers[numb];
        if x > numbers[numb+1]
        {
            y = x;
        }
        else
        {
            y = numbers[numb+1]
        }
    }
    println!("The largets number is: {}\n", y)
}

//Here I Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise
fn is_even(n: i32) -> bool
{

    if n%2 == 0
    {
        return true
    }
    else
    {
        return false
    }
}
fn assignment_3()
{
    println!("\nAssingment Three: \n");
    let mut secret: i32 = 18;
    let mut guess: i32 = 15;
    let mut guesses: i32 = 0;
    let mut check: i32;
    secret = secret + 1;
    while guesses < 10
    {
        check = check_guess(guess, secret);
        if check == 0
        {
            println!("You guessed {} correctly! It took you {} guesses!\n", secret, guesses);
            break;
        }
        else if check > 0
        {
            println!("Your guess is too high at {}",guess);
        }
        else
        {
            println!("Your guess is too low at {}",guess);
        }
        guess = guess + 1;
        guesses = guesses + 1
    }
    secret = 3;
    guess = 8;
    guesses = 0;
    while guesses < 10
    {
        check = check_guess(guess, secret);
        if check == 0
        {
            println!("You guessed {} correctly! It took you {} guesses!\n", secret, guesses);
            break;
        }
        else if check > 0
        {
            println!("Your guess is too high at {}",guess);
        }
        else
        {
            println!("Your guess is too low at {}",guess);
        }
        guess = guess - 1;
        guesses = guesses + 1
    }
    if guesses == 10
    {
        println!("You're over the 9 total guesses, Try Again later!")
    }

}
fn check_guess(guess: i32, secret: i32) -> i32
{
    if guess == secret
    {
        return 0
    }
    else if guess > secret
    {
        return 1
    }
    else
    {
        return -1
    }
}
fn main() 
{
    assignment_1();
    assignment_2();
    assignment_3()

}
