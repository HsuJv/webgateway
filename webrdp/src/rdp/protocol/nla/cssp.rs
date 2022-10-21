#![allow(dead_code)]

use super::ntlm::*;
use super::{super::ber::*, to_unicode};
use crate::rdp::rdp_impl::{ConnectCb, FailCb, RdpInitializer, RdpInner};
use x509_parser::prelude::*;

enum State {
    Init,
    ServerPubkeyWait,
    ClientNegoSent,
    ClientChalSent,
    ServerAuthRecv,
}

pub struct CsspClient {
    on_connect: ConnectCb,
    on_fail: FailCb,
    server_key: Vec<u8>,
    state: State,
    ntlm: Ntlm,
    username: String,
    password: String,
    domain: String,
    hostname: String,
    send_seq: u32,
    recv_seq: u32,
}

impl CsspClient {
    pub fn new(on_connect: ConnectCb, on_fail: FailCb, u: &str, p: &str, d: &str, h: &str) -> Self {
        Self {
            on_connect,
            on_fail,
            server_key: Vec::new(),
            state: State::Init,
            ntlm: Ntlm::new(u, p, d, h),
            username: u.to_string(),
            password: p.to_string(),
            domain: d.to_string(),
            hostname: h.to_string(),
            send_seq: 0,
            recv_seq: 0,
        }
    }
}

impl RdpInitializer for CsspClient {
    fn hello(&mut self, rdp: &mut RdpInner) {
        self.state = State::ServerPubkeyWait;
        self.ntlm
            .init(ISC_REQ_MUTUAL_AUTH | ISC_REQ_CONFIDENTIALITY | ISC_REQ_USE_SESSION_KEY);
        rdp.wait(1);
    }
    fn do_input(&mut self, rdp: &mut RdpInner) {
        match self.state {
            State::Init => unreachable!(),
            State::ServerPubkeyWait => self.handle_server_publickey(rdp),
            State::ClientNegoSent => self.handle_server_challenge(rdp),
            State::ClientChalSent => self.handle_server_auth(rdp),
            State::ServerAuthRecv => unreachable!(),
        }
    }
}

impl CsspClient {
    fn handle_server_publickey(&mut self, rdp: &mut RdpInner) {
        let x509_der = rdp.reader.read_to_end();
        let x509 = X509Certificate::from_der(&x509_der).unwrap();
        self.server_key = x509
            .1
            .tbs_certificate
            .subject_pki
            .subject_public_key
            .data
            .to_vec();
        self.ntlm.generate_nego();

        self.send_tsrequest(rdp, self.ntlm.get_nego_msg());
        self.state = State::ClientNegoSent;
        rdp.wait(3); // any valid ans.1 struct
    }

    fn handle_server_challenge(&mut self, rdp: &mut RdpInner) {
        let server_challenge = rdp.reader.read_to_end();
        let ans1_tree = BerObj::from_der(&server_challenge);
        // console_log!("ans1_tree {:#?}", ans1_tree);
        if let ASN1Type::SequenceOwned(ref seq) = ans1_tree.get_value() {
            if let ASN1Type::PrivOwned(nego_tokens) = seq[1].get_value() {
                if let ASN1Type::SequenceOwned(ref nego_seq) = nego_tokens.get_value() {
                    if let ASN1Type::SequenceOwned(ref nego_items) = nego_seq[0].get_value() {
                        if let ASN1Type::PrivOwned(nego_item) = nego_items[0].get_value() {
                            if let ASN1Type::OctetString(server_chal) = nego_item.get_value() {
                                self.ntlm.generate_client_chal(server_chal);
                            } else {
                                (self.on_fail)(rdp, "Wrong nla response");
                                return;
                            }
                        } else {
                            (self.on_fail)(rdp, "Wrong nla response");
                            return;
                        }
                    } else {
                        (self.on_fail)(rdp, "Wrong nla response");
                        return;
                    }
                } else {
                    (self.on_fail)(rdp, "Wrong nla response");
                    return;
                }
            } else {
                (self.on_fail)(rdp, "Wrong nla response");
                return;
            }
        } else {
            (self.on_fail)(rdp, "Wrong nla response");
            return;
        }
        let pubkey_auth = self.ntlm.encrypt(&self.server_key, self.send_seq);
        self.send_seq += 1;
        self.send_ts_challenge(rdp, self.ntlm.get_chal_msg(), &pubkey_auth);
        self.state = State::ClientChalSent;
        rdp.wait(3); // any valid ans.1 struct
    }

    fn handle_server_auth(&mut self, rdp: &mut RdpInner) {
        let _ = rdp.reader.read_to_end();

        let auth_data = ber_seq!(
            /* [0] credType (INTEGER) */
            ber!(0, &ber!(1)),
            /* [1] credentials (OCTET STRING) */
            ber!(
                1,
                &ber!(
                    /* make the whole credentials as an octet string */
                    &ber_seq!(
                        ber!(0, &ber!(to_unicode(&self.domain)[..])),
                        ber!(1, &ber!(to_unicode(&self.username)[..])),
                        ber!(2, &ber!(to_unicode(&self.password)[..]))
                    )
                    .to_der()[..]
                )
            )
        )
        .to_der();
        let auth = self.ntlm.encrypt(&auth_data, self.send_seq);
        self.send_seq += 1;
        self.send_ts_auth(rdp, &auth);
        self.state = State::ServerAuthRecv;
        (self.on_connect)(rdp)
    }

    fn send_tsrequest(&self, rdp: &mut RdpInner, nego: &[u8]) {
        let output = ber_seq!(
            /* [0] version, 2 */
            ber!(0, &ber!(2)),
            /* [1] negoTokens(NegoData) */
            ber!(
                1,
                /*  SEQUENCE OF NegoDataItem */
                &ber_seq!(/* NegoDataItem */ ber_seq!(ber!(0, &ber!(nego))))
            )
        )
        .to_der();
        rdp.writer.write_slice(&output);
    }

    fn send_ts_challenge(&self, rdp: &mut RdpInner, nego: &[u8], pubkey: &[u8]) {
        let output = ber_seq!(
            /* [0] version, 2 */
            ber!(0, &ber!(2)),
            /* [1] negoTokens(NegoData) */
            ber!(
                1,
                /*  SEQUENCE OF NegoDataItem */
                &ber_seq!(/* NegoDataItem */ ber_seq!(ber!(0, &ber!(nego))))
            ),
            /* [3] pubKeyAuth (OCTET STRING) */
            ber!(3, &ber!(pubkey))
        )
        .to_der();
        rdp.writer.write_slice(&output);
    }

    fn send_ts_auth(&self, rdp: &mut RdpInner, auth: &[u8]) {
        let output = ber_seq!(
            /* [0] version, 2 */
            ber!(0, &ber!(2)),
            /* [2] authInfo (OCTET STRING) */
            ber!(2, &ber!(auth))
        )
        .to_der();
        rdp.writer.write_slice(&output);
    }
}
