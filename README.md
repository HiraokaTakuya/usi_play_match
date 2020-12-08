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
    usi_play_match [FLAGS] <target> <reference> <target_eval_dir> <reference_eval_dir> <target_threads> <reference_threads> <target_usi_hash> <reference_usi_hash> <parallel_num> <match_num> <movetime>

FLAGS:
    -h, --help       Prints help information
        --nodes      Uses movetime as nodes
    -V, --version    Prints version information

ARGS:
    <target>                Target USI engine
    <reference>             Reference USI engine
    <target_eval_dir>       Target engine Eval_Dir ("-" is default Eval_Dir)
    <reference_eval_dir>    Reference engine Eval_Dir ("-" is default Eval_Dir)
    <target_threads>        Target engine Threads
    <reference_threads>     Reference engine Threads
    <target_usi_hash>       Target engine USI_Hash
    <reference_usi_hash>    Reference engine USI_Hash
    <parallel_num>          Parallel Num
    <match_num>             Match Num
    <movetime>              movetime
```

Output sample.

```
(    1) W:     0 L:     1 D:     0 WR:   0.00% +-  0.00%(95%)   0.00%(99%) Elo: None
(    2) W:     1 L:     1 D:     0 WR:  50.00% +- 69.30%(95%)  91.22%(99%) Elo:    0
(    3) W:     2 L:     1 D:     0 WR:  66.67% +- 53.34%(95%)  70.22%(99%) Elo:  120
(    4) W:     2 L:     2 D:     0 WR:  50.00% +- 49.00%(95%)  64.50%(99%) Elo:    0
(    5) W:     3 L:     2 D:     0 WR:  60.00% +- 42.94%(95%)  56.52%(99%) Elo:   70
(    6) W:     4 L:     2 D:     0 WR:  66.67% +- 37.72%(95%)  49.65%(99%) Elo:  120
(    7) W:     5 L:     2 D:     0 WR:  71.43% +- 33.47%(95%)  44.05%(99%) Elo:  159
(    8) W:     5 L:     3 D:     0 WR:  62.50% +- 33.55%(95%)  44.16%(99%) Elo:   89
(    9) W:     5 L:     4 D:     0 WR:  55.56% +- 32.46%(95%)  42.73%(99%) Elo:   39
(   10) W:     5 L:     5 D:     0 WR:  50.00% +- 30.99%(95%)  40.79%(99%) Elo:    0
```

## License

This is distributed under the MIT license.

See LICENSE-MIT for details.
