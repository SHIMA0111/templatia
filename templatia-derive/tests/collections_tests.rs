use std::collections::{BTreeSet, HashSet};
use templatia::Template;

// Tests follow AGENTS.md policy. They express intended behavior derived from current docs and
// observable patterns in existing tests. For collections, values are represented as
// comma-separated lists in a single placeholder field (e.g., items=a,b,c).

// ---------------------- Vec<T> ----------------------

#[test]
fn vec_string_basic_parse() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "items={items}")]
    struct S {
        items: Vec<String>,
    }

    let parsed = S::from_str("items=alice,bob,carol").expect("should parse Vec<String>");
    assert_eq!(parsed.items, vec!["alice", "bob", "carol"]);
}

#[test]
fn vec_u32_basic_parse() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "nums={nums}")]
    struct S {
        nums: Vec<u32>,
    }

    let parsed = S::from_str("nums=1,2,3,4").expect("should parse Vec<u32>");
    assert_eq!(parsed.nums, vec![1, 2, 3, 4]);
}

#[test]
fn vec_empty_value_means_empty_vec() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "values={values}")]
    struct S {
        values: Vec<String>,
    }

    let parsed = S::from_str("values=").expect("empty should mean empty Vec");
    assert!(parsed.values.is_empty());
}

#[test]
fn vec_parse_error_invalid_element_reports_placeholder_and_type() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "nums={nums}")]
    struct S {
        nums: Vec<u32>,
    }

    // "x" cannot parse to u32. Current implementation reports the whole segment value.
    let err = S::from_str("nums=1,2,x,4").expect_err("expected parse error");
    match err {
        templatia::TemplateError::ParseToType { placeholder, value, type_name } => {
            assert_eq!(placeholder, "nums");
            assert_eq!(value, "1,2,x,4");
            assert_eq!(type_name, "Vec<u32>");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn vec_duplicate_placeholders_require_equal_segments() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "a={items};b={items}")]
    struct S {
        items: Vec<u8>,
    }

    // Equal segments: OK
    let ok = S::from_str("a=1,2,3;b=1,2,3").expect("should parse when duplicate segments equal");
    assert_eq!(ok.items, vec![1, 2, 3]);

    // Inconsistent segments: error
    let bad = S::from_str("a=1,2,3;b=1,2,4").expect_err("expected inconsistency error");
    match bad {
        templatia::TemplateError::InconsistentValues { placeholder, first_value, second_value } => {
            assert_eq!(placeholder, "items");
            assert_eq!(first_value, "1,2,3");
            assert_eq!(second_value, "1,2,4");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

// ---------------------- HashSet<T> ----------------------

#[test]
fn hashset_string_parse_deduplicates() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "tags={tags}")]
    struct S {
        tags: HashSet<String>,
    }

    let parsed = S::from_str("tags=red,green,red,blue,green").expect("should parse HashSet<String>");
    let expected: HashSet<String> = ["red", "green", "blue"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(parsed.tags, expected);
}

#[test]
fn hashset_empty_value_means_empty_set() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "tags={tags}")]
    struct S {
        tags: HashSet<u32>,
    }

    let parsed = S::from_str("tags=").expect("empty should mean empty set");
    assert!(parsed.tags.is_empty());
}

#[test]
fn hashset_parse_error_invalid_element() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "ids={ids}")]
    struct S {
        ids: HashSet<u16>,
    }

    let err = S::from_str("ids=1,2,x,3").expect_err("expected parse error");
    match err {
        templatia::TemplateError::ParseToType { placeholder, value, type_name } => {
            assert_eq!(placeholder, "ids");
            assert_eq!(value, "1,2,x,3");
            assert_eq!(type_name, "HashSet<u16>");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn hashset_duplicate_placeholders_must_match() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "one={ids}&two={ids}")]
    struct S {
        ids: HashSet<u8>,
    }

    let val = S::from_str("one=1,2,3&two=1,2,3").expect("equal duplicate segments parse");
    assert_eq!(val.ids, vec![1, 2, 3].into_iter().collect());
}

// ---------------------- Missing placeholders behavior ----------------------

#[test]
fn collections_not_in_template_default_when_allowed() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "id={id}", allow_missing_placeholders)]
    struct S {
        id: u8,
        items: Vec<u32>,
        tags: HashSet<String>,
        ord: BTreeSet<i64>,
    }

    let s = S::from_str("id=7").expect("should parse with missing placeholders allowed");
    assert_eq!(s.id, 7);
    assert!(s.items.is_empty());
    assert!(s.tags.is_empty());
    assert!(s.ord.is_empty());
}

