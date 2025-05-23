// paste this file into main.rs

fn stats(text: &str) -> [u32; 26] {
    //count each the letter in the text. determine how many of each letter are present
    // create a vector of 26 elements, all initialized to 0
    let mut counts = [0; 26];
    // iterate over the text
    for c in text.chars() {
        // check if the character is a letter
        if c.is_ascii_alphabetic() {
            // convert to lowercase
            let c = c.to_ascii_lowercase();
            // get the index of the letter
            let index = (c as u8 - b'a') as usize;
            // increment the count
            counts[index] += 1;
        }
    }
     counts
}

fn is_pangram(counts: &[u32]) -> bool {
    //determine if pangram given the array that counts eaach letter
    // check if all letters are present
    if counts.len() != 26 {
        return false;
    }   
    for &count in counts.iter() {
        if count == 0 {
            return false;
        }
    }
    true

    // size it's predefined but mf  sent a wrong one
}

// call this function from main
// load here the contents of the file

pub fn run_pangram() {
    //from command line argument take the filename and read it
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file name as an argument.");
        return;
    }
    let filename = &args[1];

    // read the file
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    // call the stats function
    let counts = stats(&contents);

    // check if it is a pangram
    if is_pangram(&counts) {
       
        println!("The text is a pangram.");
        for (i, count) in counts.iter().enumerate() {
            let letter = (b'a' + i as u8) as char;
            println!("{} {:?}", letter, count);
        }
    } else {
        println!("The text is not a pangram.");
    }
}

// please note, code has been splittend in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests {
    // tests are separated modules, yuou must import the code you are testing
    use super::*;

    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 0;
        counts[1] = 0;
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size() {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }

    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);
    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

fn main() {
    run_pangram();
}
