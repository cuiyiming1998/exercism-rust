use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for &possible_anagram in possible_anagrams {
        if word.to_lowercase() != possible_anagram.to_lowercase()
            && sorted_str(word) == sorted_str(possible_anagram)
        {
            set.insert(possible_anagram);
        }
    }

    set
}

fn sorted_str(word: &str) -> String {
    let mut word: Vec<char> = word.to_lowercase().chars().collect();
    word.sort();

    word.into_iter().collect()
}
