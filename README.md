ipinfo is a tool to print infos about a IP Address with an optional network.

The `INPUT` can be either an IP Address (IPv4 or IPv6) or a Network in [CIDR](https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing) notation.
The tool will output a bunch of information about this IP Address including the Address Family, if it is a global routed address, etc.
If a Network Mask (aka Prefix Length) is specified, additional information about the according network will be printed, too.

The tool is also able to generate [JSON](https://en.wikipedia.org/wiki/JSON) output which contains the same information but in machine readable format.


## Installation
To build the tool a [Rust-Toolchain](https://www.rust-lang.org/) is required.

Download the source and run the following command to build:
```shell script
cargo build --release
```

The result can be found in `./target /release/ipinfo`.

## Usage
```
USAGE:
    ipinfo [FLAGS] <input>

FLAGS:
    -h, --help       Prints help information
    -j, --json       Output as JSON structure
    -V, --version    Prints version information

ARGS:
    <input>    IP address or subnet in CIDR format
```

You will figure this out, right?


## Bugs and Enhancements
I'm open for new feature ideas (and according pull-requests).
Feel free to use the issues to tell me abpout Bugs, Ideas and Use-Cases.


## License
The code is published under WTFPL.
