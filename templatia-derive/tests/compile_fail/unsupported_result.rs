use templatia::Template;

#[derive(Template)]
#[templatia(template = "res={res}")]
struct HasResult {
    res: Result<i32, String>,
}
