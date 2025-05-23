const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";

const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
// conver subs in vector

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// input string
    #[clap(long)]
    slug_in: Option<String>,
}

fn main() {
    let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
    let subs_o_vec: Vec<char> = SUBS_O.chars().collect();

    let to_slug = "b";
    let args = Args::parse();

    let slugified = if let Some(slug) = args.slug_in {
        slugify(&slug, &subs_i_vec, &subs_o_vec)
    } else {
        slugify(to_slug, &subs_i_vec, &subs_o_vec)
    };
    println!("{}", slugified);
}

fn slugify(s: &str, subs_i: &[char], subs_o: &[char]) -> String {
    let mut prev_dash = false;
    let mut result = String::new();


    for c in s.chars().map(|c| conv(c, &subs_i, &subs_o)) {
        if c != '-' {
            result.push(c);
            prev_dash=false;
        } else {
            if !prev_dash {
                result.push('-');
                prev_dash= true;
            }
        }
    }

    if result.len() > 1 && result.ends_with('-') {
        result.pop();
    }
    result
}

fn conv(c: char, subs_i: &[char], subs_o: &[char]) -> char {
    if let Some(index) = subs_i.iter().position(|&x| x == c) {
        subs_o[index]
    } else if c.is_ascii_alphabetic() {
        c.to_ascii_lowercase()
    } else {
        '-'
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // i. conversione lettera accentata
    #[test]
    fn test_accented_letter() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("à", &subs_i_vec, &subs_o_vec), "a");
    }

    // ii. conversione lettera non accentata
    #[test]
    fn test_non_accented_letter() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("b", &subs_i_vec, &subs_o_vec), "b");
    }

    // iii. conversione lettera non ammessa sconosciuta
    #[test]
    fn test_unknown_invalid_letter() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("@", &subs_i_vec, &subs_o_vec), "-");
    }

    // iv. conversione lettera accentata non compresa nella lista (es ῶ)
    #[test]
    fn test_accented_letter_not_in_list() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("ῶ", &subs_i_vec, &subs_o_vec), "-");
    }

    // v. stringa con più di una parola separata da spazio
    #[test]
    fn test_multiple_words_with_space() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("ciao mondo", &subs_i_vec, &subs_o_vec), "ciao-mondo");
    }

    // vi. stringa con caratteri accentati
    #[test]
    fn test_string_with_accented_chars() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("città già", &subs_i_vec, &subs_o_vec), "citta-gia");
    }

    // vii. stringa vuota
    #[test]
    fn test_empty_string() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("", &subs_i_vec, &subs_o_vec), "");
    }

    // viii. stringa con più spazi consecutivi
    #[test]
    fn test_multiple_consecutive_spaces() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("ciao   mondo", &subs_i_vec, &subs_o_vec), "ciao-mondo");
    }

    // ix. stringa con più caratteri non validi consecutivi
    #[test]
    fn test_multiple_invalid_chars_consecutive() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("ciao@@@mondo", &subs_i_vec, &subs_o_vec), "ciao-mondo");
    }

    // x. stringa con solo caratteri non validi
    #[test]
    fn test_only_invalid_chars() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("@@@", &subs_i_vec, &subs_o_vec), "-");
    }

    // xi. stringa con spazio alla fine
    #[test]
    fn test_space_at_end() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("ciao ", &subs_i_vec, &subs_o_vec), "ciao");
    }

    // xii. stringa con più caratteri non validi consecutivi alla fine
    #[test]
    fn test_multiple_invalid_chars_at_end() {
        let subs_i_vec: Vec<char> = SUBS_I.chars().collect();
        let subs_o_vec: Vec<char> = SUBS_O.chars().collect();
        assert_eq!(slugify("ciao@@@", &subs_i_vec, &subs_o_vec), "ciao");
    }
}