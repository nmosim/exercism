use itertools::Itertools;
use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation; 

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let is_anagram = |input: &str| {
        input.to_lowercase().graphemes(true).sorted().collect::<String>()
            == word.to_lowercase().graphemes(true).sorted().collect::<String>()
            && input.to_lowercase() != word.to_lowercase()
    };

    possible_anagrams
        .iter()
        .filter(|s| is_anagram(s))
        .cloned()
        .collect()
}
