#![warn(missing_docs)]
//! Generate permutations via a pattern.

const UPPERCASE: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const LOWERCASE: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL: [char; 33] = ['!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', ' '];
const ALL: [char; 95] = ['!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', ' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

/// Generate permutations via a pattern.
/// 
///`,` for all uppercase letters (A B C D E F G H I J K L M N O P Q R S T U V W X Y Z)
/// 
///`@` for all lowercase letters (a b c d e f g h i j k l m n o p q r s t u v w x y z)
/// 
///`%` for all numeric characters (0 1 2 3 4 5 6 7 8 9)
/// 
///`^` for all special characters ( ! " # $ % ( ) * + , ' - . / : ; < = > ? @ \[ \\ \] ^ _ \` { | } ~ ` ` )
/// 
///`!` for all uppercase, lowercase, numeric & special characters as listed above
/// ### Example
/// ```
/// use pincer::pattern::from_pattern;
/// for s in from_pattern(vec!['a', '%', '@']) {
///     println!("{}", s); // prints a0a, a0b ... a1a, a1b ... a9z
/// }
/// ```
pub fn from_pattern(elements: Vec<char>) -> Vec<String> {
    
    let args: String = elements.iter().collect();
    let index: usize = 0;
    if index == args.len() {
        return vec![];
    }
    
    let curr = &args[index..=index];
    let this_permutation = match curr {
        "," => UPPERCASE.to_vec(),
        "@" => LOWERCASE.to_vec(),
        "%" => NUMBERS.to_vec(),
        "^" => SPECIAL.to_vec(),
        "!" => ALL.to_vec(),
        _ => curr.chars().collect(),
    };

    if index == args.len() - 1 {
        return this_permutation.iter().map(|x| String::from(*x)).collect();
    }

    let mut l = vec![];

    for i in this_permutation {
        for j in from_pattern_private(args.chars().collect::<Vec<char>>(), Some(index + 1)) {
            let mut tmp = String::from(i);
            tmp.push_str(&j);
            l.push(tmp);
        }
    }
    l
}

/// Removes need for index parameter in `from_pattern`.
fn from_pattern_private(elements: Vec<char>, index: Option<usize>) -> Vec<String> {
    let args: String = elements.iter().collect();
    let index: usize = index.unwrap_or(0);
    if index == args.len() {
        return vec![];
    }
    
    let curr = &args[index..=index];
    let this_permutation = match curr {
        "," => UPPERCASE.to_vec(),
        "@" => LOWERCASE.to_vec(),
        "%" => NUMBERS.to_vec(),
        "^" => SPECIAL.to_vec(),
        "!" => ALL.to_vec(),
        _ => curr.chars().collect(),
    };

    if index == args.len() - 1 {
        return this_permutation.iter().map(|x| String::from(*x)).collect();
    }

    let mut l = vec![];

    for i in this_permutation {
        for j in from_pattern_private(args.chars().collect::<Vec<char>>(), Some(index + 1)) {
            let mut tmp = String::from(i);
            tmp.push_str(&j);
            l.push(tmp);
        }
    }
    l
}

/// Generate permutations via a custom pattern.
/// 
/// Uses `*` as a wildcard character.
/// 
/// ### Example
/// ```
/// use pincer::pattern::from_custom_pattern;
/// for s in from_custom_pattern(vec!['A', '*'], vec!['A', 'B', 'C']) {
///     println!("{}", s); // prints AA, AB, AC
/// }
/// ```
pub fn from_custom_pattern(elements: Vec<char>, custom_pattern: Vec<char>) -> Vec<String> {
    let args: String = elements.iter().collect();
    let cpat: String = custom_pattern.iter().collect();
    let index: usize = 0;
    if index == args.len() {
        return vec![];
    }
    
    let curr = &args[index..=index];
    let this_permutation = match curr {
        "*" => custom_pattern, 
        _ => curr.chars().collect(),
    };

    if index == args.len() - 1 {
        return this_permutation.iter().map(|x| String::from(*x)).collect();
    }

    let mut l = vec![];

    for i in this_permutation {
        for j in from_custom_pattern_private(args.chars().collect::<Vec<char>>(), cpat.chars().collect::<Vec<char>>(), Some(index + 1)) {
            let mut tmp = String::from(i);
            tmp.push_str(&j);
            l.push(tmp);
        }
    }
    l
}

fn from_custom_pattern_private(elements: Vec<char>, custom_pattern: Vec<char>, index: Option<usize>) -> Vec<String> {
    let args: String = elements.iter().collect();
    let cpat: String = custom_pattern.iter().collect();
    let index: usize = index.unwrap_or(0);
    if index == args.len() {
        return vec![];
    }
    
    let curr = &args[index..=index];
    let this_permutation = match curr {
        "*" => custom_pattern, 
        _ => curr.chars().collect(),
    };

    if index == args.len() - 1 {
        return this_permutation.iter().map(|x| String::from(*x)).collect();
    }

    let mut l = vec![];

    for i in this_permutation {
        for j in from_custom_pattern_private(args.chars().collect::<Vec<char>>(), cpat.chars().collect::<Vec<char>>(), Some(index + 1)) {
            let mut tmp = String::from(i);
            tmp.push_str(&j);
            l.push(tmp);
        }
    }
    l
}