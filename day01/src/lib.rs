use std::collections::HashSet;

pub fn resulting_frequency(changes: &[i32]) -> i32 {
    changes.iter().sum()
}

pub fn duplicate_frequency(changes: &[i32]) -> i32 {
    let mut frequencies = HashSet::new();
    let mut frequency = 0;

    frequencies.insert(frequency);

    for change in changes.iter().cycle() {
        frequency += change;

        if !frequencies.insert(frequency) {
            return frequency;
        }
    }

    frequency
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_resulting_frequency() {
        assert_eq!(3, resulting_frequency(&[1, -2, 3, 1]));
    }

    #[test]
    fn resulting_frequency_of_positive_changes() {
        assert_eq!(3, resulting_frequency(&[1, 1, 1]));
    }

    #[test]
    fn resulting_frequency_of_mixed_changes() {
        assert_eq!(0, resulting_frequency(&[1, 1, -2]));
    }

    #[test]
    fn resulting_frequency_of_negative_changes() {
        assert_eq!(-6, resulting_frequency(&[-1, -2, -3]));
    }

    #[test]
    fn sample_duplicate_frequency() {
        assert_eq!(2, duplicate_frequency(&[1, -2, 3, 1]));
    }

    #[test]
    fn duplicate_frequency_of_zero() {
        assert_eq!(0, duplicate_frequency(&[1, -1]));
    }

    #[test]
    fn duplicate_frequency_of_ten() {
        assert_eq!(10, duplicate_frequency(&[3, 3, 4, -2, -4]));
    }

    #[test]
    fn duplicate_frequency_of_five() {
        assert_eq!(5, duplicate_frequency(&[-6, 3, 8, 5, -6]));
    }

    #[test]
    fn duplicate_frequency_of_fourteen() {
        assert_eq!(14, duplicate_frequency(&[7, 7, -2, -7, -4]));
    }
}
