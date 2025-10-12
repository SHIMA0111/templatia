use templatia::Template;

#[derive(Template)]
#[templatia(template = "vals={vals}")]
struct HasTuple {
    vals: (i32, i32),
}
