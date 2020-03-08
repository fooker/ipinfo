use std::io::Write;

use anyhow::Result;
use json::{JsonValue, stringify_pretty};

use crate::details::{Details, Family};

pub fn display(mut w: impl Write, details: Details) -> Result<()> {
    let mut data = JsonValue::new_object();

    data.insert("family", match details.family {
        Family::IPv4 => "ipv4",
        Family::IPv6 => "ipv6",
    })?;

    data.insert("address", details.address)?;

    data.insert("unspecified", details.kind.unspecified)?;
    data.insert("loopback", details.kind.loopback)?;
    data.insert("multicast", details.kind.multicast)?;

    if let Some(network) = details.network {
        data.insert("netmask", network.netmask)?;
        data.insert("network", network.network)?;
    }

    writeln!(w, "{}", stringify_pretty(data, 2))?;

    return Ok(());
}