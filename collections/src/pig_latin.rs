pub fn run() {
    let phrase = String::from("the quick brown fox jumped over the lazy log or something");
    let mut translated_phrase = String::new();

    for word in phrase.split_whitespace() {
        let mut translated = String::new();
        let mut previous_letter = None;
        for (ind, letter) in word.chars().enumerate() {
            if is_vowel(letter) {
                if letter == 'u' && previous_letter == Some('q') {
                    continue;
                }
                translated = format!("{}-{}ay ", &word[ind..], &word[..ind]);
                break;
            }
            previous_letter = Some(letter);
        }
        translated_phrase.push_str(&translated);
    }

    println!("{}", translated_phrase.trim());
}

fn is_vowel(letter: char) -> bool {
    match letter {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        'y' => true,
        _ => false,
    }
}
