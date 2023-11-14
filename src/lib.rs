pub fn pigify(text: String) -> String {
    // Vectors of vowels and consonants, with uppercase in them cause I didn't think about
    // making chars into lowercase and now I'm lazy to remove those big boys
    let vowels = vec!["a", "e", "i", "o", "u", "y", "A", "E", "I", "O", "U", "Y"];

    let conson = vec!["B", "C", "D",  "F",  "G", 
                                 "J", "K", "L", "M", "N",
                                 "P", "Q", "S", "T", "V",
                                 "X", "Z", "H", "R", "W", 
                                 "b", "c", "d", "f", "g", 
                                 "j", "k", "l", "m", "n", 
                                 "p", "q", "s", "t", "v", 
                                 "x", "z", "h", "r", "w" 
                                ];
    // Make a vector of string slices from passed string splited at spaces
    let words: Vec<&str> = text.trim().split(' ').collect();
    #[allow(unused)]
    // Variables to store results
    let mut piglat = String::new();
    let mut final_result = String::new();
    let mut error_case = String::new();
    // Loop over every word in vector from text
    for word in words {
        // Checking first character by getting string slice with first element only
        let char_check = match word.get(..1) {
            None => {
                // Update error in case of word not starting with valid UTF-8 character
                // or being empty, breaking off the loop
                error_case.push('0');
                break;
            },
            // Successfull value
            Some(char) => char,
        };
        // Simple check if vector of vowels contains value
        if vowels.contains(&char_check) {
            // Passing formated string to piglat
            piglat = format!("{}-hay ", word);
        //Same check for consonants
        } else if conson.contains(&char_check) {
            // Getting all but first characters in string
            let word_no_first = match word.get(1..) {
                Some(stroke) => stroke,
                None => {
                    // Should be checking if there's something to get
                    // It doesn't reach it with word.len() == 1 for some reason ? fix it later
                    error_case.push('2');
                    &error_case
                    // Maybe break isn't needed then?
                    
                },
            };
            // If there's only one character, format with first, else with second
            if word.len() < 2 {
                piglat = format!{"{}ay ", word};
            } else {
                piglat = format!{"{}-{}ay ", word_no_first, char_check.to_lowercase()};
            }
        // If first character neither vowel or consonant
        } else {
            // Error case for not being a word that starts with a letter
            error_case.push('1');
            break;
        };
    // Pushing formated string into final result for all words in text
    final_result.push_str(&piglat)
    
    }
    // Check if error case contains something
    if error_case.len() > 0 {
        // Return it if yes
        error_case
    // Else return final string
    } else {
        final_result
    }
    
}

#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;
    // Simple test to check if it formats string with length more than one
    #[test]

    fn works() {
        let some = "Mu".to_string();
        let result = pigify(some);

        println!("{result}");
    }
    #[test]
    // Test for error case 0 with empty string
    fn error_0() {
        let some = "".to_string();
        let result = pigify(some);

        assert_eq!(result, "0".to_string());
        print!("{result}");
    }
    // Test for error case 1 which is uncertain if needed
    #[test]

    fn error_2() {
        let some = "c".to_string();
        let result = pigify(some);

        assert_eq!(result, "2".to_string());
    }
    // Test for error case 2 with some words beginning not with letters
    #[test]

    fn error_1() {
        let some: String = "me and you 1 someone".to_string();
        let result = pigify(some);

        assert_eq!(result, "1".to_string());
        println!("{result}");
    }
    // Tests to see if handling errors in main will be succesfull
    #[test]
    fn handle_errors() {
        let some: String = "1 out of 10 people are in endless pain".to_string();
        let result = pigify(some);
        let two = "2".to_string();

        assert_eq!(result, two);

        match result {
           two => println!("No"),
           _ => println!("Wow"),

        };
    }

    #[test]

    fn handle_errors_2() {
        
        let some = String::new();
        let result = pigify(some);
        let zero = "0".to_string();

        assert_eq!(result, zero);
        
        match result {
            zero => println!("No_2"),
            _ => print!("hao"),
        };
    }
}
