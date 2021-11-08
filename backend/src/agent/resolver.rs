// use actix::{Actor, Addr, Context, Handler, Message, MessageResponse};

// use std::net::*;
// use trust_dns_resolver::config::*;
// use trust_dns_resolver::Resolver;

// use log::info;

// #[derive(MessageResponse)]
// pub enum ResolveResp {
//     Success(IpAddr),
//     Failed,
// }

// #[derive(Message)]
// #[rtype(result = "ResolveResp")]
// pub enum ResolveMsg {
//     Resolve(String),
// }

// pub struct DnsResolver {
//     resolver: Resolver,
// }

// impl Actor for DnsResolver {
//     type Context = Context<Self>;
// }

// impl Handler<ResolveMsg> for DnsResolver {
//     type Result = ResolveResp;

//     fn handle(&mut self, msg: ResolveMsg, _: &mut Context<Self>) -> Self::Result {
//         match msg {
//             ResolveMsg::Resolve(name) => {
//                 if let Ok(response) = self.resolver.lookup_ip(name.clone()) {
//                     if let Some(address) = response.iter().next() {
//                         info!("Resolved {} to {}", name, address);
//                         ResolveResp::Success(address)
//                     } else {
//                         ResolveResp::Failed
//                     }
//                 } else {
//                     info!("Failed to resolve {}", name);
//                     ResolveResp::Failed
//                 }
//             }
//         }
//     }
// }

// impl DnsResolver {
//     pub fn new() -> Addr<Self> {
//         let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

//         DnsResolver { resolver }.start()
//     }
// }
use std::net::IpAddr;

use trust_dns_resolver::{
    config::*,
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
};
use trust_dns_resolver::{AsyncResolver, TokioHandle};

use log::*;

pub struct DnsResolver {
    resolver: AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>,
}

impl DnsResolver {
    pub fn new() -> Self {
        let resolver = AsyncResolver::new(
            ResolverConfig::default(),
            ResolverOpts::default(),
            TokioHandle,
        )
        .unwrap();

        Self { resolver }
    }

    pub async fn lockup(&self, name: String) -> Option<IpAddr> {
        let lookup = self.resolver.lookup_ip(name.clone());

        if let Ok(response) = lookup.await {
            if let Some(address) = response.iter().next() {
                info!("Resolved {} to {}", name, address);
                Some(address)
            } else {
                info!("Failed to resolve {}", name);
                None
            }
        } else {
            info!("Failed to resolve {}", name);
            None
        }
    }
}
