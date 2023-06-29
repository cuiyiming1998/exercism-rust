use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lowered_word = word.to_lowercase();
    let sorted_word = sorted_str(&lowered_word);

    possible_anagrams
        .iter()
        .filter(|&&possible_anagram| {
            let lowered_possible_anagram = possible_anagram.to_lowercase();
            lowered_word != lowered_possible_anagram
                && sorted_word == sorted_str(&lowered_possible_anagram)
        })
        .cloned()
        .collect()
}

fn sorted_str(word: &str) -> String {
    let mut word: Vec<char> = word.chars().collect();
    word.sort();

    word.into_iter().collect()
}
