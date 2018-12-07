use std::collections::HashMap;

fn letter_frequencies(s: &str) -> HashMap<char, u32> {
    let mut frequencies = HashMap::new();

    for c in s.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }

    frequencies
}

pub fn checksum(ids: &[String]) -> u32 {
    let mut doubles = 0;
    let mut triples = 0;

    for id in ids {
        let frequencies = letter_frequencies(id);

        if frequencies.values().any(|&v| v == 2) {
            doubles += 1;
        }

        if frequencies.values().any(|&v| v == 3) {
            triples += 1;
        }
    }

    doubles * triples
}

fn single_difference_at(id_a: &str, id_b: &str) -> Option<usize> {
    let mut differences = 0;
    let mut difference_at = 0;

    for (char_index, (char_a, char_b)) in id_a.chars().zip(id_b.chars()).enumerate() {
        if char_a != char_b {
            differences += 1;

            if differences > 1 {
                break;
            }

            difference_at = char_index;
        }
    }

    if differences == 1 {
        return Some(difference_at);
    }

    None
}

pub fn common_letters(ids: &[String]) -> String {
    for (index, id_a) in ids.iter().enumerate() {
        for id_b in ids.iter().skip(index + 1) {
            if let Some(difference_at) = single_difference_at(id_a, id_b) {
                return id_a
                    .chars()
                    .enumerate()
                    .filter(|&(index, _)| index != difference_at)
                    .map(|(_, c)| c)
                    .collect();
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_checksum() {
        let ids = [
            "abcdef".to_owned(),
            "bababc".to_owned(),
            "abbcde".to_owned(),
            "abcccd".to_owned(),
            "aabcdd".to_owned(),
            "abcdee".to_owned(),
            "ababab".to_owned(),
        ];

        assert_eq!(12, checksum(&ids));
    }

    #[test]
    fn sample_common_letters() {
        let ids = [
            "abcde".to_owned(),
            "fghij".to_owned(),
            "klmno".to_owned(),
            "pqrst".to_owned(),
            "fguij".to_owned(),
            "axcye".to_owned(),
            "wvxyz".to_owned(),
        ];

        assert_eq!("fgij", common_letters(&ids));
    }
}
