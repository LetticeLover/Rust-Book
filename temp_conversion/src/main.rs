use std::io;

enum Scale {
    Kelvin,
    Celsius,
    Fahrenheit,
}

fn main() {
    let mut understood = false;
    let mut temp = 0.0;
    println!("Please input the temperature to convert.");
    while !understood {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

        temp = match input.trim().parse() {
            Ok(num) => { understood = true; num },
            Err(_) => { println!("Please try again."); continue },
        };
    }

    understood = false;
    let mut scale_from = Scale::Fahrenheit;
    println!("Please input the scale the temperature is in.");
    println!("Type \"K\" or \"k\" for Kelvin.");
    println!("Type \"C\" or \"c\" for Celsius.");
    println!("Type \"F\" or \"f\" for Fahrenheit.");
    while !understood {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        scale_from = match input.trim() {
            "k" | "K" => { understood = true; Scale::Kelvin },
            "c" | "C" => { understood = true; Scale::Celsius },
            "f" | "F" => { understood = true; Scale::Fahrenheit },
            _ => { println!("Please try again."); continue },
        };
    }
    understood = false;
    let mut scale_to: Scale = Scale::Fahrenheit;
    println!("Please input the scale you want the temperature in.");
    println!("Type \"K\" or \"k\" for Kelvin.");
    println!("Type \"C\" or \"c\" for Celsius.");
    println!("Type \"F\" or \"f\" for Fahrenheit.");
    while !understood {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        scale_to = match input.trim() {
            "k" | "K" => { understood = true; Scale::Kelvin },
            "c" | "C" => { understood = true; Scale::Celsius },
            "f" | "F" => { understood = true; Scale::Fahrenheit },
            _ => { println!("Please try again."); continue },
        };
    }
    println!("The converted temperature is {}.", convert(temp, scale_from, scale_to));
}

fn convert(temp: f64, scale_from: Scale, scale_to: Scale) -> f64 {
    match scale_from {
        Scale::Kelvin => convert_k(temp, scale_to),
        Scale::Celsius => convert_c(temp, scale_to),
        Scale::Fahrenheit => convert_f(temp, scale_to)
    }
}

fn convert_k(temp: f64, scale_to: Scale) -> f64 {
    match scale_to {
        Scale::Kelvin => temp,
        Scale::Celsius => temp - 273.15,
        Scale::Fahrenheit => ((temp - 273.15) * 9.0/5.0) + 32.0
    }
}

fn convert_c(temp: f64, scale_to: Scale) -> f64 {
    match scale_to {
        Scale::Kelvin => temp + 273.15,
        Scale::Celsius => temp,
        Scale::Fahrenheit => (temp * 9.0/5.0) + 32.0
    }
}

fn convert_f(temp: f64, scale_to: Scale) -> f64 {
    match scale_to {
        Scale::Kelvin => ((temp - 32.0) * 5.0/9.0) + 273.15,
        Scale::Celsius => (temp - 32.0) * 5.0/9.0,
        Scale::Fahrenheit => temp
    }
}