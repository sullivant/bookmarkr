//utils module

pub mod github;
pub mod google;
pub mod instagram;
pub mod twitter;

pub fn get_command_from_query_string(query_string: &str) -> &str {
    if query_string.contains(' ') {
        // determine where to slice the string
        let idx_of_space = query_string.find(' ').unwrap_or(0);

        // return the first portion of the string, as a slice
        return &query_string[..idx_of_space];
    }

    // otherwise return the query string
    query_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_command_from_query_string_no_whitespace() {
        let actual = get_command_from_query_string("tw");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_get_command_from_query_string_with_whitespace() {
        let actual = get_command_from_query_string("tw @thomaslaneart");
        let expected = "tw";
        assert_eq!(actual, expected);
    }
}
