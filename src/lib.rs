pub mod pincer;

#[cfg(test)]
mod tests {
    use crate::pincer;

    #[test]
    fn at_length_check() {
        let combinations = pincer::at_length(3, vec!['a', 'b', 'c']);
        assert!(combinations.eq(["aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc", "caa", "cab", "cac", "cba", "cbb", "cbc", "cca", "ccb", "ccc"].iter().map(|s| s.to_string())));
    }
    #[test]
    fn to_length_check() {
        let combinations = pincer::to_length(2, vec!['a', 'b', 'c']);
        assert!(combinations.eq(["a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"].iter().map(|s| s.to_string())));
    }
    #[test]
    fn from_minimum_check() {
        let combinations = pincer::from_minimum(2, 3, vec!['a', 'b', 'c']);
        assert!(combinations.eq(["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc", "caa", "cab", "cac", "cba", "cbb", "cbc", "cca", "ccb", "ccc"].iter().map(|s| s.to_string())));
    }
}
