use std::collections::HashSet;

pub fn letter_fcy(word: &str,) -> Vec<u32> {
    let mut v: [u32; 11000] = [0; 11000]; // tests use weird characters hence the size bigger than the alphabet
    for c in word.chars() {
        let c_ord = (c as usize) - 97;
        v[c_ord] += 1;
    }

    v.to_vec()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a[&str]) -> HashSet<&'a str> {
    
    let word = word.to_lowercase();
    let mut result: HashSet<&str> = HashSet::new();

    let word_fcy = letter_fcy(&word);
    

    

    for anagram in possible_anagrams {
        if word.len() != anagram.len() {
            continue;
        } else if word == anagram.to_lowercase() {
            continue;
        }

        let anagram_lower = anagram.to_lowercase();
        let anagram_fcy = letter_fcy(&anagram_lower);

        
        if anagram_fcy == word_fcy {
            result.insert(&anagram);
        }
    }

    result
}
