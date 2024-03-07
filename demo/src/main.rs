use askama::Template;
// use crate::command::Command;
use macros::{query, Builder, RawBuilder};

#[macro_use]
mod my_macros;
mod command;

#[derive(Builder, Debug, Default)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

#[derive(Template)]
#[template(path = "demo.j2", escape = "none")]
struct T {
    name: String,
}

fn main() {
    // let t = T {
    //     name: "ayou".to_string(),
    // };
    // t.render_into();

    // let v = my_vec!(1, 2, 3, 4);
    // println!("{:?}", v);
    // query!(SELECT * FROM users WHERE age > 10);
    // hello();

    let a = Command::default();
    // let command = Command::builder()
    //     .executable("cargo".to_owned())
    //     .args(vec!["build".to_owned(), "--release".to_owned()])
    //     .env(vec![])
    //     .build()
    //     .unwrap();
    // assert!(command.current_dir.is_none());
    // let command = Command::builder()
    //     .executable("cargo".to_owned())
    //     .args(vec!["build".to_owned(), "--release".to_owned()])
    //     .env(vec![])
    //     .current_dir("..".to_owned())
    //     .build()
    //     .unwrap();
    // assert!(command.current_dir.is_some());
    // println!("{:?}", command);
}
