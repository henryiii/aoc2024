# Advent of Code 2024 in Rust

I'm trying the [advent of code](https://adventofcode.com/2024) in Rust again this
year to learn more Rust. I'm not trying to be fast and place on the leaderboards
(which also require working at midnight, which I'm mostly not interested in
doing), I'm trying to be somewhat elegant and learn new things in Rust. The
documentation is [live here](https://henryiii.github.io/aoc2024).

The focus of these solutions is a balance of speed and readability.

I highly recommend loading this up in a good editor, like Visual Studio Code or
VIM with the ALE plugin. It will add type information to all inferred types,
autocomplete, show documentation, etc.

Feel free to check out my 2023 repo at <https://github.com/henryiii/aoc2023>.

## Formatting and linting

Use:

```bash
cargo fmt --all && cargo clippy-all
```

If you want to auto-fix anything, you can:

```bash
cargo clippy --fix --allow-dirty --allow-staged
```

I also looked for removable features using
[unused-features](https://crates.io/crates/cargo-unused-features), both to
speed up compilation and it helped removed a small dependence on unicode in
regex, though it doesn't handle workspaces, sadly.

## Tests

Use:

```bash
cargo test --package year_2024
```

Useful flags include `-- --nocapture` and `-- day_<NUMBER>` for just one set of tests.

If you have `cargo-nextest` (say, from `brew install cargo-nextest`), then
`cargo nextest run` also works.

## New days

To create a new day, use:

```bash
cargo newday 01
```

This will make three files for you, the `src/bin` file, the `samples` file, and
the `data` file.

## Downloading input

Download the input files to `data/<number>.txt`. For example, `data/01.txt`.

To download the input from the CLI, use:

```bash
cargo getdata 2024 01 --session $AOC_TOKEN > data/2024/01.txt
```

The AOC token is in your cookies, and you can get it from the browser. You can
set the environment variable `AOC_TOKEN` to avoid typing it each time.

The `newday` command above also accepts `--session`/`AOC_TOKEN`, which will
cause it to download the input for you if it's available.

See <https://github.com/gobanos/cargo-aoc> which inspired this feature and has
more info about getting your token. In Safari, it's in the devtools inspection
pane.

## Running

Use:

```bash
cargo 2024 01
```

A few days accept `--vis` to add a visual.

## Docs

You can build with:

```bash
cargo docs --no-deps --workspace
```

## Profiling

I've used [samply](https://github.com/mstange/samply) to profile. After
installing (`cargo install --locked samply`), you can use `samply record
target/bin/XX` and open the page in Chrome or Firefox. I've enabled debug info
in release mode for this. You can rebuild Rust's stdlib with debuginfo for more
fine grained detail, but I haven't needed that. I have replaced `par_iter` with
`iter` when doing this, otherwise Rayon hides what's happening. I could use a
feature to automate this, but I think keeping usage simple is fine.

## Debugging

To use lldb, just do the following:

```console
$ cargo build --package year_2024 --bin XX
$ lldb target/debug/XX
> breakpoint set -f crates/year_2024/src/XX.rs -l YY
> r
```

It should work like any other executable, mostly. The visual debugger in VSCode
might work, though I had a hard time telling it to run the correct executable.
Note that LLDB only supports the subset of Rust and C for conditional
breakpoints, so you might have to manually add code and use line based
breakpoints instead.

## Notes

Also see [Blessed.rs](https://blessed.rs), a curated list of good Rust libraries.
