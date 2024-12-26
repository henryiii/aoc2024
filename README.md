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

On my MacBook (Intel), these are my timings (total program runtime 640ms, was 1223ms before rewriting day 16):

```text
Day 01
Solution A: 1197984    (0.110ms)
Solution B: 23387399    (0.139ms)
Time taken: 0.767ms
Day 02
Solution A: 486    (0.438ms)
Solution B: 540    (0.452ms)
Time taken: 1.623ms
Day 03
Solution A: 165225049    (0.464ms)
Solution B: 108830766    (0.477ms)
Time taken: 1.520ms
Day 04
Solution A: 2464    (0.763ms)
Solution B: 1982    (0.950ms)
Time taken: 2.078ms
Day 05
Solution A: 5208    (0.998ms)
Solution B: 6732    (0.972ms)
Time taken: 2.526ms
Day 06
Solution A: 5312    (0.131ms)
Solution B: 1748    (13.497ms)
Time taken: 14.147ms
Day 07
Solution A: 12940396350192    (0.882ms)
Solution B: 106016739583593    (7.081ms)
Time taken: 8.512ms
Day 08
Solution A: 228    (0.143ms)
Solution B: 766    (0.179ms)
Time taken: 1.142ms
Day 09
Solution A: 6471961544878    (4.996ms)
Solution B: 6511178035564    (78.937ms)
Time taken: 84.780ms
Day 10
Solution A: 794    (1.275ms)
Solution B: 1706    (1.389ms)
Time taken: 4.157ms
Day 11
Solution A: 194782    (0.175ms)
Solution B: 233007586663131    (7.913ms)
Time taken: 8.663ms
Day 12
Solution A: 1477762    (14.772ms)
Solution B: 923480    (17.588ms)
Time taken: 33.411ms
Day 13
Solution A: 27157    (0.608ms)
Solution B: 104015411578548    (0.389ms)
Time taken: 2.036ms
Day 14
Solution A: 228457125    (0.197ms)
Solution B: 6493    (24.696ms)
Time taken: 25.718ms
Day 15
Solution A: 1457740    (1.695ms)
Solution B: 1467145    (7.208ms)
Time taken: 10.240ms
Day 16
Solution A: 72400    (14.226ms)
Solution B: 435    (66.575ms)
Time taken: 80.892ms
Day 17
Solution A: 7,4,2,5,1,4,6,0,4    (0.018ms)
Solution B: 164278764924605    (0.124ms)
Time taken: 1.591ms
Day 18
Solution A: 320    (4.596ms)
Solution B: 34,40    (152.103ms)
Time taken: 157.803ms
Day 19
Solution A: 300    (24.618ms)
Solution B: 624802218898092    (69.590ms)
Time taken: 95.798ms
Day 20
Solution A: 1524    (17.897ms)
Solution B: 1033746    (18.670ms)
Time taken: 37.071ms
Day 21
Solution A: 137870    (0.068ms)
Solution B: 170279148659464    (0.332ms)
Time taken: 1.962ms
Day 22
Solution A: 18694566361    (11.455ms)
Solution B: 2100    (39.084ms)
Time taken: 51.247ms
Day 23
Solution A: 1512    (33.596ms)
Solution B: ac,ed,fh,kd,lf,mb,om,pe,qt,uo,uy,vr,wg    (20.583ms)
Time taken: 55.113ms
Day 24
Solution A: 57270694330992    (3.280ms)
Solution B: gwh,jct,rcb,wbw,wgb,z09,z21,z39    (0.786ms)
Time taken: 4.176ms
Day 25
Solution A: 3287    (2.986ms)
Solution B: 0    (0.000ms)
Time taken: 3.108ms
```
