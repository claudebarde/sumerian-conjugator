pub fn starts_with_consonant(s: &str) -> bool {
    let consonants = vec![
        'b', 'd', 'f', 'g', 'ĝ', 'h', 'k', 'l', 'm', 'n', 'r', 's', 'š', 't', 'z',
    ];
    if let Some(first_char) = s.chars().next() {
        consonants.contains(&first_char)
    } else {
        false // Return false if the string is empty
    }
}
