use std::io;

fn main() {
    println!("Please input the temperature:");

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    temperature = temperature.trim().to_uppercase();
    let unit = temperature.chars().last().unwrap();

    temperature.pop();
    let temperature: f64 = temperature.parse().expect("Please enter a number!");

    if unit == 'F' {
        let celsius = (temperature - 32.) * 5./9.;
        println!("{temperature}F° = {celsius}C°"); 
    } else if unit == 'C' {
        let fahrenheit = (temperature * 9./5.) + 32.;
        println!("{temperature}C° = {fahrenheit}F°");
    } else {
        println!("Please add a proper unit (F/C) to the temperature!");
    }
}
