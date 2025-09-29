use templatia::Template;

#[derive(Template)]
#[templatia(template = "host={host}, port={port}, user={username}")]
struct DbCfg { host: String, port: u16 }
