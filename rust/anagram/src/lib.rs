use std::collections::{HashSet, HashMap};


fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() { return false; };
    let ascii_1 = word1.to_lowercase();
    let ascii_2 = word2.to_lowercase();
    if ascii_1 == ascii_2 { return false; };
    let mut letter_mapper: HashMap<char, i32> = HashMap::new();
    for char_index in 0..word1.len() {
        let index_1 = ascii_1.chars().nth(char_index);
        let index_2 = ascii_2.chars().nth(char_index);
        if index_1.is_some() {
            letter_mapper.entry(index_1.unwrap()).and_modify(|x| *x += 1).or_insert(1);
        }
        if index_2.is_some() {
            letter_mapper.entry(index_2.unwrap()).and_modify(|x| *x -= 1).or_insert(-1);
        }
    }
    letter_mapper.values().all(|x| *x == 0)
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    for possible_word in possible_anagrams {
        if is_anagram(word, possible_word) {
            anagrams.insert(possible_word);
        };
        
    }
    anagrams
}
