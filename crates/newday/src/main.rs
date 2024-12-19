/*!
# New day template processor

This program produces a new day, given a number. The day must not already exist.
*/
use std::fs::File;
use std::io::Write;

use clap::Parser;
use derive_new::new;
use serde::Serialize;
use tinytemplate::TinyTemplate;

use getdata::get_input;

#[derive(Parser, Debug)]
#[command()]
struct Opts {
    /// The day to create
    day: u8,

    /// The year to create, defaults to 2024
    #[clap(long, default_value = "2024")]
    year: u16,

    /// The session token to use. Will use the `AOC_SESSION` environment variable if not provided.
    #[clap(long, env = "AOC_SESSION")]
    session: Option<String>,
}

#[derive(Serialize, new)]
struct Context {
    year: u16,
    day: String,
}

const TMPL: &str = include_str!("template.rs.in");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let year = opts.year;
    let mut tt = TinyTemplate::new();
    tt.add_template("template", TMPL)?;
    tt.add_formatter("simplify", |value, f| {
        let val: u32 = value
            .as_str()
            .ok_or(std::fmt::Error)?
            .parse()
            .map_err(|_| std::fmt::Error)?;
        f.push_str(&val.to_string());
        Ok(())
    });
    let day = format!("{:02}", opts.day);
    let data = Context::new(opts.year, day.clone());
    let rendered = tt.render("template", &data)?;
    // Using create_new to avoid overwriting existing files, instead of simpler std::fs::write
    File::create_new(format!("crates/year_{year}/src/{day}.rs"))?.write_all(rendered.as_bytes())?;
    File::create_new(format!("samples/{year}/{day}.txt"))?.write_all(b"")?;

    let data = if let Some(session) = opts.session {
        get_input(opts.day, opts.year, &session)?
    } else {
        String::new()
    };
    File::create_new(format!("data/{year}/{day}.txt"))?.write_all(data.as_bytes())?;
    Ok(())
}
