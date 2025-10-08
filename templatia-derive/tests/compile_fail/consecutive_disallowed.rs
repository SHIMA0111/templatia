use templatia::Template;

#[derive(Template)]
#[templatia(template = "{first}{second}")]
struct BadConsecutive {
    first: String,
    second: String,
}
