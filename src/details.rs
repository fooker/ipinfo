use std::net::IpAddr;
use ip_network::IpNetwork;

use crate::Input;

pub enum Family {
    IPv4,
    IPv6,
}

pub struct Network {
    pub netmask: u8,
    pub network: String,
}

impl From<IpNetwork> for Network {
    fn from(network: IpNetwork) -> Self {
        return Self {
            netmask: network.netmask(),
            network: network.network_address().to_string(),
        };
    }
}

pub struct Kind {
    pub unspecified: bool,
    pub loopback: bool,
    pub multicast: bool,
}

impl From<IpAddr> for Kind {
    fn from(address: IpAddr) -> Self {
        return Self {
            unspecified: address.is_unspecified(),
            loopback: address.is_loopback(),
            multicast: address.is_multicast(),
        };
    }
}

pub struct Details {
    pub family: Family,
    pub address: String,

    pub kind: Kind,

    pub network: Option<Network>,
}

impl Details {
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = Some(network);
        return self;
    }
}

impl From<IpAddr> for Details {
    fn from(address: IpAddr) -> Self {
        return Self {
            family: match address {
                IpAddr::V4(_) => Family::IPv4,
                IpAddr::V6(_) => Family::IPv6,
            },
            address: address.to_string(),
            kind: Kind::from(address),
            network: None,
        };
    }
}

impl From<Input> for Details {
    fn from(input: Input) -> Self {
        return match input {
            Input::Pure(address) => address.into(),
            Input::CIDR(address, network) => Details::from(address).with_network(Network::from(network)),
        };
    }
}

