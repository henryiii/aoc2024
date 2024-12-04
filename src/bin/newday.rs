use std::fs::File;
use std::io::Write;

use clap::Parser;
use derive_new::new;
use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Parser, Debug)]
#[command()]
struct Opts {
    day: u32,
}

#[derive(Serialize, new)]
struct Context {
    day: String,
}

const TMPL: &str = include_str!("template.rs.in");

fn main() {
    let opts: Opts = Opts::parse();
    let mut tt = TinyTemplate::new();
    tt.add_template("template", TMPL).unwrap();
    tt.add_formatter("simplify", |value, f| {
        let val: u32 = value.as_str().unwrap().parse().unwrap();
        f.push_str(&val.to_string());
        Ok(())
    });
    let day = format!("{:02}", opts.day);
    let data = Context::new(day.clone());
    let rendered = tt.render("template", &data).unwrap();
    // Using create_new to avoid overwriting existing files, instead of simpler std::fs::write
    File::create_new(format!("src/bin/{day}.rs"))
        .unwrap()
        .write_all(rendered.as_bytes())
        .unwrap();
    File::create_new(format!("data/{day}.txt"))
        .unwrap()
        .write_all(b"")
        .unwrap();
    File::create_new(format!("samples/{day}.txt"))
        .unwrap()
        .write_all(b"")
        .unwrap();
}
