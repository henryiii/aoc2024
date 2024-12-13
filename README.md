# Advent of Code 2024 in Rust

I'm trying the [advent of code](https://adventofcode.com/2024) in Rust again this
year to learn more Rust. I'm not trying to be fast and place on the leaderboards
(which also require working at midnight, which I'm mostly not interested in
doing), I'm trying to be somewhat elegant and learn new things in Rust. The
documentation is [live here](https://henryiii.github.io/aoc2024).

I highly recommend loading this up in a good editor, like Visual Studio Code or
VIM with the ALE plugin. It will add type information to all inferred types,
autocomplete, show documentation, etc.

Feel free to check out my 2023 repo at <https://github.com/henryiii/aoc2023>.

## Formatting and linting

Use:

```bash
cargo fmt && cargo clippy-all
```

If you want to auto-fix anything, you can:

```bash
cargo clippy --fix --allow-dirty --allow-staged
```

I also looked for removable features using
[unused-features](https://crates.io/crates/cargo-unused-features), both to
speed up compilation and it helped removed a small dependence on unicode in
regex.

## Tests

Use:

```bash
cargo test
```

Useful flags include `-- --nocapture` and `--bin <NUMBER>` for just one set of tests.

If you have `cargo-nextest` (say, from `brew install cargo-nextest`), then
`cargo nextest run` also works.

## New days

To create a new day, use:

```bash
cargo newday 01
```

This will make three files for you, the `src/bin` file, the `samples` file, and
the `data` file.

## Running

Download the input files to `data/<number>.txt`. For example, `data/01.txt`.

Use:

```bash
cargo day 01
```

## Docs

You can build with:

```bash
cargo docs --no-deps
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
$ cargo build --bin XX
$ lldb target/debug/XX
> breakpoint set -f src/bin/XX.rs -l YY
> r
```

It should work like any other executable, mostly. The visual debugger in VSCode
should work too (haven't checked yet).

## Notes

Also see [Blessed.rs](https://blessed.rs), a curated list of good Rust libraries.
