
pub fn equal_with_deletions(string_one: &str, string_two: &str) -> bool {
    process_deletions(string_one).eq(&process_deletions(string_two))
}

fn process_deletions(string_value: &str) -> String {
    let mut processed_string = String::new();
    let mut forward_delete_count = 0;
    for c in string_value.chars() {
        match c {
            '#' => {processed_string.pop();},
            '%' => forward_delete_count = forward_delete_count + 1,
            _ if forward_delete_count > 0 => forward_delete_count = forward_delete_count - 1,
            _ => processed_string.push(c)
        }
    }
    processed_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_deletions() {
        assert_eq!(process_deletions("a##x"), "x");
        assert_eq!(process_deletions("#a#x"), "x");
        assert_eq!(process_deletions("fi##f%%%th %%year #time###"), "fart");
    }

    #[test]
    fn test_equal_with_deletions() {
        assert!(equal_with_deletions("a##x", "#a#x"));
        assert!(!equal_with_deletions("fi##f%%%th %%year #time###", "fifth year time"));
    }
}
