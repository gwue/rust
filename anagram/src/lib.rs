use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut matches = HashSet::new();
    let dist_orig = char_distribution(word);
    for anagram in possible_anagrams {
        if word.to_lowercase() != *anagram.to_lowercase() {
            let dist_ana = char_distribution(anagram);
            if dist_orig == dist_ana {
                matches.insert(*anagram);
            }
        }
    }

    matches
}

fn char_distribution(word: &str) -> HashMap<String, u32> {
    let mut dist = HashMap::new();
    for c in word.chars() {
        let lc = c.to_lowercase().to_string();

        let entry = dist.entry(lc).or_insert(0);
        *entry += 1;
    }
    dist
}
