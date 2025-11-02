use templatia::Template;

// Tests follow AGENTS.md policy. They express intended behavior from documentation patterns.
// Vec<T> is represented as a comma-separated list within a single placeholder.

#[test]
fn vec_render_and_parse_roundtrip_strings() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "items={items}")]
    struct S {
        items: Vec<String>,
    }

    let s = S { items: vec!["a".into(), "b".into(), "c".into()] };
    let rendered = s.render_string();
    assert_eq!(rendered, "items=a,b,c");

    let parsed = S::from_str(&rendered).expect("should parse");
    assert_eq!(parsed, s);
}

#[test]
fn vec_parse_numbers_and_preserve_order() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "nums={nums}")]
    struct N { nums: Vec<u32> }

    let parsed = N::from_str("nums=10,20,30").expect("parse numbers");
    assert_eq!(parsed.nums, vec![10, 20, 30]);
}

#[test]
fn vec_empty_string_means_empty_vec() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "v={v}")]
    struct V { v: Vec<String> }

    let parsed = V::from_str("v=").expect("empty -> empty vec");
    assert!(parsed.v.is_empty());
}

#[test]
fn vec_duplicate_placeholders_must_match_as_strings() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "a={xs};b={xs}")]
    struct Xs { xs: Vec<u8> }

    // Equal segments -> ok
    let ok = Xs::from_str("a=1,2,3;b=1,2,3").expect("equal segments ok");
    assert_eq!(ok.xs, vec![1,2,3]);

    // Different segments -> inconsistency error
    let err = Xs::from_str("a=1,2,3;b=1,2,4").expect_err("expected inconsistency");
    match err {
        templatia::TemplateError::InconsistentValues { placeholder, first_value, second_value } => {
            assert_eq!(placeholder, "xs");
            assert_eq!(first_value, "1,2,3");
            assert_eq!(second_value, "1,2,4");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn vec_parse_error_reports_placeholder_and_type() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "nums={nums}")]
    struct N { nums: Vec<u16> }

    let err = N::from_str("nums=1,2,x,4").expect_err("expect parse error");
    match err {
        templatia::TemplateError::ParseToType { placeholder, value, type_name } => {
            assert_eq!(placeholder, "nums");
            assert_eq!(value, "1,2,x,4");
            assert_eq!(type_name, "Vec<u16>");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
