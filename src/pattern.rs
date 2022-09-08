#![warn(missing_docs)]
//! Generate permutations via a pattern.

const UPPERCASE: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const LOWERCASE: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const NUMBERS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const SPECIAL: [char; 33] = ['!', '\"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', ' '];


/// Generate permutations via a pattern.
/// 
///`,` for all uppercase letters (A B C D E F G H I J K L M N O P Q R S T U V W X Y Z)
/// 
///`@` for all lowercase letters (a b c d e f g h i j k l m n o p q r s t u v w x y z)
/// 
///`%` for all numeric characters (0 1 2 3 4 5 6 7 8 9)
/// 
///`^` for all special characters ( ! " # $ % ( ) * + , ' - . / : ; < = > ? @ \[ \\ \] ^ _ \` { | } ~ ` ` )
/// ### Example
/// ```
/// for s in from_pattern(vec!['a', '%', '@'], None) {
///     println!("{}", s);
/// }
/// ```
/// Note that `None` should always be present, even though it is not used.
/// 
/// It is a necessary component for permutation generation.
/// 
/// Also note that this function differs in its output from the other functions in this crate.
pub fn from_pattern(elements: Vec<char>, index: Option<usize>) -> Vec<String> {
    
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
        _ => curr.chars().collect(),
    };

    if index == args.len() - 1 {
        return this_permutation.iter().map(|x| String::from(*x)).collect();
    }

    let mut l = vec![];

    for i in this_permutation {
        for j in from_pattern(args.chars().collect::<Vec<char>>(), Some(index + 1)) {
            let mut tmp = String::from(i);
            tmp.push_str(&j);
            l.push(tmp);
        }
    }
    l
}
