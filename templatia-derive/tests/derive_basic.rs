use templatia::Template;
// Tests follow AGENTS.md policy. They express intended behavior from docs.

#[test]
fn default_template_to_string_contains_fields() {
    #[derive(Template, Debug, PartialEq)]
    struct DbCfg {
        host: String,
        port: u16,
    }

    let cfg = DbCfg {
        host: "localhost".into(),
        port: 5432,
    };
    let s = cfg.render_string();
    assert_eq!(s, "host = localhost\nport = 5432");
}

#[test]
fn custom_template_roundtrip() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "url={host}:{port}")]
    struct Url {
        host: String,
        port: u16,
    }

    let url = Url {
        host: "example.com".into(),
        port: 8080,
    };
    let s = url.render_string();
    assert_eq!(s, "url=example.com:8080");

    let parsed = Url::from_str(&s).expect("should parse");
    assert_eq!(parsed, url);
}

#[test]
fn parse_error_is_reported_as_template_error_parse() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "host={host}\nport={port}")]
    struct Cfg {
        host: String,
        port: u16,
    }

    let bad = "host=local\nport=not_a_number";
    let err = Cfg::from_str(bad).expect_err("expected parse error");
    match err {
        templatia::TemplateError::ParseToType{ placeholder, value, type_name } => {
            assert_eq!(placeholder, "port");
            assert_eq!(value, "not_a_number");
            assert_eq!(type_name, "u16");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn duplicate_placeholder_inconsistent_values() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "name={name}&again={name}")]
    struct S {
        name: String,
    }

    let bad = "name=alice&again=bob";
    let err = S::from_str(bad).expect_err("expected inconsistency error");
    match err {
        templatia::TemplateError::InconsistentValues {
            placeholder,
            first_value,
            second_value,
        } => {
            assert_eq!(placeholder, "name");
            assert_eq!(first_value, "alice");
            assert_eq!(second_value, "bob");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[test]
fn duplicate_placeholder_equal_values_ok() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "name={name}&again={name}")]
    struct S {
        name: String,
    }

    let ok = "name=alice&again=alice";
    let parsed = S::from_str(ok).expect("should parse when duplicates equal");
    assert_eq!(
        parsed,
        S {
            name: "alice".into()
        }
    );
}
