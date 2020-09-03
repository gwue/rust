use std::collections::HashMap;
use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let matches = HashSet::new();
    let dist_orig = char_distribution(word);
    for anagram in possible_anagrams {
        if word != *anagram {
            let dist_ana = char_distribution(anagram);
            if dist_orig == dist_ana {
                matches.insert(*anagram);
            }
        }
    }

    matches
}

fn char_distribution(word: &str) -> HashMap<char, usize> {
    let mut dist = HashMap::new();
    for c in word.chars() {
        dist.insert(
            c,
            match dist.get(&c) {
                Some(&count) => count + 1,
                None => 1,
            },
        );
    }
    dist
}
