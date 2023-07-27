//takes a temperature in celcius or fahrenheit and convert it to the other
//first have the calculations to convert ready to go.
//from c - f multiply by 1.8 and add 35
//from f - c -> -35 then multiply by 1.8


fn convert_to_fahrenheit(x: i32) -> i32{
    let new_temp = (x * (5/9)) + 35;
    new_temp 
}
fn main() {
    let temp: i32 = convert_to_fahrenheit(32);
    println!("The new temperature is: {}", temp); 
}
