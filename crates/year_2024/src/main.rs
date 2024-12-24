use clap::Parser;
use seq_macro::seq;

#[derive(Parser)]
struct Opts {
    #[clap()]
    days: Vec<u8>,

    #[clap(long)]
    vis: bool,
}

fn main() {
    let opts = Opts::parse();

    seq!(N in 01..=24 {
        if opts.days.is_empty() || opts.days.contains(&N) {
            year_2024::day_~N::main(opts.vis);
        }
    });
}
