# usi_play_match

Play matches 2 usi engines, and output results.

## Install

1. Install rustup and cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

See detail.
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

2. Install usi_play_match

```bash
cargo install --git https://github.com/HiraokaTakuya/usi_play_match.git
```

## Usage

```bash
USAGE:
    usi_play_match [FLAGS] <target> <reference> <target-eval-dir> <reference-eval-dir> <target-threads> <reference-threads> <target-usi-hash> <reference-usi-hash> <parallel-num> <match-num> <movetime>

FLAGS:
    -h, --help       Prints help information
        --nodes      Uses movetime as nodes
    -V, --version    Prints version information

ARGS:
    <target>                Target USI engine
    <reference>             Reference USI engine
    <target-eval-dir>       Target engine Eval_Dir ("-" is default Eval_Dir)
    <reference-eval-dir>    Reference engine Eval_Dir ("-" is default Eval_Dir)
    <target-threads>        Target engine Threads
    <reference-threads>     Reference engine Threads
    <target-usi-hash>       Target engine USI_Hash
    <reference-usi-hash>    Reference engine USI_Hash
    <parallel-num>          Parallel Num
    <match-num>             Match Num
    <movetime>              movetime
```

Output sample.

```
(    1) W:     0 L:     1 D:     0 WR:   0.00% +-  0.00%(95%)   0.00%(99%) Elo: None
(    2) W:     1 L:     1 D:     0 WR:  50.00% +- 69.30%(95%)  91.22%(99%) Elo:    0
(    3) W:     1 L:     1 D:     1 WR:  50.00% +- 56.58%(95%)  74.48%(99%) Elo:    0
(    4) W:     2 L:     1 D:     1 WR:  62.50% +- 47.44%(95%)  62.45%(99%) Elo:   89
(    5) W:     2 L:     2 D:     1 WR:  50.00% +- 43.83%(95%)  57.69%(99%) Elo:    0
(    6) W:     3 L:     2 D:     1 WR:  58.33% +- 39.45%(95%)  51.93%(99%) Elo:   58
(    7) W:     3 L:     3 D:     1 WR:  50.00% +- 37.04%(95%)  48.76%(99%) Elo:    0
(    8) W:     3 L:     4 D:     1 WR:  43.75% +- 34.38%(95%)  45.25%(99%) Elo:  -44
(    9) W:     4 L:     4 D:     1 WR:  50.00% +- 32.67%(95%)  43.00%(99%) Elo:    0
(   10) W:     5 L:     4 D:     1 WR:  55.00% +- 30.83%(95%)  40.59%(99%) Elo:   35
```

## License

This is distributed under the MIT license.

See LICENSE-MIT for details.
