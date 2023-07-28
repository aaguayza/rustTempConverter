use core::num;
//takes a temperature in celcius or fahrenheit and convert it to the other
//first have the calculations to convert ready to go.
//from c - f multiply by 1.8 and add 35
//from f - c -> -35 then multiply by 1.8
//We want user to input a number so that it can be converted
//give user the ability to choose which measurment to convert to
use std::io;


fn convert_to_fahrenheit(x: f32) -> f32{
    
    let new_temp:f32 = (x * 1.8) + 32.0;
    new_temp
}
fn convert_to_celcius(x: f32) -> f32{
    
    let new_temp:f32 = (x - 32.0) +0.5;
    new_temp
}

fn main() {
    println!("give a number to convert");
    let mut user_number = String::new();
        io::stdin()
            .read_line(&mut user_number)
            .expect("failed to readline");

//needs to be validated with a match statement 
    let user_number: f32 = user_number
        .trim()
        .parse()
        .expect("couldnt parse");
    println!("Your number: {}", user_number );


    
    let mut user_temp_preference:String;
      
    // while user_temp_preference != "F"{

    //     println!("Which converter would you like to use? F/C)");
    //         io::stdin()
    //         .read_line(&mut user_temp_preference)
    //         .expect("failed to readline");
    //     let user_temp_preference = user_temp_preference.trim();
    //     println!("{}", user_temp_preference == "F")
    // };
    loop{
        user_temp_preference = String::new();
        println!("Which converter would you like to use? (F/C)");
            io::stdin()
            .read_line(&mut user_temp_preference)
            .expect("failed to readline");
        let user_temp_preference = user_temp_preference.trim();

        if user_temp_preference.to_uppercase() == "F"{
            let temp_in_fahrenheit: f32 = convert_to_fahrenheit(user_number);
             println!("The temperature in fahrenheit: {} F", temp_in_fahrenheit); 
             break;

        }else if user_temp_preference.to_uppercase() == "C"{
            let temp_in_celcius: f32 = convert_to_celcius(user_number);
            println!("The temperature in celcius is: {} C", temp_in_celcius);
            break;

        }else{
            println!("Invalid input");
            continue;
        }
    }



    // let temp_in_fahrenheit: f32 = convert_to_fahrenheit(user_number);
    // println!("The temperature in fahrenheit: {}", temp_in_fahrenheit); 

    // let temp_in_celcius: f32 = convert_to_celcius(user_number);
    // println!("The temperature in celcius is: {}", temp_in_celcius);
}