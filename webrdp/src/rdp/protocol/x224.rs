#![allow(dead_code)]

use super::super::rdp_impl::RdpInner;
use super::super::*;

const RDP_X224_VER: u8 = 3;
const RDP_X224_LEN: u8 = 0x13;
const TPTK_HDR_LEN: u8 = 4;
const RDP_NEG_REQ: u8 = 1;
const RDP_NEG_RSP: u8 = 2;
const RDP_NEG_FAIL: u8 = 3;

const PROTOCOL_RDP: u32 = 0;
const PROTOCOL_SSL: u32 = 1;
const PROTOCOL_HYBRID: u32 = 2;
const PROTOCOL_RDSTLS: u32 = 4;
const PROTOCOL_HYBRID_EX: u32 = 8;
const PROTOCOL_RDSAAD: u32 = 16;

const SSL_REQUIRED_BY_SERVER: u32 = 0x00000001;
const SSL_REQUIRED_BY_SERVER_MSG: &str = "The server requires that the client support Enhanced RDP Security (section 5.4) with either TLS 1.0, 1.1 or 1.2 (section 5.4.5.1) or CredSSP (section 5.4.5.2). If only CredSSP was requested then the server only supports TLS.";

const SSL_NOT_ALLOWED_BY_SERVER: u32 = 0x00000002;
const SSL_NOT_ALLOWED_BY_SERVER_MSG: &str = "The server is configured to only use Standard RDP Security mechanisms (section 5.3) and does not support any External Security Protocols (section 5.4.5).";

const SSL_CERT_NOT_ON_SERVER: u32 = 0x00000003;
const SSL_CERT_NOT_ON_SERVER_MSG: &str = "The server does not possess a valid authentication certificate and cannot initialize the External Security Protocol Provider (section 5.4.5).";

const INCONSISTENT_FLAGS: u32 = 0x00000004;
const INCONSISTENT_FLAGS_MSG: &str = "The list of requested security protocols is not consistent with the current security protocol in effect. This error is only possible when the Direct Approach (sections 5.4.2.2 and 1.3.1.2) is used and an External Security Protocol (section 5.4.5) is already being used.";

const HYBRID_REQUIRED_BY_SERVER: u32 = 0x00000005;
const HYBRID_REQUIRED_BY_SERVER_MSG: &str = "The server requires that the client support Enhanced RDP Security (section 5.4) with CredSSP (section 5.4.5.2).";

const SSL_WITH_USER_AUTH_REQUIRED_BY_SERVER: u32 = 0x00000006;
const SSL_WITH_USER_AUTH_REQUIRED_BY_SERVER_MSG: &str = "The server requires that the client support Enhanced RDP Security (section 5.4) with TLS 1.0, 1.1 or 1.2 (section 5.4.5.1) and certificate-based client authentication.<4>";

pub struct X224 {
    on_connect: ConnectCb,
    on_fail: FailCb,
}

impl X224 {
    pub fn new(on_connect: ConnectCb, on_fail: FailCb) -> Self {
        Self {
            on_connect,
            on_fail,
        }
    }
}

