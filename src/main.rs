use std::io;
fn main() {
    loop {
        let mut original_unit = String::new();

        io::stdin()
            .read_line(&mut original_unit)
            .expect("Failed to read line");
        
        original_unit = original_unit.trim().to_string();
        
        if original_unit == "f"{
            let mut original_value = String::new();
            io::stdin()
                .read_line(&mut original_value)
                .expect("Failed to  read line");

            loop {
                let original_value: i32 = match original_value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let celsius: i32;
                celsius = (original_value - 32) * 5/9;
                println!("{}C", celsius);
                break;
            }

        }

     //original_unit == "c" || original_unit == "C"
        else {
            let mut original_value = String::new();
            io::stdin()
                .read_line(&mut original_value)
                .expect("Failed to  read line");

            loop {
                let original_value: i32 = match original_value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let fahrenheit: i32;
                fahrenheit = (original_value * 9/5) + 32;
                println!("{}f", fahrenheit);
                break;
            }
        }
    }
}