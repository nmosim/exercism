use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let is_anagram = |input: &&str| {
        input
            .chars()
            .all(|other_word_char| word.chars().any(|word_char| word_char == other_word_char))
    };
    let anagrams = possible_anagrams.iter().filter(is_anagram);
    HashSet::from_iter(anagrams)
}
