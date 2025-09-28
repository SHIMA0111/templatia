use templatia::Template;
use templatia_derive::Template as TemplateDerive;

#[derive(TemplateDerive)]
#[templatia(template = "host={host}\nport={port}\nuser={username}")]
struct DbCfg { host: String, port: u16 }

fn main() {}
