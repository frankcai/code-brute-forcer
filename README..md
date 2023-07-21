# Code Brute Forcer

A tool that uses a brute-force approach to find possible codes from a given code where some characters are missing and are replaced with a placeholder character. Each placeholder character can have a different set of possible replacements.

One day I needed to register two codes - one from a pack of Pampers and another from a book to get the e-book version. However, the fonts used to print these codes were printed such that that some characters of the code were ineligible. As a solution, I wrote this tool to brute force all the possible codes and guessed the correct codes within a few minutes. 

## Prerequisites

To build and run this program, you need to have Rust installed. If you haven't already installed Rust, you can download it from [the official website](https://www.rust-lang.org/tools/install).

## Usage

To build the program, navigate to its directory and use the command:

```bash
cargo build --release
```

To run the program, use the command:

```bash
cargo run "<code>" "<target_char>:<possible_chars>"...
```

`<code>` is the code with missing characters.

`<target_char>:<possible_chars>` is a pair where `<target_char>` is a placeholder character in the code and `<possible_chars>` is a string of characters that the placeholder can be replaced with.

You can specify multiple target_char-possible_chars pairs.

Example:

```bash
cargo run "443kop3?23?d" "?:ab" "?:xy"
```

This will fill the missing characters in the code ("443kop3?23?d") with the possible characters for each target character (first '?' with 'a', 'b', second '?' with 'x' and 'y').

## Limitations

This program uses a brute-force approach to generate all possible codes. Therefore, it may not be suitable for codes with many missing characters or a large number of possible replacements for each missing character.

It is highly recommended to constrain the search space as much as possible if you partially know the pattern to greatly reduce the amount of generated codes.

## License
This project is released under the MIT license. Check out the [LICENSE](LICENSE) file for more information.