pub fn starts_with_consonant(s: &str) -> bool {
    let consonants = vec![
        'b', 'd', 'f', 'g', 'ĝ', 'h', 'k', 'l', 'm', 'n', 'r', 'ř', 's', 'š', 't', 'z',
    ];
    if let Some(first_char) = s.chars().next() {
        consonants.contains(&first_char)
    } else {
        false // Return false if the string is empty
    }
}

pub fn ends_with_vowel(s: &str) -> bool {
    let vowels = vec!['a', 'e', 'i', 'u', 'è', 'é', 'á', 'ā', 'ē', 'ī', 'ū'];
    if let Some(last_char) = s.chars().last() {
        vowels.contains(&last_char)
    } else {
        false // Return false if the string is empty
    }
}
