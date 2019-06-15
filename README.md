# RAWS

Re-implementation of [aws-profile-util](https://github.com/hpcsc/aws-profile-utils) using Rust

### Build Status
[![Build Status](https://travis-ci.org/hpcsc/raws.png)](https://travis-ci.org/hpcsc/raws)

### Installation

- Download latest build from [https://dl.bintray.com/hpcsc/raws](https://dl.bintray.com/hpcsc/raws)
- Move to `/usr/local/bin`

    ```
    chmod +x ./raws && mv ./raws /usr/local/bin
    ```

### Usage

```
USAGE:
    raws [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get     get current AWS profile (that is set to default profile)
    help    Prints this message or the help of the given subcommand(s)
    set     set default profile with credentials of selected profile (this command assumes fzf is already setup)
```
