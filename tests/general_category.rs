// Copyright 2012-2015 The Rust Project Developer
// Copyright 2026 Jakob Franke
// See the COPYRIGHT file at the top-level directory of this distribution
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg(feature = "general-category")]

#[test]
fn general_category_test() {
    use std::ops::Not;
    use unicode_properties::UnicodeGeneralCategory;
    use unicode_properties::{GeneralCategory, GeneralCategoryGroup};
    assert_eq!('A'.general_category(), GeneralCategory::UppercaseLetter);
    assert_eq!('A'.general_category_group(), GeneralCategoryGroup::Letter);
    assert!('A'.is_letter_cased());
    assert_eq!(' '.general_category(), GeneralCategory::SpaceSeparator);
    assert_eq!(
        ' '.general_category_group(),
        GeneralCategoryGroup::Separator
    );
    assert!(' '.is_letter_cased().not());
    assert_eq!('一'.general_category(), GeneralCategory::OtherLetter);
    assert_eq!('一'.general_category_group(), GeneralCategoryGroup::Letter);
    assert!('一'.is_letter_cased().not());
    assert_eq!('🦀'.general_category(), GeneralCategory::OtherSymbol);
    assert_eq!('🦀'.general_category_group(), GeneralCategoryGroup::Symbol);
    assert!('🦀'.is_letter_cased().not());
}
