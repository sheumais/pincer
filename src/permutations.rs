#![warn(missing_docs)]
//! Generate permutations of a set of elements.

/// Generate an Iterator of all permutations of a set of elements.
/// 
/// ### Example
/// ```
/// for s in at_length(3, vec!['a', 'b', 'c']) {
///    println!("{}", s);
/// }
/// ```
#[allow(dead_code)]
pub fn at_length(len: u32, elements: Vec<char>) -> impl Iterator<Item = String> {
    let count = elements.len().pow(len);
    (0..count).map(move |iteration| {
        (0..len)
            .rev()
            .map(|decrease_from_length_linear| {
                let decrease_from_length_exponential = elements.len().pow(decrease_from_length_linear);
                let char_selected = (iteration / decrease_from_length_exponential) % elements.len();
                elements[char_selected]
            })
            .collect()
    })
}

/// Generate an Iterator of all permutations of a set of elements up to a given length.
/// 
/// ### Example
/// ```
/// for s in to_length(3, vec!['a', 'b', 'c']) {
///    println!("{}", s);
/// }
/// ```
#[allow(dead_code)]
pub fn to_length(len: u32, elements: Vec<char>) -> impl Iterator<Item = String> {
    (1..=len).flat_map(move |len| at_length(len, elements.clone()))
}

/// Generate an Iterator of all permutations of a set of elements from a minimum length up to a maximum length.
/// 
/// ### Example
/// ```
/// for s in from_minimum(2, 3, vec!['a', 'b', 'c']) {
///    println!("{}", s);
/// }
/// ```
#[allow(dead_code)]
pub fn from_minimum(min: u32, max: u32, elements: Vec<char>) -> impl Iterator<Item = String> {
    (min..=max).flat_map(move |max| at_length(max, elements.clone()))
}