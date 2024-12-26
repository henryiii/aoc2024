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


## Final timings

On my MacBook (Intel), these are my timings (total program runtime 496ms, was 1223ms originally):

```text
Day 01
Solution A: 1197984    (0.149ms)
Solution B: 23387399    (0.194ms)
Time taken: 0.424ms
Day 02
Solution A: 486    (0.510ms)
Solution B: 540    (0.590ms)
Time taken: 1.142ms
Day 03
Solution A: 165225049    (0.588ms)
Solution B: 108830766    (0.646ms)
Time taken: 1.280ms
Day 04
Solution A: 2464    (0.979ms)
Solution B: 1982    (1.193ms)
Time taken: 2.222ms
Day 05
Solution A: 5208    (1.415ms)
Solution B: 6732    (1.299ms)
Time taken: 2.780ms
Day 06
Solution A: 5312    (0.169ms)
Solution B: 1748    (15.697ms)
Time taken: 15.955ms
Day 07
Solution A: 12940396350192    (0.989ms)
Solution B: 106016739583593    (7.491ms)
Time taken: 8.614ms
Day 08
Solution A: 228    (0.121ms)
Solution B: 766    (0.130ms)
Time taken: 0.319ms
Day 09
Solution A: 6471961544878    (5.007ms)
Solution B: 6511178035564    (84.903ms)
Time taken: 89.977ms
Day 10
Solution A: 794    (1.283ms)
Solution B: 1706    (1.180ms)
Time taken: 2.521ms
Day 11
Solution A: 194782    (0.217ms)
Solution B: 233007586663131    (7.066ms)
Time taken: 7.312ms
Day 12
Solution A: 1477762    (12.501ms)
Solution B: 923480    (17.244ms)
Time taken: 29.902ms
Day 13
Solution A: 27157    (0.578ms)
Solution B: 104015411578548    (0.316ms)
Time taken: 0.961ms
Day 14
Solution A: 228457125    (0.259ms)
Solution B: 6493    (26.993ms)
Time taken: 27.298ms
Day 15
Solution A: 1457740    (1.269ms)
Solution B: 1467145    (7.383ms)
Time taken: 8.726ms
Day 16
Solution A: 72400    (15.688ms)
Solution B: 435    (22.294ms)
Time taken: 38.059ms
Day 17
Solution A: 7,4,2,5,1,4,6,0,4    (0.018ms)
Solution B: 164278764924605    (0.106ms)
Time taken: 0.186ms
Day 18
Solution A: 320    (4.663ms)
Solution B: 34,40    (22.072ms)
Time taken: 26.791ms
Day 19
Solution A: 300    (25.500ms)
Solution B: 624802218898092    (61.810ms)
Time taken: 87.409ms
Day 20
Solution A: 1524    (15.204ms)
Solution B: 1033746    (16.505ms)
Time taken: 31.793ms
Day 21
Solution A: 137870    (0.066ms)
Solution B: 170279148659464    (0.284ms)
Time taken: 0.414ms
Day 22
Solution A: 18694566361    (9.778ms)
Solution B: 2100    (34.496ms)
Time taken: 44.336ms
Day 23
Solution A: 1512    (32.724ms)
Solution B: ac,ed,fh,kd,lf,mb,om,pe,qt,uo,uy,vr,wg    (18.433ms)
Time taken: 51.254ms
Day 24
Solution A: 57270694330992    (3.131ms)
Solution B: gwh,jct,rcb,wbw,wgb,z09,z21,z39    (0.732ms)
Time taken: 3.934ms
Day 25
Solution A: 3287    (3.423ms)
Solution B: 0    (0.000ms)
Time taken: 3.468ms
```
