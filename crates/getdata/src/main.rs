use clap::Parser;

use getdata::get_input;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    /// The year to get data for
    year: u16,
    /// The day to get data for
    day: u8,
    /// The session token to use. Will use the `AOC_SESSION` environment variable if not provided.
    #[clap(long, env = "AOC_SESSION")]
    session: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = get_input(args.day, args.year, &args.session)?;
    println!("{input}");
    Ok(())
}
