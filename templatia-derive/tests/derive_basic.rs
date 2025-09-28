use templatia::derive::Template;
use templatia::Template;
// Tests follow AGENTS.md policy. They express intended behavior from docs.

#[test]
fn default_template_to_string_contains_fields() {
    #[derive(Template, Debug, PartialEq)]
    struct DbCfg { host: String, port: u16 }

    let cfg = DbCfg { host: "localhost".into(), port: 5432 };
    let s = cfg.to_string();
    assert_eq!(s, "host = localhost\nport = 5432");
}

#[test]
fn custom_template_roundtrip() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "url={host}:{port}")]
    struct Url { host: String, port: u16 }

    let url = Url { host: "example.com".into(), port: 8080 };
    let s = url.to_string();
    assert_eq!(s, "url=example.com:8080");

    let parsed = Url::from_string(&s).expect("should parse");
    assert_eq!(parsed, url);
}

#[test]
fn parse_error_is_reported_as_template_error_parse() {
    #[derive(Template, Debug, PartialEq)]
    #[templatia(template = "host={host}\nport={port}")]
    struct Cfg { host: String, port: u16 }

    let bad = "host=local\nport=not_a_number";
    let err = Cfg::from_string(bad).expect_err("expected parse error");
    match err {
        templatia::TemplateError::Parse(msg) => {
            assert!(msg.contains("Failed to parse field \"port\""));
        }
        other => panic!("unexpected error: {other:?}"),
    }
}
