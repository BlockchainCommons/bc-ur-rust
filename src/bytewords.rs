pub use ur::bytewords::Style;

pub fn encode(data: impl AsRef<[u8]>, style: Style) -> String {
    ur::bytewords::encode(data.as_ref(), style)
}

/// Encodes a 4-byte slice of data as a string of bytewords for identification
/// purposes.
#[must_use]
pub fn identifier(data: &[u8; 4]) -> String {
    let data = data.iter();
    let words: Vec<&str> = data
            .map(|&b| BYTEWORDS.get(b as usize).copied().unwrap())
            .collect();
    words.join(" ")
}

// Example Bytemoji identifiers:
// 💛 🚩 🐥 🫠
// 🧵 💀 🎂 🛟
// 💫 🤠 👆 😂
// 🪐 👔 👚 👻
// 🧸 🥚 🧀 🙀
// 👃 👄 🐬 🧄
// 🧦 🌽 🏠 🦆
// 🌐 🌭 🥺 🛑
// 🥁 🦞 🌹 🐢
// 😽 😐 🐺 🌀

/// Encodes a 4-byte slice of data as a string of bytemojis for identification
/// purposes.
#[must_use]
pub fn bytemoji_identifier(data: &[u8; 4]) -> String {
    let data = data.iter();
    let words: Vec<&str> = data
            .map(|&b| BYTEMOJIS.get(b as usize).copied().unwrap())
            .collect();
    words.join(" ")
}

pub fn decode(data: &str, style: Style) -> Result<Vec<u8>, anyhow::Error> {
    ur::bytewords::decode(data, style).map_err(|e| anyhow::anyhow!(e))
}

pub const BYTEWORDS: [&str; 256] = [
    "able", "acid", "also", "apex", "aqua", "arch", "atom", "aunt",
    "away", "axis", "back", "bald", "barn", "belt", "beta", "bias",
    "blue", "body", "brag", "brew", "bulb", "buzz", "calm", "cash",
    "cats", "chef", "city", "claw", "code", "cola", "cook", "cost",
    "crux", "curl", "cusp", "cyan", "dark", "data", "days", "deli",
    "dice", "diet", "door", "down", "draw", "drop", "drum", "dull",
    "duty", "each", "easy", "echo", "edge", "epic", "even", "exam",
    "exit", "eyes", "fact", "fair", "fern", "figs", "film", "fish",
    "fizz", "flap", "flew", "flux", "foxy", "free", "frog", "fuel",
    "fund", "gala", "game", "gear", "gems", "gift", "girl", "glow",
    "good", "gray", "grim", "guru", "gush", "gyro", "half", "hang",
    "hard", "hawk", "heat", "help", "high", "hill", "holy", "hope",
    "horn", "huts", "iced", "idea", "idle", "inch", "inky", "into",
    "iris", "iron", "item", "jade", "jazz", "join", "jolt", "jowl",
    "judo", "jugs", "jump", "junk", "jury", "keep", "keno", "kept",
    "keys", "kick", "kiln", "king", "kite", "kiwi", "knob", "lamb",
    "lava", "lazy", "leaf", "legs", "liar", "limp", "lion", "list",
    "logo", "loud", "love", "luau", "luck", "lung", "main", "many",
    "math", "maze", "memo", "menu", "meow", "mild", "mint", "miss",
    "monk", "nail", "navy", "need", "news", "next", "noon", "note",
    "numb", "obey", "oboe", "omit", "onyx", "open", "oval", "owls",
    "paid", "part", "peck", "play", "plus", "poem", "pool", "pose",
    "puff", "puma", "purr", "quad", "quiz", "race", "ramp", "real",
    "redo", "rich", "road", "rock", "roof", "ruby", "ruin", "runs",
    "rust", "safe", "saga", "scar", "sets", "silk", "skew", "slot",
    "soap", "solo", "song", "stub", "surf", "swan", "taco", "task",
    "taxi", "tent", "tied", "time", "tiny", "toil", "tomb", "toys",
    "trip", "tuna", "twin", "ugly", "undo", "unit", "urge", "user",
    "vast", "very", "veto", "vial", "vibe", "view", "visa", "void",
    "vows", "wall", "wand", "warm", "wasp", "wave", "waxy", "webs",
    "what", "when", "whiz", "wolf", "work", "yank", "yawn", "yell",
    "yoga", "yurt", "zaps", "zero", "zest", "zinc", "zone", "zoom",
];

