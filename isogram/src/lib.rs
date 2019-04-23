use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    //unimplemented!("Is {} an isogram?", candidate);
    if candidate.len() == 0 {
        return true;
    }
    let mut word1 = candidate.to_lowercase();
    let mut word2 = candidate.to_lowercase();

    word1 = word1.replace(" ", "");
    word1 = word1.replace("-", "");
    word2 = word2.replace(" ", "");
    word2 = word2.replace("-", "");

    let mut set = HashSet::new();
    for i in word1.chars() {
        set.insert(i);
    }

    if set.len() == word2.len() {
        return true;
    } else {
        return false;
    }
}
