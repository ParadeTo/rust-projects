use clap::Parser;
use log::trace;

#[derive(Parser, Debug)]
#[command(setting = AppSettings::ColoredHelp)]
struct Opts {
    subcmd: SubCommand,
}

enum SubCommand {
    Get(Get),
    Post(Post),
}

struct Get {
    url: String
}

struct Post {
    url: String,
    body: Vec<String>
}

fn main() {
    trace!();
}

