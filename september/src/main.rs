fn add(x:i32,y:i32) -> i32
{
    //return x+y; //statements have semicolons ;
    //last line of code is treated as an expression
    x + y
}

fn main() 
{
    let res = add(5,15);
    println!("{}", res);
}
