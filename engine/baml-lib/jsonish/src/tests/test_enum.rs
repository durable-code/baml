use super::*;

const ENUM_FILE: &str = r#"
// Enums
enum Category {
ONE
TWO
}
"#;

test_deserializer!(
    test_enum,
    ENUM_FILE,
    r#"TWO"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    case_insensitive,
    ENUM_FILE,
    r#"two"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    with_quotes,
    ENUM_FILE,
    r#""TWO""#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    from_enum_list_single,
    ENUM_FILE,
    r#"["TWO"]"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    from_enum_list_multi,
    ENUM_FILE,
    r#"["TWO", "THREE"]"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    from_string_with_extra_text_after_1,
    ENUM_FILE,
    r#""ONE: The description of k1""#,
    FieldType::Enum("Category".to_string()),
    "ONE"
);

test_deserializer!(
    from_string_with_extra_text_after_2,
    ENUM_FILE,
    r#""ONE - The description of an enum value""#,
    FieldType::Enum("Category".to_string()),
    "ONE"
);

test_failing_deserializer!(
    from_string_with_extra_text_after_3,
    ENUM_FILE,
    r#""ONE - is the answer, not TWO""#,
    FieldType::Enum("Category".to_string())
);

test_failing_deserializer!(
    from_string_with_extra_text_after_4,
    ENUM_FILE,
    r#""ONE. is the answer, not TWO""#,
    FieldType::Enum("Category".to_string())
);

test_failing_deserializer!(
    from_string_with_extra_text_after_5,
    ENUM_FILE,
    r#""ONE: is the answer, not TWO""#,
    FieldType::Enum("Category".to_string())
);

const ENUM_FILE_WITH_DESCRIPTIONS: &str = r#"
// Enums
enum Category {
ONE @alias(k1) @description("The description of enum value une")
TWO @alias("k-2-3.1_1") @description("The description of enum value deux")
THREE @alias(NUMBER THREE)
}
"#;

test_deserializer!(
    aliases_1,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k1"#,
    FieldType::Enum("Category".to_string()),
    "ONE"
);

test_deserializer!(
    aliases_2,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k-2-3.1_1"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    aliases_3,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"NUMBER THREE"#,
    FieldType::Enum("Category".to_string()),
    "THREE"
);

test_deserializer!(
    no_punctuation,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"number three"#,
    FieldType::Enum("Category".to_string()),
    "THREE"
);

test_deserializer!(
    no_punctuation_2,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k-2-3 1_1"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    descriptions,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k1: The description of enum value une"#,
    FieldType::Enum("Category".to_string()),
    "ONE"
);

test_deserializer!(
    descriptions_whitespace,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k-2-3.1_1 The description of enum value deux"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    descriptions_period,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k-2-3.1_1. The description of enum value deux"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    alias_with_text,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"I would think k-2-3.1_1 is the best"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

test_deserializer!(
    multi_aliases,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k1 is the best! k-2-3.1_1 is bad. k1!"#,
    FieldType::Enum("Category".to_string()),
    "ONE"
);

test_deserializer!(
    multi_aliases_1,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k1 is ok! k-2-3.1_1 is better. I would advise k-2-3.1_1!"#,
    FieldType::Enum("Category".to_string()),
    "TWO"
);

// Too many ties
test_failing_deserializer!(
    multi_aliases_2,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"k1 is the best! k-2-3.1_1 is bad. NUMBER_THREE!"#,
    FieldType::Enum("Category".to_string())
);

test_deserializer!(
    list_of_enums,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"["k1", "k-2-3.1_1"]"#,
    FieldType::List(FieldType::Enum("Category".to_string()).into()),
    ["ONE", "TWO"]
);

test_deserializer!(
    list_of_enums_2,
    ENUM_FILE_WITH_DESCRIPTIONS,
    r#"I would think something like this!
```json    
[k1, "k-2-3.1_1", "NUMBER THREE"]
```
"#,
    FieldType::List(FieldType::Enum("Category".to_string()).into()),
    ["ONE", "TWO", "THREE"]
);
