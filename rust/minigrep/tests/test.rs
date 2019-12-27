#[cfg(test)]
mod tests {
    #[test]
    fn case_sensitive() {
        let key = "duct";
        let contents="\
Rust:
safe, fast, productive.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], minigrep::search(key, contents));
    }
    #[test]
    fn case_insensitive() {
        let key = "Duct";
        let contents="\
Rust:
safe, fast, productive.";
        assert_eq!(vec!["safe, fast, productive."], minigrep::search_case_insensitive(key, contents));
    }
}