
fn fahrenheit_to_celsius(f: f64)
{
    let c = (f-32.0)*(5.0/9.0);
     println!("Converting Fahrenheit: {}° to Celsius: {}°", f,(c* 100.0).round() / 100.0);
}

fn main() 
{
    const water_freezing_point: f64 = 32.0;
    let mut fahrenheit:f64 = 32.0;
    for num in 0..6
    {
    fahrenheit = fahrenheit + num as f64;
    fahrenheit_to_celsius(fahrenheit);
    }
}
