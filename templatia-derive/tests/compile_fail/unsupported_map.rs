use std::collections::HashMap;
use templatia::Template;

#[derive(Template)]
#[templatia(template = "map={map}")]
struct HasMap {
    map: HashMap<String, i32>,
}
