use fnv::FnvHashMap;
use once_cell::sync::OnceCell;
fn _get_hashmap() -> FnvHashMap<char, Entry> {
    let mapping = include_str!("../kanji_mapping_table.txt");

    let mut hashmap = FnvHashMap::default();

    for line in mapping.lines().skip(17) {
        if let Some(entry) = Entry::from_line(line) {
            hashmap.insert(entry.japanese, entry.clone());
            for val in &entry.traditional_chinese {
                hashmap.insert(*val, entry.clone());
            }
            for val in &entry.simplified_chinese {
                hashmap.insert(*val, entry.clone());
            }
        }
    }
    hashmap
}

static CELL: OnceCell<FnvHashMap<char, Entry>> = OnceCell::new();

pub fn get_hashmap() -> &'static FnvHashMap<char, Entry> {
    CELL.get_or_init(|| _get_hashmap())
}

/// Converts a string of Japanese Kanji Character to Traditional Chinese Characters
/// Leaves chars unchanged that can't be converted.
pub fn convert_to_traditional_chinese(input: &str) -> String {
    let mut out = String::new();
    for cha in input.chars() {
        if let Some(entry) = get_hashmap()
            .get(&cha)
            .map(|entry| entry.traditional_chinese.get(0))
            .flatten()
        {
            out.push(*entry);
        } else {
            out.push(cha);
        }
    }
    out
}

/// Converts a string of Japanese Kanji Character to Simplified Chinese Characters
/// Leaves chars unchanged that can't be converted.
pub fn convert_to_simplified_chinese(input: &str) -> String {
    let mut out = String::new();
    for cha in input.chars() {
        if let Some(entry) = get_hashmap()
            .get(&cha)
            .map(|entry| entry.simplified_chinese.get(0))
            .flatten()
        {
            out.push(*entry);
        } else {
            out.push(cha);
        }
    }
    out
}

/// Converts a string of Chinese Characters to Japanese Kanji Characters
/// Leaves chars unchanged that can't be converted.
pub fn convert_to_japanese_kanji(input: &str) -> String {
    let mut out = String::new();
    for cha in input.chars() {
        if let Some(entry) = get_hashmap().get(&cha).map(|entry| entry.japanese) {
            out.push(entry);
        } else {
            out.push(cha);
        }
    }
    out
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub japanese: char,
    pub traditional_chinese: Vec<char>,
    pub simplified_chinese: Vec<char>,
}
impl Entry {
    pub fn from_line(line: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() != 3 {
            return None; // If it doesn't match the format, we'll return None.
        }

        let jap = parts[0].chars().next().unwrap();
        let traditional_chinese: Vec<char> = parts[1]
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed == "N/A" {
                    None
                } else {
                    trimmed.chars().next()
                }
            })
            .collect();

        let simplified_chinese: Vec<char> = parts[2]
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed == "N/A" {
                    None
                } else {
                    trimmed.chars().next()
                }
            })
            .collect();

        Some(Entry {
            japanese: jap,
            traditional_chinese,
            simplified_chinese,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_from_line() {
        let line = "七\t七,柒,漆\t七,柒,漆";
        let entry = Entry::from_line(line).unwrap();

        assert_eq!(entry.japanese, '七');
        assert_eq!(entry.traditional_chinese, vec!['七', '柒', '漆']);
        assert_eq!(entry.simplified_chinese, vec!['七', '柒', '漆']);

        let line_with_na = "鰄\tN/A\tN/A";
        assert!(Entry::from_line(line_with_na).is_some()); // Still return an Entry even if TC and SC are "N/A"

        let incorrect_format_line = "just some random text";
        assert!(Entry::from_line(incorrect_format_line).is_none()); // Should not be able to parse this line
    }

    #[test]
    fn to_simplified_test() {
        assert_eq!(convert_to_simplified_chinese("醫生"), "医生");
    }
}
