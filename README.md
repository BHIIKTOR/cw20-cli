# cw20-cli

## Help

```
Usage: mysupercontract --chain <CHAIN> <COMMAND>

Commands:
  deploy      Deploy contract to configurable chain
  initialize  Initialize our contract
  transfer    Transfer tokens
  find-tx     Search TX by hash
  help        Print this message or the help of the given subcommand(s)

Options:
  -c, --chain <CHAIN>  Name of the target chain. NOTE: testing is local juno [possible values: UNI_6, JUNO_1, TESTING, PISCO_1, PHOENIX_1, LOCAL_TERRA, INJECTIVE_888, CONSTANTINE_1, BARYON_1, INJECTIVE_1, HARPOON_4, OSMO_4, LOCAL_OSMO]
  -h, --help           Print help
  -V, --version        Print version
```

## Usage examples

```
cargo run -- -c TESTING deploy -n mycontractooor

cargo run -- -c TESTING initialize -n token -s TOK -d 12
```

