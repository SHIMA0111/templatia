use templatia::Template;

#[derive(Template)]
#[templatia(template = "items={items}")]
struct HasCollection {
    items: Vec<String>,
}