impl Engine for X224 {
    fn hello(&mut self, rdp: &mut RdpInner) {
        // send X.224 request
        // Client X.224 Connection Request PDU
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/18a27ef9-6f9a-4501-b000-94b1fe3c2c10

        // tpktHeader (4 bytes): A TPKT Header
        // https://www.itu.int/rec/T-REC-T.123-200701-I/en
        rdp.writer.write_u8(RDP_X224_VER); // version number
        rdp.writer.write_u8(0); // reversed
        rdp.writer.write_u8(0); // length (MSB)
        rdp.writer.write_u8(RDP_X224_LEN); // length (LSB)

        // x224Crq (7 bytes):
        // An X.224 Class 0 Connection Request transport protocol data unit (TPDU).
        // https://www.itu.int/rec/T-REC-X.224-199511-I/en section 13.3
        rdp.writer.write_u8(RDP_X224_LEN - TPTK_HDR_LEN - 1); // Length indicator
        rdp.writer.write_u8(0b11100000); // Connection request(4MSB) | Initial credit allocation(4LSB)
        rdp.writer.write_u8(0); // DST-REF
        rdp.writer.write_u8(0); // DST-REF
        rdp.writer.write_u8(0); // SRC-REF
        rdp.writer.write_u8(0); // SRC-REF
        rdp.writer.write_u8(0); // CLASS OPTION

        // rdpNegReq (8 bytes):
        // An optional RDP Negotiation Request (section 2.2.1.1.1) structure.
        // The length of this field is included in the X.224 Connection Request Length Indicator field.
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/902b090b-9cb3-4efc-92bf-ee13373371e3
        rdp.writer.write_u8(RDP_NEG_REQ); // type TYPE_RDP_NEG_REQ
        rdp.writer.write_u8(0); // flags
        rdp.writer.write_u8(8); // length (LSB)
        rdp.writer.write_u8(0); // length (MSB)
        rdp.writer.write_u32_le(PROTOCOL_SSL | PROTOCOL_HYBRID); // requestedProtocols: TLS | CredSSP

        rdp.wait(RDP_X224_LEN as usize);
    }

    fn do_input(&mut self, rdp: &mut RdpInner) {
        // tpktHeader (4 bytes): A TPKT Header
        // https://www.itu.int/rec/T-REC-T.123-200701-I/en
        let _ = rdp.reader.read_u32_be();

        // x224Ccf (7 bytes): An X.224 Class 0 Connection Confirm TPDU
        // https://www.itu.int/rec/T-REC-X.224-199511-I/en section 13.4
        let _ = rdp.reader.read_u32_be();
        let _ = rdp.reader.read_u16_be();
        let _ = rdp.reader.read_u8();

        // rdpNegData (8 bytes):
        // An optional RDP Negotiation Response structure or an optional RDP Negotiation Failure structure.
        // The length of this field is included in the X.224 Connection Confirm Length Indicator field.
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/b2975bdc-6d56-49ee-9c57-f2ff3a0b6817
        // https://docs.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/1b3920e7-0116-4345-bc45-f2c4ad012761
        let type_ = rdp.reader.read_u8();
        let flags = rdp.reader.read_u8();
        let length = rdp.reader.read_u16_le();
        assert!(length == 8);
        let payload = rdp.reader.read_u32_le();

        match type_ {
            RDP_NEG_RSP => {
                // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/b2975bdc-6d56-49ee-9c57-f2ff3a0b6817
                let selected_protocal = payload;
                if selected_protocal & PROTOCOL_HYBRID == 0 {
                    (self.on_fail)(rdp, "Server does not support nla");
                }
                rdp.nla(true);
                (self.on_connect)(rdp);
            }
            RDP_NEG_FAIL => {
                // https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-rdpbcgr/1b3920e7-0116-4345-bc45-f2c4ad012761
                assert!(flags == 0);
                let fail_reason = payload;
                match fail_reason {
                    SSL_REQUIRED_BY_SERVER => (self.on_fail)(rdp, SSL_REQUIRED_BY_SERVER_MSG),
                    SSL_NOT_ALLOWED_BY_SERVER => (self.on_fail)(rdp, SSL_NOT_ALLOWED_BY_SERVER_MSG),
                    SSL_CERT_NOT_ON_SERVER => (self.on_fail)(rdp, SSL_CERT_NOT_ON_SERVER_MSG),
                    INCONSISTENT_FLAGS => (self.on_fail)(rdp, INCONSISTENT_FLAGS_MSG),
                    HYBRID_REQUIRED_BY_SERVER => (self.on_fail)(rdp, HYBRID_REQUIRED_BY_SERVER_MSG),
                    SSL_WITH_USER_AUTH_REQUIRED_BY_SERVER => {
                        (self.on_fail)(rdp, SSL_WITH_USER_AUTH_REQUIRED_BY_SERVER_MSG)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
}
