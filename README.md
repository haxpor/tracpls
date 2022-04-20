# tracpls
Cli tool to get smart contract code + ABI on BSC chain for ease of piping and viewing at terminal (e.g. with vim or others)

# Installation

```
cargo install tracpls
```

# Usage

Users are required to define environment variable of `TRACPLS_BSCSCAN_APIKEY` to
be bscscan's apikey. Check bscscan.com website to sign up an account, then
create an apikey.

The following options are available

```
tracpls
Wasin Thonkaew (wasin@wasin.io)
cli tool to get smart contract code and its ABI for ease of viewing on terminal

USAGE:
    tracpls [OPTIONS] --address <ADDRESS>

OPTIONS:
    -a, --address <ADDRESS>      Target contract address to get its smart contract code or ABI from
        --abi-only               Get only contract ABI
    -h, --help                   Print help information
        --no-abi-pretty-print    Pretty print output for contract ABI. It can only be used if --abi-
                                 only exists
        --no-clean-crlf          Make sure to clean CR/LF character codes to make it suitable to
                                 view the content on the platform running the application
```

# Examples

1. Get smart contract then pipe directly to vim

```
$ tracpls -a 0x0000000000000000000000000000000000001004 | vim -c "set syntax=solidity" -
```

You might not need `-c "set syntax=solidity"` if you already configured your
`~/.vimrc` to support solidity syntax highlighting.

2. Get contract's ABI then save to file

```
$ tracpls -a 0x0000000000000000000000000000000000001004 --abi-only > abi.json
```

3. Same as 2. but with no pretty print

```
$ tracpls -a 0x0000000000000000000000000000000000001004 --abi-only --no-abi-pretty-print > abi.json
```

# Note

Error message will always be outputted to `stderr`. So normal correct and proper
output won't be interfere with the error message. But you are free to combine
them into one.

# License
MIT, Wasin Thonkaew
