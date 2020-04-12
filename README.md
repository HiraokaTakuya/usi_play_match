# usi_play_match

Play matches 2 usi engines, and output results.

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

## License

This is distributed under the MIT license.

See LICENSE-MIT for details.
