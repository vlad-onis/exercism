// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    if note.len() > magazine.len() {
        return false;
    }

    let mut magazine_word_frequency: HashMap<&str, i32> = HashMap::new(); 
    let mut note_word_frequency: HashMap<&str, i32> = HashMap::new(); 

   for word in magazine {
        *magazine_word_frequency.entry(*word).or_insert(0) += 1;
    }

    for word in note {
        *note_word_frequency.entry(*word).or_insert(0) += 1;
    }

    note_word_frequency.iter()
        .all(|(k,v)| magazine_word_frequency.get(*k).unwrap_or(&0) >= v)

}
