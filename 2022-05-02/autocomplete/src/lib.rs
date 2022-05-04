
pub fn autocomplete<'a>(dict: Vec<&'a str>, input_str: &'a str) -> Vec<&'a str> {
    dict.into_iter()
        .filter(|word| word.contains(input_str))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_autocomplete() {
        let dict = vec!["apple", "banana", "cranberry", "strawberry"];
        assert_eq!(autocomplete(dict, "app"), vec!["apple"]);
        let dict = vec!["apple", "banana", "cranberry", "strawberry"];
        assert_eq!(autocomplete(dict, "berry"), vec!["cranberry", "strawberry"]);
        let dict = vec!["apple", "banana", "cranberry", "strawberry"];
        assert_eq!(autocomplete(dict, "fart"), Vec::<&str>::new());
    }
}
