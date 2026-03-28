pub use bytewords::Style;
pub use ur::bytewords;

use crate::Result;

pub fn encode(data: impl AsRef<[u8]>, style: Style) -> String {
    ur::bytewords::encode(data.as_ref(), style)
}

/// Encodes an arbitrary byte slice as a string of space-separated bytewords.
#[must_use]
pub fn encode_to_words(data: &[u8]) -> String {
    data.iter()
        .map(|&b| BYTEWORDS[b as usize])
        .collect::<Vec<_>>()
        .join(" ")
}

/// Encodes an arbitrary byte slice as a string of space-separated bytemojis.
#[must_use]
pub fn encode_to_bytemojis(data: &[u8]) -> String {
    data.iter()
        .map(|&b| BYTEMOJIS[b as usize])
        .collect::<Vec<_>>()
        .join(" ")
}

/// Encodes an arbitrary byte slice as minimal bytewords (first+last letter of
/// each word, concatenated without separator).
#[must_use]
pub fn encode_to_minimal_bytewords(data: &[u8]) -> String {
    data.iter()
        .map(|&b| {
            let w = BYTEWORDS[b as usize].as_bytes();
            let mut s = String::with_capacity(2);
            s.push(w[0] as char);
            s.push(w[w.len() - 1] as char);
            s
        })
        .collect::<String>()
}

/// Encodes a 4-byte slice of data as a string of bytewords for identification
/// purposes.
#[must_use]
pub fn identifier(data: &[u8; 4]) -> String { encode_to_words(data) }

/// Encodes a 4-byte slice of data as a string of bytemojis for identification
/// purposes.
#[must_use]
pub fn bytemoji_identifier(data: &[u8; 4]) -> String {
    encode_to_bytemojis(data)
}

pub fn decode(data: &str, style: Style) -> Result<Vec<u8>> {
    Ok(ur::bytewords::decode(data, style)?)
}

/// Returns `true` if `emoji` is one of the 256 bytemojis.
#[must_use]
pub fn is_valid_bytemoji(emoji: &str) -> bool { BYTEMOJIS.contains(&emoji) }

/// Canonicalizes a byteword token (2–4 ASCII letters, case-insensitive) to its
/// full 4-letter lowercase form. Returns `None` if the token is not a valid
/// byteword or any of its short forms.
#[must_use]
pub fn canonicalize_byteword(token: &str) -> Option<String> {
    use std::sync::LazyLock;

    static WORD_SET: LazyLock<std::collections::HashSet<&'static str>> =
        LazyLock::new(|| BYTEWORDS.iter().copied().collect());
    static FIRST_LAST: LazyLock<
        std::collections::HashMap<String, &'static str>,
    > = LazyLock::new(|| {
        BYTEWORDS
            .iter()
            .map(|w| {
                let bytes = w.as_bytes();
                let key = format!(
                    "{}{}",
                    bytes[0] as char,
                    bytes[bytes.len() - 1] as char
                );
                (key, *w)
            })
            .collect()
    });
    static FIRST_THREE: LazyLock<
        std::collections::HashMap<&'static str, &'static str>,
    > = LazyLock::new(|| BYTEWORDS.iter().map(|w| (&w[..3], *w)).collect());
    static LAST_THREE: LazyLock<
        std::collections::HashMap<&'static str, &'static str>,
    > = LazyLock::new(|| BYTEWORDS.iter().map(|w| (&w[1..], *w)).collect());

    let lower = token.to_ascii_lowercase();
    match lower.len() {
        4 => {
            if WORD_SET.contains(lower.as_str()) {
                Some(lower)
            } else {
                None
            }
        }
        2 => FIRST_LAST.get(&lower).map(|w| w.to_string()),
        3 => FIRST_THREE
            .get(lower.as_str())
            .or_else(|| LAST_THREE.get(lower.as_str()))
            .map(|w| w.to_string()),
        _ => None,
    }
}

