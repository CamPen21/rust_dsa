// Implement an algorithm to determine if a string has alll unique characters
pub fn has_unique_characters(s: &str) -> bool {
    // M: O(n)
    let mut charset = [false; 128];
    // T: O(n)
    for c in s.chars() {
        let ascii_val = c as usize;
        if charset[ascii_val] {
            return false
        }
        charset[ascii_val] = true;
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_has_unique_characters(){
        let strings = [
            ("abcd", true), 
            ("abcda", false)
        ];
        for (string, result) in strings {
            assert_eq!(has_unique_characters(string), result)
        }
    }
}
