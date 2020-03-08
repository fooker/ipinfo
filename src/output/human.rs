use std::io::Write;

use anyhow::Result;

use crate::details::{Details, Family, Kind};

fn kind(kind: Kind) -> String {
    let mut flags = Vec::new();

    if kind.unspecified {
        flags.push("unspecified");
    }

    if kind.loopback {
        flags.push("loopback");
    }

    if kind.multicast {
        flags.push("multicast");
    }

    return flags.join(" ");
}

pub fn display(mut w: impl Write, details: Details) -> Result<()> {
    let input = if let Some(ref network) = details.network {
        format!("{}/{}", details.address, network.netmask)
    } else {
        details.address
    };

    writeln!(w, "{} [{}]", input, kind(details.kind))?;
    writeln!(w, "  Family: {}", match details.family {
        Family::IPv4 => "IPv4",
        Family::IPv6 => "IPv6",
    })?;

    if let Some(ref network) = details.network {
        writeln!(w, "  Network: {}", network.network)?;
    }

    return Ok(());
}