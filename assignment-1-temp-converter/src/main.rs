
const FREEZING_POINT_F: f64 = 32.0;


fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}


fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    println!("--- Assignment 1: Temperature Converter ---");
    

    let mut temp_f: f64 = 68.0; 
    
    println!("Initial Fahrenheit: {}°F", temp_f);

    // Convert to Celsius and print the result
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("Converted to Celsius: {:.2}°C", temp_c);

    println!("\n--- Next 5 Fahrenheit Conversions ---");
    let mut counter = 0;
    
    
    loop {
        if counter >= 5 {
            break;
        }

        temp_f += 1.0; 

        let next_c = fahrenheit_to_celsius(temp_f);
        println!("{}°F is {:.2}°C", temp_f, next_c);
        
        counter += 1;
    }
}