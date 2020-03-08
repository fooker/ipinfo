use std::net::IpAddr;

use anyhow::{Result, anyhow};
use clap::{App, Arg};
use ip_network::IpNetwork;

use crate::details::Details;
use crate::output::{Output, display};

mod details;
mod output;

pub enum Input {
    Pure(IpAddr),
    CIDR(IpAddr, IpNetwork),
}

fn main() -> Result<()> {
    let matches = App::new("IP info")
        .version("0.1")
        .author("Dustin Frisch <fooker@lab.sh>")
        .about("Displays info about an IP address or subnet")
        .arg(Arg::with_name("json")
            .short("j")
            .long("json")
            .help("Output as JSON structure"))
        .arg(Arg::with_name("INPUT")
            .help("IP address or subnet in CIDR format")
            .required(true)
            .index(1))
        .get_matches();

    let output = if matches.is_present("json") {
        Output::JSON
    } else {
        Output::HUMAN
    };

    let input = matches.value_of("INPUT").expect("Input missing");
    let input = parse_input(input).map_err(|err| anyhow!("Invalid input: {}", err))?;

    let details: Details = input.into();

    display(std::io::stdout().lock(), output, details)?;

    return Ok(());
}

fn parse_input(s: &str) -> Result<Input> {
    if let Some(i) = s.find('/') {
        let address = s[..(i - 0)].parse()?;
        let netmask = s[(i + 1)..].parse()?;

        let network = IpNetwork::new_truncate(address, netmask)?;

        return Ok(Input::CIDR(address, network));
    } else {
        return Ok(Input::Pure(s.parse()?));
    }
}
