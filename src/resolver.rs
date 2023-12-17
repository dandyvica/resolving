use crate::error::Error;
use std::net::IpAddr;

#[cfg(target_family = "unix")]
use std::str::FromStr;

#[cfg(target_family = "windows")]
use windows::Win32::{
    Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_INVALID_PARAMETER, ERROR_SUCCESS},
    NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX, IP_ADAPTER_ADDRESSES_LH,
    },
    Networking::WinSock::{AF_INET, AF_INET6, AF_UNSPEC, SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6},
};

#[derive(Debug)]
pub struct Resolvers {
    pub v4: Vec<IpAddr>,
    pub v6: Vec<IpAddr>,
}

impl Resolvers {
    #[cfg(target_family = "unix")]
    /// Return IPV4 & IPV6 DNS resolvers on the machine.
    pub fn get_servers(conf: Option<&str>) -> Result<Self, Error> {
        const RESOLV_CONF_FILE: &str = "/etc/resolv.conf";

        // resolv file is usually at "/etc/resolv.conf" but some distros (Ubuntu) moved it elsewhere
        let resolv_file = conf.unwrap_or(RESOLV_CONF_FILE);

        // read whole file, get rid of comments and extract DNS stubs
        let resolv_conf = std::fs::read_to_string(resolv_file)?;

        let servers: Vec<IpAddr> = resolv_conf
            .lines()
            .filter(|line| line.trim().starts_with("nameserver"))
            .filter_map(|addr| addr.split_ascii_whitespace().nth(1))
            .map(IpAddr::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        if servers.is_empty() {
            return Err(Error::NoResolverConfigured);
        }

        let v4: Vec<IpAddr> = servers.iter().filter(|x| x.is_ipv4()).cloned().collect();
        let v6: Vec<IpAddr> = servers.iter().filter(|x| x.is_ipv6()).cloned().collect();

        Ok(Self { v4, v6 })
    }

}
