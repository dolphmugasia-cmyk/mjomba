//Exercise 1: convert temperature between fahrenheit to celsius and vice versa//
//Formula: C = (F - 32) * 5/9
//Formula: F = (C * 9/5) + 32
fn main() {
    // Convert Celsius to Fahrenheit
    let celsius = 24.9;
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{}°C is equal to {}°F", celsius, fahrenheit);

    //convert Fahrenheit to Celsius
    let fahrenheit = 88.6;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F is equal to {}°C", fahrenheit, celsius);

    //Trying more values
    let celsius = 47.7;
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{}°C is equal to {}°F", celsius, fahrenheit);


//convert Fahrenheit to Celsius
    let fahrenheit = 99.1;
    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{}°F is equal to {}°C", fahrenheit, celsius);




}
