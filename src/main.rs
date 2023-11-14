use std::io;
use str_to_pig::pigify;

fn main() {
    'outer: loop {

        println!("\nText to pigify:\n");
        // Taking text from input
        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read the line.");
        // Pigifying it
        let result = pigify(text);
        // Checking if any errors returned instead
        if result == "0".to_string() {
            println!("\nIt's empty or some words don't start with English letter!\n");
            continue 'outer;
        } else if result == "1".to_string() {
            println!("\nError, some words don't start with letters!\n");
            continue 'outer;
        } else if result == "2".to_string() {
            println!("\nThis error shouldn't happen, but just in case\n");
            continue 'outer;
        } else {
            println!("{result}");
            break 'outer;
        }
    }
}
