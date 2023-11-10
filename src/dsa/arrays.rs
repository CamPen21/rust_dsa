// Implement an algorithm to determine if a string has alll unique characters
pub fn has_unique_characters(s: &str) -> bool {
    // M: O(n)
    let mut charset = [false; 128];
    // T: O(n)
    for c in s.chars() {
        let ascii_val = c as usize;
        if charset[ascii_val] {
            return false;
        }
        charset[ascii_val] = true;
    }
    true
}

// Check permutations
// Given two strings decide if one is a permutation of the other
pub fn is_permutation(m: &str, n: &str) -> bool {
    // This solution only works if m.len() == n.len()
    let mut m = Vec::from(m);
    let mut n = Vec::from(n);
    // O(m log m)
    m.sort();
    // O(n log n)
    n.sort();
    m == n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_unique_characters() {
        let strings = [("abcd", true), ("abcda", false)];
        for (string, result) in strings {
            assert_eq!(has_unique_characters(string), result)
        }
    }

    #[test]
    fn test_is_permutation() {
        let strings = [
            ("abcd", "cbad", true), 
            ("test", "stet", true),
            ("car", "bad", false),
        ];
        for (s1, s2, result) in strings {
            assert_eq!(is_permutation(s1, s2), result)
        }
    }
}
