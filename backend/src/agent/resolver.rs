use actix::{Actor, Addr, Context, Handler, Message, MessageResponse};

use std::net::*;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

use log::info;

#[derive(MessageResponse)]
pub enum ResolveResp {
    Success(IpAddr),
    Failed,
}

#[derive(Message)]
#[rtype(result = "ResolveResp")]
pub enum ResolveMsg {
    Resolve(String),
}

pub struct DnsResolver {
    resolver: Resolver,
}

impl Actor for DnsResolver {
    type Context = Context<Self>;
}

impl Handler<ResolveMsg> for DnsResolver {
    type Result = ResolveResp;

    fn handle(&mut self, msg: ResolveMsg, _: &mut Context<Self>) -> Self::Result {
        match msg {
            ResolveMsg::Resolve(name) => {
                if let Ok(response) = self.resolver.lookup_ip(name.clone()) {
                    if let Some(address) = response.iter().next() {
                        info!("Resolved {} to {}", name, address);
                        ResolveResp::Success(address)
                    } else {
                        ResolveResp::Failed
                    }
                } else {
                    info!("Failed to resolve {}", name);
                    ResolveResp::Failed
                }
            }
        }
    }
}

impl DnsResolver {
    pub fn new() -> Addr<Self> {
        let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

        DnsResolver { resolver }.start()
    }
}

// Construct a new Resolver with default configuration options
// let mut resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

// On Unix/Posix systems, this will read the /etc/resolv.conf
// let mut resolver = Resolver::from_system_conf().unwrap();

// Lookup the IP addresses associated with a name.
// let mut response = resolver.lookup_ip("www.example.com.").unwrap();

// There can be many addresses associated with the name,
//  this can return IPv4 and/or IPv6 addresses
// let address = response.iter().next().expect("no addresses returned!");
// if address.is_ipv4() {
//     assert_eq!(address, IpAddr::V4(Ipv4Addr::new(93, 184, 216, 34)));
// } else {
//     assert_eq!(address, IpAddr::V6(Ipv6Addr::new(0x2606, 0x2800, 0x220, 0x1, 0x248, 0x1893, 0x25c8, 0x1946)));
// }
