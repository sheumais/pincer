#[cfg(test)]
mod tests {
    use pincer::permutations;
    use pincer::pattern;

    #[test]
    fn at_length_check() {
        let combinations = permutations::at_length(3, vec!['a', 'b', 'c']);
        assert!(combinations.eq(["aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc", "caa", "cab", "cac", "cba", "cbb", "cbc", "cca", "ccb", "ccc"].iter().map(|s| s.to_string())));
    }
    #[test]
    fn to_length_check() {
        let combinations = permutations::to_length(2, vec!['a', 'b', 'c']);
        assert!(combinations.eq(["a", "b", "c", "aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc"].iter().map(|s| s.to_string())));
    }
    #[test]
    fn from_minimum_check() {
        let combinations = permutations::from_minimum(2, 3, vec!['a', 'b', 'c']);
        assert!(combinations.eq(["aa", "ab", "ac", "ba", "bb", "bc", "ca", "cb", "cc", "aaa", "aab", "aac", "aba", "abb", "abc", "aca", "acb", "acc", "baa", "bab", "bac", "bba", "bbb", "bbc", "bca", "bcb", "bcc", "caa", "cab", "cac", "cba", "cbb", "cbc", "cca", "ccb", "ccc"].iter().map(|s| s.to_string())));
    }
    #[test]
    fn pattern_check_lowercase() {
        let combinations = pattern::from_pattern(vec!['@']);
        assert!(combinations.eq(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]));
    }
    #[test]
    fn pattern_check_uppercase() {
        let combinations = pattern::from_pattern(vec![',']);
        assert!(combinations.eq(&["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"]));
    }
    #[test]
    fn pattern_check_number() {
        let combinations = pattern::from_pattern(vec!['%']);
        assert!(combinations.eq(&["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]));
    }
    #[test]
    fn pattern_check_symbol() {
        let combinations = pattern::from_pattern(vec!['^']);
        assert!(combinations.eq(&["!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~", " "]));
    }
    #[test]
    fn pattern_check_lowercase_number() {
        let combinations = pattern::from_pattern(vec!['@', '%']);
        assert!(combinations.eq(&["a0", "a1", "a2", "a3", "a4", "a5", "a6", "a7", "a8", "a9", "b0", "b1", "b2", "b3", "b4", "b5", "b6", "b7", "b8", "b9", "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", "c9", "d0", "d1", "d2", "d3", "d4", "d5", "d6", "d7", "d8", "d9", "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7", "e8", "e9", "f0", "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "g0", "g1", "g2", "g3", "g4", "g5", "g6", "g7", "g8", "g9", "h0", "h1", "h2", "h3", "h4", "h5", "h6", "h7", "h8", "h9", "i0", "i1", "i2", "i3", "i4", "i5", "i6", "i7", "i8", "i9", "j0", "j1", "j2", "j3", "j4", "j5", "j6", "j7", "j8", "j9", "k0", "k1", "k2", "k3", "k4", "k5", "k6", "k7", "k8", "k9", "l0", "l1", "l2", "l3", "l4", "l5", "l6", "l7", "l8", "l9", "m0", "m1", "m2", "m3", "m4", "m5", "m6", "m7", "m8", "m9", "n0", "n1", "n2", "n3", "n4", "n5", "n6", "n7", "n8", "n9", "o0", "o1", "o2", "o3", "o4", "o5", "o6", "o7", "o8", "o9", "p0", "p1", "p2", "p3", "p4", "p5", "p6", "p7", "p8", "p9", "q0", "q1", "q2", "q3", "q4", "q5", "q6", "q7", "q8", "q9", "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7", "t8", "t9", "u0", "u1", "u2", "u3", "u4", "u5", "u6", "u7", "u8", "u9", "v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7", "v8", "v9", "w0", "w1", "w2", "w3", "w4", "w5", "w6", "w7", "w8", "w9", "x0", "x1", "x2", "x3", "x4", "x5", "x6", "x7", "x8", "x9", "y0", "y1", "y2", "y3", "y4", "y5", "y6", "y7", "y8", "y9", "z0", "z1", "z2", "z3", "z4", "z5", "z6", "z7", "z8", "z9"]));
    }
    #[test]
    fn pattern_check_number_number() {
        let combinations = pattern::from_pattern(vec!['%', '%']);
        assert!(combinations.eq(&["00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64", "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80", "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96", "97", "98", "99"]));
    }

    #[test]
    fn pattern_check_static(){
        let combinations = pattern::from_pattern(vec!['A', 'B', 'C']);
        assert!(combinations.eq(&["ABC"]));
    }

    #[test]
    fn pattern_check_static_number() {
        let combinations = pattern::from_pattern(vec!['A', '%']);
        assert!(combinations.eq(&["A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9"]));
    }

    #[test]
    fn pattern_check_all() {
        let combinations = pattern::from_pattern(vec!['!']);
        assert!(combinations.eq(&["!", "\"", "#", "$", "%", "&", "\'", "(", ")", "*", "+", ",", "-", ".", "/", ":", ";", "<", "=", ">", "?", "@", "[", "\\", "]", "^", "_", "`", "{", "|", "}", "~", " ", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"]));
    }

    #[test]
    fn pattern_custom_check_single() {
        let combinations = pattern::from_custom_pattern(vec!['A', '*'], vec!['A', 'B', 'C']);
        assert!(combinations.eq(&["AA", "AB", "AC"]));
    }
}
