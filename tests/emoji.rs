// Copyright 2012-2015 The Rust Project Developer
// Copyright 2026 Jakob Franke
// See the COPYRIGHT file at the top-level directory of this distribution
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "emoji")]

#[test]
fn all_ascii_are_either_nonemoji_or_emojiother() {
    use unicode_properties::EmojiStatus;
    use unicode_properties::UnicodeEmoji;
    for i in 0u8..=255u8 {
        let c = i as char;
        let s = c.emoji_status();
        assert!(matches!(
            s,
            EmojiStatus::NonEmoji
                | EmojiStatus::EmojiOther
                | EmojiStatus::EmojiOtherAndEmojiComponent
        ))
    }
}

#[test]
fn emoji_test() {
    use std::ops::Not;
    use unicode_properties::EmojiStatus;
    use unicode_properties::UnicodeEmoji;
    assert_eq!('🦀'.emoji_status(), EmojiStatus::EmojiPresentation);
    assert!('🦀'.is_emoji_char());
    assert!('🦀'.is_emoji_component().not());
    assert!('🦀'.is_emoji_char_or_emoji_component());
}