/*
Selection criteria:
- All code points are 3 or 4 UTF-8 bytes.
    - Some emojis like 👁️‍🗨️ (“I am a witness”) are 17 UTF-8 bytes!
- All single glyphs.
    - Some emojis are sequences, like 👨🏿‍👩🏾‍👧🏽‍👦🏼 ("family: man, woman, girl, boy with various skin tones") in 28 bytes.
- All emojis are visually distinct, with unique shapes and designs.
- Avoid emojis that are highly similar or could be easily confused.
- Avoid emojis that depend solely on color differences to be distinguished.
- Exclude combining forms, skin tone modifiers, and gender modifiers.
- Ensure the set covers a wide range of themes and concepts.
- Prefer emojis with positive or neutral connotations.
- Prefer emojis that render well at small sizes and on dark backgrounds.
- Avoid national, ideological, and controversial symbols.

Full list:
😀😂😆😉🙄😋😎😍😘😭🫠🥱🤩😶🤨🫥
🥵🥶😳🤪😵😡🤢😇🤠🤡🥳🥺😬🤑🙃🤯
😈👹👺💀👻👽😺😹😻😽🙀😿🫶🤲🙌🤝
👍👎👈👆💪👄🦷👂👃🧠👀🤚🦶🍎🍊🍋
🍌🍉🍇🍓🫐🍒🍑🍍🥝🍆🥑🥦🍅🌽🥕🫒
🧄🥐🥯🍞🧀🥚🍗🌭🍔🍟🍕🌮🥙🍱🍜🍤
🍚🥠🍨🍦🎂🪴🌵🌱💐🍁🍄🌹🌺🌼🌻🌸
💨🌊💧💦🌀🌈🌞🌝🌛🌜🌙🌎💫⭐🪐🌐
💛💔💘💖💕🏁🚩💬💯🚫🔴🔷🟩🛑🔺🚗
🚑🚒🚜🛵🚨🚀🚁🛟🚦🏰🎡🎢🎠🏠🔔🔑
🚪🪑🎈💌📦📫📖📚📌🧮🔒💎📷⏰⏳📡
💡💰🧲🧸🎁🎀🎉🪭👑🫖🔭🛁🏆🥁🎷🎺
🏀🏈🎾🏓✨🔥💥👕👚👖🩳👗👔🧢👓🧶
🧵💍👠👟🧦🧤👒👜🐱🐶🐭🐹🐰🦊🐻🐼
🐨🐯🦁🐮🐷🐸🐵🐔🐥🦆🦉🐴🦄🐝🐛🦋
🐌🐞🐢🐺🐍🪽🐙🦑🪼🦞🦀🐚🦭🐟🐬🐳
*/

pub const BYTEMOJIS: [&str; 256] = [
    "😀", "😂", "😆", "😉", "🙄", "😋", "😎", "😍",
    "😘", "😭", "🫠", "🥱", "🤩", "😶", "🤨", "🫥",
    "🥵", "🥶", "😳", "🤪", "😵", "😡", "🤢", "😇",
    "🤠", "🤡", "🥳", "🥺", "😬", "🤑", "🙃", "🤯",
    "😈", "👹", "👺", "💀", "👻", "👽", "😺", "😹",
    "😻", "😽", "🙀", "😿", "🫶", "🤲", "🙌", "🤝",
    "👍", "👎", "👈", "👆", "💪", "👄", "🦷", "👂",
    "👃", "🧠", "👀", "🤚", "🦶", "🍎", "🍊", "🍋",
    "🍌", "🍉", "🍇", "🍓", "🫐", "🍒", "🍑", "🍍",
    "🥝", "🍆", "🥑", "🥦", "🍅", "🌽", "🥕", "🫒",
    "🧄", "🥐", "🥯", "🍞", "🧀", "🥚", "🍗", "🌭",
    "🍔", "🍟", "🍕", "🌮", "🥙", "🍱", "🍜", "🍤",
    "🍚", "🥠", "🍨", "🍦", "🎂", "🪴", "🌵", "🌱",
    "💐", "🍁", "🍄", "🌹", "🌺", "🌼", "🌻", "🌸",
    "💨", "🌊", "💧", "💦", "🌀", "🌈", "🌞", "🌝",
    "🌛", "🌜", "🌙", "🌎", "💫", "⭐", "🪐", "🌐",
    "💛", "💔", "💘", "💖", "💕", "🏁", "🚩", "💬",
    "💯", "🚫", "🔴", "🔷", "🟩", "🛑", "🔺", "🚗",
    "🚑", "🚒", "🚜", "🛵", "🚨", "🚀", "🚁", "🛟",
    "🚦", "🏰", "🎡", "🎢", "🎠", "🏠", "🔔", "🔑",
    "🚪", "🪑", "🎈", "💌", "📦", "📫", "📖", "📚",
    "📌", "🧮", "🔒", "💎", "📷", "⏰", "⏳", "📡",
    "💡", "💰", "🧲", "🧸", "🎁", "🎀", "🎉", "🪭",
    "👑", "🫖", "🔭", "🛁", "🏆", "🥁", "🎷", "🎺",
    "🏀", "🏈", "🎾", "🏓", "✨", "🔥", "💥", "👕",
    "👚", "👖", "🩳", "👗", "👔", "🧢", "👓", "🧶",
    "🧵", "💍", "👠", "👟", "🧦", "🧤", "👒", "👜",
    "🐱", "🐶", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼",
    "🐨", "🐯", "🦁", "🐮", "🐷", "🐸", "🐵", "🐔",
    "🐥", "🦆", "🦉", "🐴", "🦄", "🐝", "🐛", "🦋",
    "🐌", "🐞", "🐢", "🐺", "🐍", "🪽", "🐙", "🦑",
    "🪼", "🦞", "🦀", "🐚", "🦭", "🐟", "🐬", "🐳",
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
        let duplicates: Vec<_> = dict.iter().filter(|(_, &v)| v > 1).collect();
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
}
