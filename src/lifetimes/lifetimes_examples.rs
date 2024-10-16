pub fn next_language<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true
        }
    }

    languages.last().unwrap()
}

pub fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

pub fn longest_language<'a>(lang_1: &'a str, lang_2: &'a str) -> &'a str {
    if lang_1.len() >= lang_2.len() {
        lang_1
    } else {
        lang_2
    }
}
