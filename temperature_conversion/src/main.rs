use std::io;

fn main() {
    println!("##################################");
    println!("Fahrenheit/Celsius conversion tool");
    println!("##################################");
    print_menu();
}

fn print_menu() {
    println!("Please enter temperature type (F/C): ");
    let mut temp_type = String::new();

    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line");

    let temp_type = temp_type.trim();

    if temp_type == "C" {
        println!("Please enter Celsius temperature: ");
        let temp = get_temperature();
        println!("Fahrenheit equivalent is {:.2}", (temp * 5.0 / 9.0) + 32.0)
    } else if temp_type == "F" {
        println!("Please enter Fahrenheit temperature: ");
        let temp = get_temperature();
        println!("Celsius equivalent is {:.2}", (temp - 32.0) * 9.0 / 5.0)
    } else {
        println!("{temp_type} was not 'F' or 'C'. Please try again.");
        print_menu();
    }
}

fn get_temperature() -> f64 {
    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = match temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{} is not a number. Please try again", temperature.trim());
            get_temperature()
        }
    };
    temperature
}