pub const BYTEWORDS: [&str; 256] = [
    "able", "acid", "also", "apex", "aqua", "arch", "atom", "aunt", "away",
    "axis", "back", "bald", "barn", "belt", "beta", "bias", "blue", "body",
    "brag", "brew", "bulb", "buzz", "calm", "cash", "cats", "chef", "city",
    "claw", "code", "cola", "cook", "cost", "crux", "curl", "cusp", "cyan",
    "dark", "data", "days", "deli", "dice", "diet", "door", "down", "draw",
    "drop", "drum", "dull", "duty", "each", "easy", "echo", "edge", "epic",
    "even", "exam", "exit", "eyes", "fact", "fair", "fern", "figs", "film",
    "fish", "fizz", "flap", "flew", "flux", "foxy", "free", "frog", "fuel",
    "fund", "gala", "game", "gear", "gems", "gift", "girl", "glow", "good",
    "gray", "grim", "guru", "gush", "gyro", "half", "hang", "hard", "hawk",
    "heat", "help", "high", "hill", "holy", "hope", "horn", "huts", "iced",
    "idea", "idle", "inch", "inky", "into", "iris", "iron", "item", "jade",
    "jazz", "join", "jolt", "jowl", "judo", "jugs", "jump", "junk", "jury",
    "keep", "keno", "kept", "keys", "kick", "kiln", "king", "kite", "kiwi",
    "knob", "lamb", "lava", "lazy", "leaf", "legs", "liar", "limp", "lion",
    "list", "logo", "loud", "love", "luau", "luck", "lung", "main", "many",
    "math", "maze", "memo", "menu", "meow", "mild", "mint", "miss", "monk",
    "nail", "navy", "need", "news", "next", "noon", "note", "numb", "obey",
    "oboe", "omit", "onyx", "open", "oval", "owls", "paid", "part", "peck",
    "play", "plus", "poem", "pool", "pose", "puff", "puma", "purr", "quad",
    "quiz", "race", "ramp", "real", "redo", "rich", "road", "rock", "roof",
    "ruby", "ruin", "runs", "rust", "safe", "saga", "scar", "sets", "silk",
    "skew", "slot", "soap", "solo", "song", "stub", "surf", "swan", "taco",
    "task", "taxi", "tent", "tied", "time", "tiny", "toil", "tomb", "toys",
    "trip", "tuna", "twin", "ugly", "undo", "unit", "urge", "user", "vast",
    "very", "veto", "vial", "vibe", "view", "visa", "void", "vows", "wall",
    "wand", "warm", "wasp", "wave", "waxy", "webs", "what", "when", "whiz",
    "wolf", "work", "yank", "yawn", "yell", "yoga", "yurt", "zaps", "zero",
    "zest", "zinc", "zone", "zoom",
];

