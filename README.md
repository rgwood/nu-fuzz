# Nushell fuzzer

Install [`pledge`](https://justine.lol/pledge/) somewhere in your PATH

Run with something like:
`cargo fuzz run fuzz_target_1 --jobs=16 -- -rss_limit_mb=300`

WARNING: this fuzzer is executing random input. It should be safe due to the sandboxing provided by `pledge`, but I can't make any guarantees.