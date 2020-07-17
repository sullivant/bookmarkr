extern crate percent_encoding;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_instagram_url(query: &str) -> String {
    if query == "ig" {
        let instagram_base = "https://instagram.com";
        instagram_base.to_string()
    } else if &query[..4] == "ig @" {
        // Try for a url
        construct_instagram_profile_url(&query[4..])
    } else {
        // Otherwise search twitter
        construct_instagram_search_url(&query[3..])
    }
}

pub fn construct_instagram_profile_url(profile: &str) -> String {
    format!("https://instagram.com/{}/", profile)
}

pub fn construct_instagram_search_url(query: &str) -> String {
    let encoded_query = utf8_percent_encode(query, FRAGMENT).to_string();
    let search_url = format!("https://instagram.com/explore/tags/{}", encoded_query);
    search_url
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_instagram_url() {
        let fake_query = "ig";
        assert_eq!(construct_instagram_url(fake_query), "https://instagram.com");
    }

    #[test]
    fn test_construct_instagram_url_query() {
        let fake_query = "ig hello world";
        assert_eq!(
            construct_instagram_url(fake_query),
            "https://instagram.com/explore/tags/hello%20world"
        );
    }

    #[test]
    fn test_construct_instagram_url_profile() {
        let fake_query = "ig @thomaslaneart";
        assert_eq!(
            construct_instagram_url(fake_query),
            "https://instagram.com/thomaslaneart/"
        );
    }

    #[test]
    fn test_construct_instagram_profile_url() {
        let fake_profile = "thomaslaneart";
        assert_eq!(
            construct_instagram_profile_url(fake_profile),
            "https://instagram.com/thomaslaneart/"
        );
    }

    #[test]
    fn test_construct_instagram_search_url() {
        let fake_query = "hello world";
        assert_eq!(
            construct_instagram_search_url(fake_query),
            "https://instagram.com/explore/tags/hello%20world"
        );
    }
}