/// See: https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2024-008-bytemoji.md
pub const BYTEMOJIS: [&str; 256] = [
    "😀", "😂", "😆", "😉", "🙄", "😋", "😎", "😍", "😘", "😭", "🫠", "🥱",
    "🤩", "😶", "🤨", "🫥", "🥵", "🥶", "😳", "🤪", "😵", "😡", "🤢", "😇",
    "🤠", "🤡", "🥳", "🥺", "😬", "🤑", "🙃", "🤯", "😈", "👹", "👺", "💀",
    "👻", "👽", "😺", "😹", "😻", "😽", "🙀", "😿", "🫶", "🤲", "🙌", "🤝",
    "👍", "👎", "👈", "👆", "💪", "👄", "🦷", "👂", "👃", "🧠", "👀", "🤚",
    "🦶", "🍎", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🫐", "🍒", "🍑", "🍍",
    "🥝", "🍆", "🥑", "🥦", "🍅", "🌽", "🥕", "🫒", "🧄", "🥐", "🥯", "🍞",
    "🧀", "🥚", "🍗", "🌭", "🍔", "🍟", "🍕", "🌮", "🥙", "🍱", "🍜", "🍤",
    "🍚", "🥠", "🍨", "🍦", "🎂", "🪴", "🌵", "🌱", "💐", "🍁", "🍄", "🌹",
    "🌺", "🌼", "🌻", "🌸", "💨", "🌊", "💧", "💦", "🌀", "🌈", "🌞", "🌝",
    "🌛", "🌜", "🌙", "🌎", "💫", "⭐", "🪐", "🌐", "💛", "💔", "💘", "💖",
    "💕", "🏁", "🚩", "💬", "💯", "🚫", "🔴", "🔷", "🟩", "🛑", "🔺", "🚗",
    "🚑", "🚒", "🚜", "🛵", "🚨", "🚀", "🚁", "🛟", "🚦", "🏰", "🎡", "🎢",
    "🎠", "🏠", "🔔", "🔑", "🚪", "🪑", "🎈", "💌", "📦", "📫", "📖", "📚",
    "📌", "🧮", "🔒", "💎", "📷", "⏰", "⏳", "📡", "💡", "💰", "🧲", "🧸",
    "🎁", "🎀", "🎉", "🪭", "👑", "🫖", "🔭", "🛁", "🏆", "🥁", "🎷", "🎺",
    "🏀", "🏈", "🎾", "🏓", "✨", "🔥", "💥", "👕", "👚", "👖", "🩳", "👗",
    "👔", "🧢", "👓", "🧶", "🧵", "💍", "👠", "👟", "🧦", "🧤", "👒", "👜",
    "🐱", "🐶", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮",
    "🐷", "🐸", "🐵", "🐔", "🐥", "🦆", "🦉", "🐴", "🦄", "🐝", "🐛", "🦋",
    "🐌", "🐞", "🐢", "🐺", "🐍", "🪽", "🐙", "🦑", "🪼", "🦞", "🦀", "🐚",
    "🦭", "🐟", "🐬", "🐳",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytemoji_uniqueness() {
        let bytemojis = BYTEMOJIS.to_vec();
        let mut dict = std::collections::HashMap::new();
        for bytemoji in bytemojis.iter() {
            let count = dict.entry(bytemoji).or_insert(0);
            *count += 1;
        }
        let duplicates: Vec<_> = dict.iter().filter(|(_, v)| *v > &1).collect();
        assert!(duplicates.is_empty(), "Duplicates: {:?}", duplicates);
    }

    #[test]
    fn test_bytemoji_lengths() {
        let mut over_length = Vec::new();
        for &bytemoji in BYTEMOJIS.iter() {
            let len = bytemoji.len();
            if len > 4 {
                over_length.push((bytemoji, len));
            }
        }
        for (bytemoji, len) in over_length.iter() {
            println!("{}  : {},", bytemoji, len);
        }
        assert!(over_length.is_empty(), "Some bytemojis are over 4 bytes");
    }

    #[test]
    fn test_encode_to_words_matches_identifier() {
        let data: [u8; 4] = [0, 1, 2, 3];
        assert_eq!(encode_to_words(&data), identifier(&data));
    }

    #[test]
    fn test_encode_to_bytemojis_matches_bytemoji_identifier() {
        let data: [u8; 4] = [0, 1, 2, 3];
        assert_eq!(encode_to_bytemojis(&data), bytemoji_identifier(&data));
    }

    #[test]
    fn test_encode_to_words_various_lengths() {
        assert_eq!(encode_to_words(&[0]), "able");
        assert_eq!(encode_to_words(&[0, 255]), "able zoom");
        assert_eq!(encode_to_words(&[0, 1, 2, 3]), "able acid also apex");
        let eight: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let encoded = encode_to_words(&eight);
        let words: Vec<&str> = encoded.split(' ').collect();
        assert_eq!(words.len(), 8);
    }

    #[test]
    fn test_encode_to_words_empty() {
        assert_eq!(encode_to_words(&[]), "");
    }

    #[test]
    fn test_encode_to_words_all_bytes_unique() {
        let all_bytes: Vec<u8> = (0..=255).collect();
        let encoded = encode_to_words(&all_bytes);
        let words: Vec<&str> = encoded.split(' ').collect();
        assert_eq!(words.len(), 256);
        let unique: std::collections::HashSet<&&str> = words.iter().collect();
        assert_eq!(
            unique.len(),
            256,
            "All 256 byte values must map to distinct words"
        );
    }

    #[test]
    fn test_encode_to_bytemojis_various_lengths() {
        assert_eq!(encode_to_bytemojis(&[0]), "😀");
        let eight: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let encoded = encode_to_bytemojis(&eight);
        let emojis: Vec<&str> = encoded.split(' ').collect();
        assert_eq!(emojis.len(), 8);
    }

    #[test]
    fn test_encode_to_minimal_bytewords() {
        // "able" → "ae", "acid" → "ad", "also" → "ao", "apex" → "ax"
        assert_eq!(encode_to_minimal_bytewords(&[0, 1, 2, 3]), "aeadaoax");
        assert_eq!(encode_to_minimal_bytewords(&[0]), "ae");
        assert_eq!(encode_to_minimal_bytewords(&[]), "");
    }

    #[test]
    fn test_encode_to_minimal_bytewords_matches_words() {
        // Each minimal is first+last letter of the corresponding full word
        for b in 0..=255u8 {
            let word = BYTEWORDS[b as usize];
            let minimal = encode_to_minimal_bytewords(&[b]);
            let wb = word.as_bytes();
            let expected =
                format!("{}{}", wb[0] as char, wb[wb.len() - 1] as char);
            assert_eq!(minimal, expected, "byte {b}: word={word}");
        }
    }
}
