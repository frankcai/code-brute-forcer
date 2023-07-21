use std::env;

// Function to generate all possible codes by replacing the missing chars
fn generate_codes(code: &str, replacements: &Vec<(char, Vec<char>)>) -> Vec<String> {
    let mut generated_codes: Vec<String> = vec![code.to_string()];

    //  Iterates through the replacements vector, replacing the target character in the code with each of its replacements.
    for &(target, ref replacements) in replacements {
        let mut new_generated_codes = Vec::new();

        for replacement in replacements {
            for code in &generated_codes {
                if code.contains(target) {
                    new_generated_codes.push(code.replacen(target, &replacement.to_string(), 1));
                }
            }
        }

        generated_codes = new_generated_codes;
    }

    println!("Total possible combinations: {}", generated_codes.len());
    generated_codes
}

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    // Check that the number of arguments is correct
    if args.len() < 3 {
        eprintln!("Usage: {} <code> <replacement>...", args[0]);
        std::process::exit(1);
    }

    // Parse the arguments
    let code = &args[1];
    let replacements_specs = &args[2..];

    // Prepare a vector to hold the replacements
    let mut replacements = Vec::new();
    // For each spec of replacements...
    for spec in replacements_specs {
        let parts: Vec<_> = spec.split(':').collect();
        // Check that the spec is valid
        if parts.len() != 2 || parts[1].is_empty() {
            eprintln!("Invalid replacement spec: {}", spec);
            std::process::exit(1);
        }

        // Add the target character and its replacements to the list
        let target = parts[0].chars().next().unwrap();
        let replacements_chars = parts[1].chars().collect();
        replacements.push((target, replacements_chars));
    }

    // Generate all possible codes
    let generated_codes = generate_codes(&code, &replacements);
    for gen_code in generated_codes {
        println!("{}", gen_code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_codes_single_char() {
        let replacements = vec![('?', vec!['1', '2'])];
        let mut generated_codes = generate_codes("abc?def", &replacements);
        let mut expected_codes = vec!["abc1def", "abc2def"];
        generated_codes.sort();
        expected_codes.sort();
        assert_eq!(generated_codes, expected_codes);
    }

    #[test]
    fn test_generate_codes_multiple_chars() {
        let replacements = vec![('?', vec!['1', '2']), ('*', vec!['3', '4'])];
        let mut generated_codes = generate_codes("abc?def*ghi", &replacements);
        let mut expected_codes = vec!["abc1def3ghi", "abc2def3ghi", "abc1def4ghi", "abc2def4ghi"];
        generated_codes.sort();
        expected_codes.sort();
        assert_eq!(generated_codes, expected_codes);
    }

    #[test]
    fn test_generate_codes_repeated_char() {
        let replacements = vec![('?', vec!['1', '2']), ('?', vec!['1', '2'])];
        let mut generated_codes = generate_codes("abc??def", &replacements);
        let mut expected_codes = vec!["abc11def", "abc12def", "abc21def", "abc22def"];
        generated_codes.sort();
        expected_codes.sort();
        assert_eq!(generated_codes, expected_codes);
    }

    #[test]
    fn test_generate_codes_repeated_char_different() {
        let replacements = vec![('?', vec!['a', 'b']), ('?', vec!['x', 'y'])];
        let mut generated_codes = generate_codes("443kop3?23?d", &replacements);
        let mut expected_codes = vec![
            "443kop3a23xd",
            "443kop3b23xd",
            "443kop3a23yd",
            "443kop3b23yd",
        ];
        generated_codes.sort();
        expected_codes.sort();
        assert_eq!(generated_codes, expected_codes);
    }

    #[test]
    fn test_generate_codes_no_missing_chars() {
        let replacements = vec![];
        let generated_codes = generate_codes("abcdef", &replacements);
        assert_eq!(generated_codes, vec!["abcdef"]);
    }
}
