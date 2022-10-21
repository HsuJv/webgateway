#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use super::rc4::Rc4;
use super::*;
use crate::rdp_client::StreamWriter;
use hmac::{Hmac, Mac};
#[cfg(not(test))]
use js_sys::Math::random;
use md4::{Digest, Md4};
use md5::Md5;
use std::collections::HashMap;
use untrusted::{Input, Reader};

const NTLMSSP_NEGOTIATE_56: u32 = 0x80000000; /* W   (0) */
const NTLMSSP_NEGOTIATE_KEY_EXCH: u32 = 0x40000000; /* V   (1) */
const NTLMSSP_NEGOTIATE_128: u32 = 0x20000000; /* U   (2) */
const NTLMSSP_RESERVED1: u32 = 0x10000000; /* r1  (3) */
const NTLMSSP_RESERVED2: u32 = 0x08000000; /* r2  (4) */
const NTLMSSP_RESERVED3: u32 = 0x04000000; /* r3  (5) */
const NTLMSSP_NEGOTIATE_VERSION: u32 = 0x02000000; /* T   (6) */
const NTLMSSP_RESERVED4: u32 = 0x01000000; /* r4  (7) */
const NTLMSSP_NEGOTIATE_TARGET_INFO: u32 = 0x00800000; /* S   (8) */
const NTLMSSP_REQUEST_NON_NT_SESSION_KEY: u32 = 0x00400000; /* R   (9) */
const NTLMSSP_RESERVED5: u32 = 0x00200000; /* r5  (10) */
const NTLMSSP_NEGOTIATE_IDENTIFY: u32 = 0x00100000; /* Q   (11) */
const NTLMSSP_NEGOTIATE_EXTENDED_SESSION_SECURITY: u32 = 0x00080000; /* P   (12) */
const NTLMSSP_RESERVED6: u32 = 0x00040000; /* r6  (13) */
const NTLMSSP_TARGET_TYPE_SERVER: u32 = 0x00020000; /* O   (14) */
const NTLMSSP_TARGET_TYPE_DOMAIN: u32 = 0x00010000; /* N   (15) */
const NTLMSSP_NEGOTIATE_ALWAYS_SIGN: u32 = 0x00008000; /* M   (16) */
const NTLMSSP_RESERVED7: u32 = 0x00004000; /* r7  (17) */
const NTLMSSP_NEGOTIATE_WORKSTATION_SUPPLIED: u32 = 0x00002000; /* L   (18) */
const NTLMSSP_NEGOTIATE_DOMAIN_SUPPLIED: u32 = 0x00001000; /* K   (19) */
const NTLMSSP_NEGOTIATE_ANONYMOUS: u32 = 0x00000800; /* J   (20) */
const NTLMSSP_RESERVED8: u32 = 0x00000400; /* r8  (21) */
const NTLMSSP_NEGOTIATE_NTLM: u32 = 0x00000200; /* H   (22) */
const NTLMSSP_RESERVED9: u32 = 0x00000100; /* r9  (23) */
const NTLMSSP_NEGOTIATE_LM_KEY: u32 = 0x00000080; /* G   (24) */
const NTLMSSP_NEGOTIATE_DATAGRAM: u32 = 0x00000040; /* F   (25) */
const NTLMSSP_NEGOTIATE_SEAL: u32 = 0x00000020; /* E   (26) */
const NTLMSSP_NEGOTIATE_SIGN: u32 = 0x00000010; /* D   (27) */
const NTLMSSP_RESERVED10: u32 = 0x00000008; /* r10 (28) */
const NTLMSSP_REQUEST_TARGET: u32 = 0x00000004; /* C   (29) */
const NTLMSSP_NEGOTIATE_OEM: u32 = 0x00000002; /* B   (30) */
const NTLMSSP_NEGOTIATE_UNICODE: u32 = 0x00000001; /* A   (31) */

pub const ISC_REQ_DELEGATE: u32 = 0x00000001;
pub const ISC_REQ_MUTUAL_AUTH: u32 = 0x00000002;
pub const ISC_REQ_REPLAY_DETECT: u32 = 0x00000004;
pub const ISC_REQ_SEQUENCE_DETECT: u32 = 0x00000008;
pub const ISC_REQ_CONFIDENTIALITY: u32 = 0x00000010;
pub const ISC_REQ_USE_SESSION_KEY: u32 = 0x00000020;
pub const ISC_REQ_PROMPT_FOR_CREDS: u32 = 0x00000040;
pub const ISC_REQ_USE_SUPPLIED_CREDS: u32 = 0x00000080;
pub const ISC_REQ_ALLOCATE_MEMORY: u32 = 0x00000100;
pub const ISC_REQ_USE_DCE_STYLE: u32 = 0x00000200;
pub const ISC_REQ_DATAGRAM: u32 = 0x00000400;
pub const ISC_REQ_CONNECTION: u32 = 0x00000800;
pub const ISC_REQ_CALL_LEVEL: u32 = 0x00001000;
pub const ISC_REQ_FRAGMENT_SUPPLIED: u32 = 0x00002000;
pub const ISC_REQ_EXTENDED_ERROR: u32 = 0x00004000;
pub const ISC_REQ_STREAM: u32 = 0x00008000;
pub const ISC_REQ_INTEGRITY: u32 = 0x00010000;
pub const ISC_REQ_IDENTIFY: u32 = 0x00020000;
pub const ISC_REQ_NULL_SESSION: u32 = 0x00040000;
pub const ISC_REQ_MANUAL_CRED_VALIDATION: u32 = 0x00080000;
pub const ISC_REQ_RESERVED1: u32 = 0x00100000;
pub const ISC_REQ_FRAGMENT_TO_FIT: u32 = 0x00200000;
pub const ISC_REQ_FORWARD_CREDENTIALS: u32 = 0x00400000;
pub const ISC_REQ_NO_INTEGRITY: u32 = 0x00800000;
pub const ISC_REQ_USE_HTTP_STYLE: u32 = 0x01000000;

pub const ISC_RET_DELEGATE: u32 = 0x00000001;
pub const ISC_RET_MUTUAL_AUTH: u32 = 0x00000002;
pub const ISC_RET_REPLAY_DETECT: u32 = 0x00000004;
pub const ISC_RET_SEQUENCE_DETECT: u32 = 0x00000008;
pub const ISC_RET_CONFIDENTIALITY: u32 = 0x00000010;
pub const ISC_RET_USE_SESSION_KEY: u32 = 0x00000020;
pub const ISC_RET_USED_COLLECTED_CREDS: u32 = 0x00000040;
pub const ISC_RET_USED_SUPPLIED_CREDS: u32 = 0x00000080;
pub const ISC_RET_ALLOCATED_MEMORY: u32 = 0x00000100;
pub const ISC_RET_USED_DCE_STYLE: u32 = 0x00000200;
pub const ISC_RET_DATAGRAM: u32 = 0x00000400;
pub const ISC_RET_CONNECTION: u32 = 0x00000800;
pub const ISC_RET_INTERMEDIATE_RETURN: u32 = 0x00001000;
pub const ISC_RET_CALL_LEVEL: u32 = 0x00002000;
pub const ISC_RET_EXTENDED_ERROR: u32 = 0x00004000;
pub const ISC_RET_STREAM: u32 = 0x00008000;
pub const ISC_RET_INTEGRITY: u32 = 0x00010000;
pub const ISC_RET_IDENTIFY: u32 = 0x00020000;
pub const ISC_RET_NULL_SESSION: u32 = 0x00040000;
pub const ISC_RET_MANUAL_CRED_VALIDATION: u32 = 0x00080000;
pub const ISC_RET_RESERVED1: u32 = 0x00100000;
pub const ISC_RET_FRAGMENT_ONLY: u32 = 0x00200000;
pub const ISC_RET_FORWARD_CREDENTIALS: u32 = 0x00400000;
pub const ISC_RET_USED_HTTP_STYLE: u32 = 0x01000000;

const MsvAvEOL: u16 = 0x0;
const MsvAvNbComputerName: u16 = 0x1;
const MsvAvNbDomainName: u16 = 0x2;
const MsvAvDnsComputerName: u16 = 0x3;
const MsvAvDnsDomainName: u16 = 0x4;
const MsvAvDnsTreeName: u16 = 0x5;
const MsvAvFlags: u16 = 0x6;
const MsvAvTimestamp: u16 = 0x7;
const MsvAvSingleHost: u16 = 0x8;
const MsvAvTargetName: u16 = 0x9;
const MsvChannelBindings: u16 = 0xa;

const MSV_AV_FLAGS_AUTHENTICATION_CONSTRAINED: u32 = 0x00000001;
const MSV_AV_FLAGS_MESSAGE_INTEGRITY_CHECK: u32 = 0x00000002;
const MSV_AV_FLAGS_TARGET_SPN_UNTRUSTED_SOURCE: u32 = 0x00000004;

const NTML_MAGIC: &str = "NTLMSSP\0";

const CLIENT_SIGN_MAGIC: &[u8] = b"session key to client-to-server signing key magic constant\0";
const SERVER_SIGN_MAGIC: &[u8] = b"session key to server-to-client signing key magic constant\0";
const CLIENT_SEAL_MAGIC: &[u8] = b"session key to client-to-server sealing key magic constant\0";
const SERVER_SEAL_MAGIC: &[u8] = b"session key to server-to-client sealing key magic constant\0";

fn read_u8(reader: &mut Reader) -> u8 {
    reader.read_byte().unwrap()
}

fn read_u16(reader: &mut Reader) -> u16 {
    u16::from_le_bytes(
        reader
            .read_bytes(2)
            .unwrap()
            .as_slice_less_safe()
            .try_into()
            .unwrap(),
    )
}

fn read_u32(reader: &mut Reader) -> u32 {
    u32::from_le_bytes(
        reader
            .read_bytes(4)
            .unwrap()
            .as_slice_less_safe()
            .try_into()
            .unwrap(),
    )
}

fn read_utf8(reader: &mut Reader, len: usize) -> String {
    String::from_utf8_lossy(reader.read_bytes(len).unwrap().as_slice_less_safe()).to_string()
}

fn read_utf16(reader: &mut Reader, len: usize) -> String {
    from_unicode(reader.read_bytes(len).unwrap().as_slice_less_safe())
}

fn read_exact_vec(reader: &mut Reader, len: usize) -> Vec<u8> {
    reader
        .read_bytes(len)
        .unwrap()
        .as_slice_less_safe()
        .to_vec()
}

fn hmac_md5(key: &[u8], msg: &[u8]) -> Vec<u8> {
    let mut stream = Hmac::<Md5>::new_from_slice(key).unwrap();
    stream.update(msg);
    stream.finalize().into_bytes().to_vec()
}

fn rc4k(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut result = vec![0; plaintext.len()];
    let mut rc4_handle = Rc4::new(key);
    rc4_handle.process(plaintext, &mut result);
    result
}

struct NtlmMsgObj<'a> {
    len: u16,
    maxlen: u16,
    offset: u32,
    content: &'a [u8],
}

impl<'a> NtlmMsgObj<'a> {
    fn new(reader: &mut Reader, buf: &'a [u8]) -> Self {
        let len = read_u16(reader);
        let maxlen = read_u16(reader);
        let offset = read_u32(reader);
        Self {
            len,
            maxlen,
            offset,
            content: &buf[offset as usize..offset as usize + len as usize],
        }
    }
}

#[derive(Default)]
struct NtlmServerInfo {
    server_name: String,
    server_chal: [u8; 8],
    server_nego: u32,
    server_computer_name: Option<String>,
    server_domain_name: Option<String>,
    server_dns_computer_name: Option<String>,
    server_dns_tree_name: Option<String>,
    server_dns_domain_name: Option<String>,
    server_flags: Option<u32>,
    server_timestamp: Option<Vec<u8>>,
    server_single_host: Option<Vec<u8>>,
    server_target_name: Option<String>,
    server_server_channel_binds: Option<Vec<u8>>,
}

impl NtlmServerInfo {
    fn new_from(
        target_name: &NtlmMsgObj,
        nego_flag: u32,
        server_challenge: &[u8],
        target_info: &NtlmMsgObj,
    ) -> Self {
        let server_name = from_unicode(target_name.content);
        let server_chal = server_challenge;
        let server_nego = nego_flag;
        let mut reader = Reader::new(Input::from(target_info.content));
        let mut new_obj = Self {
            server_name,
            server_chal: server_chal.try_into().unwrap(),
            server_nego,
            ..Default::default()
        };

        loop {
            let av_id = read_u16(&mut reader);
            let av_len = read_u16(&mut reader);

            match av_id {
                MsvAvEOL => break,
                MsvAvNbComputerName => {
                    new_obj.server_computer_name = Some(read_utf16(&mut reader, av_len as usize))
                }
                MsvAvNbDomainName => {
                    new_obj.server_domain_name = Some(read_utf16(&mut reader, av_len as usize))
                }
                MsvAvDnsComputerName => {
                    new_obj.server_dns_computer_name =
                        Some(read_utf16(&mut reader, av_len as usize))
                }
                MsvAvDnsDomainName => {
                    new_obj.server_dns_domain_name = Some(read_utf16(&mut reader, av_len as usize))
                }
                MsvAvDnsTreeName => {
                    new_obj.server_dns_tree_name = Some(read_utf16(&mut reader, av_len as usize))
                }
                MsvAvFlags => new_obj.server_flags = Some(read_u32(&mut reader)),
                MsvAvTimestamp => {
                    new_obj.server_timestamp = Some(read_exact_vec(&mut reader, av_len as usize))
                }
                MsvAvSingleHost => {
                    new_obj.server_single_host = Some(read_exact_vec(&mut reader, av_len as usize))
                }
                MsvAvTargetName => {
                    new_obj.server_target_name = Some(read_utf16(&mut reader, av_len as usize))
                }
                MsvChannelBindings => {
                    new_obj.server_server_channel_binds =
                        Some(read_exact_vec(&mut reader, av_len as usize))
                }
                _ => panic!("Unknown av"),
            }
        }
        new_obj
    }
}

#[derive(Default)]
pub struct Ntlm {
    nego_msg: Vec<u8>,
    chal_msg: Vec<u8>,
    ntlm_v2: bool,
    confidentiality: bool,
    use_mic: bool,
    send_version_info: bool,
    send_single_host_data: bool,
    send_workstation_name: bool,
    suppress_extended_protection: bool,
    username: String,
    password: String,
    domain: String,
    hostname: String,
    server_info: NtlmServerInfo,
    auth_info: HashMap<&'static str, Vec<u8>>,
    encrypt: Option<Rc4>,
    decrypt: Option<Rc4>,
}

impl Ntlm {
    pub fn new(u: &str, p: &str, d: &str, h: &str) -> Self {
        Self {
            ntlm_v2: true,
            send_version_info: true,
            send_workstation_name: true,
            use_mic: true,
            username: u.to_string(),
            password: p.to_string(),
            domain: d.to_string(),
            hostname: h.to_string(),
            ..Default::default()
        }
    }
    pub fn init(&mut self, flag: u32) {
        if flag & ISC_REQ_CONFIDENTIALITY > 0 {
            self.confidentiality = true;
        }
    }

    pub fn generate_nego(&mut self) {
        let nego_msg = Vec::new();
        let mut flags = 0;
        let mut sw = StreamWriter::new(nego_msg);

        /* signature */
        sw.write_string(NTML_MAGIC);
        /* type */
        sw.write_u32_le(1);
        /* flags */
        if self.ntlm_v2 {
            flags |= NTLMSSP_NEGOTIATE_56;
            flags |= NTLMSSP_NEGOTIATE_VERSION;
            flags |= NTLMSSP_NEGOTIATE_LM_KEY;
            flags |= NTLMSSP_NEGOTIATE_OEM;
        }
        flags |= NTLMSSP_NEGOTIATE_KEY_EXCH;
        flags |= NTLMSSP_NEGOTIATE_128;
        flags |= NTLMSSP_NEGOTIATE_EXTENDED_SESSION_SECURITY;
        flags |= NTLMSSP_NEGOTIATE_ALWAYS_SIGN;
        flags |= NTLMSSP_NEGOTIATE_NTLM;
        flags |= NTLMSSP_NEGOTIATE_SIGN;
        flags |= NTLMSSP_REQUEST_TARGET;
        flags |= NTLMSSP_NEGOTIATE_UNICODE;

        if self.confidentiality {
            flags |= NTLMSSP_NEGOTIATE_SEAL;
        }

        if self.send_version_info {
            flags |= NTLMSSP_NEGOTIATE_VERSION;
        }
        sw.write_u32_le(flags);

        /* domain */
        sw.write_u16_le(0);
        sw.write_u16_le(0);
        sw.write_u32_le(40);

        /* workstation */
        sw.write_u16_le(0);
        sw.write_u16_le(0);
        sw.write_u32_le(40);

        /* version */
        sw.write_u8(6); // dwMajorVersion
        sw.write_u8(1); // dwMinorVersion
        sw.write_u16_le(7601); // dwBuildNumber
        sw.write_string("\0\0\0"); // reserved zero
        sw.write_u8(0x0f); // NTLMSSP_REVISION_W2K3

        self.nego_msg = sw.into_inner();
    }

    pub fn generate_client_chal(&mut self, server_chal: &[u8]) {
        let mut reader = Reader::new(Input::from(server_chal));

        let magic = read_utf8(&mut reader, 8);
        if magic != NTML_MAGIC {
            panic!("Unknown magic {:?}", magic);
        }

        let respone = read_u32(&mut reader);
        if respone != 2 {
            panic!("Unknown response type {:?}", respone);
        }

        // reader server info
        let target_name = NtlmMsgObj::new(&mut reader, server_chal);
        let nego_flag = read_u32(&mut reader);
        let server_challenge = reader.read_bytes(8).unwrap().as_slice_less_safe();
        let reserved = reader.read_bytes(8);
        let target_info = NtlmMsgObj::new(&mut reader, server_chal);
        let version = reader.read_bytes(8);
        self.server_info =
            NtlmServerInfo::new_from(&target_name, nego_flag, server_challenge, &target_info);

        // build auth info
        if self.ntlm_v2 {
            let v = self.construct_authenticate_target_info();
            self.auth_info.insert("target_info", v);
        }
        let v = self.server_info.server_timestamp.as_ref().unwrap().to_vec();
        self.auth_info.insert("timestamp", v);
        let mut challenge = [0_u8; 8];
        for i in &mut challenge {
            #[cfg(not(test))]
            {
                let x = ((random() as u32 * 212343) & 0xff) as u8;
                *i = x;
            }
            #[cfg(test)]
            {
                *i = 0;
            }
        }
        let v = challenge.to_vec();
        self.auth_info.insert("challenge", v);

        /* LmChallengeResponse */
        let v = self.compute_lm_v2_response();
        self.auth_info.insert("lmv2", v);

        /* NtChallengeResponse */
        let v = self.compute_ntlm_v2_response();
        self.auth_info.insert("ntlmv2", v);

        /* KeyExchangeKey */
        let v = self.generate_key_exchange_key();
        self.auth_info.insert("exchange_key", v);

        /* RandomSessionKey */
        let v = self.generate_random_session_key();
        self.auth_info.insert("random_session_key", v);

        /* ExportedSessionKey */
        let v = self.generate_exported_session_key();
        self.auth_info.insert("exported_session_key", v);

        /* EncryptedRandomSessionKey */
        let v = self.encrypt_random_session_key();
        self.auth_info.insert("encrypt_session_key", v);

        /* Generate signing keys */
        let v = self.generate_client_signing_key();
        self.auth_info.insert("client_sign_key", v);
        let v = self.generate_server_signing_key();
        self.auth_info.insert("server_sign_key", v);

        /* Generate sealing keys */
        let v = self.generate_client_sealing_key();
        self.auth_info.insert("client_seal_key", v);
        let v = self.generate_server_sealing_key();
        self.auth_info.insert("server_seal_key", v);

        let chal_msg = Vec::new();
        let mut sw = StreamWriter::new(chal_msg);
        let mut nego_flags = 0;

        if self.ntlm_v2 {
            nego_flags |= NTLMSSP_NEGOTIATE_56;
            if self.send_version_info {
                nego_flags |= NTLMSSP_NEGOTIATE_VERSION;
            }
        }

        if self.use_mic {
            nego_flags |= NTLMSSP_NEGOTIATE_TARGET_INFO;
        }

        if self.send_workstation_name {
            nego_flags |= NTLMSSP_NEGOTIATE_WORKSTATION_SUPPLIED;
        }

        if self.confidentiality {
            nego_flags |= NTLMSSP_NEGOTIATE_SEAL;
        }

        if self.server_info.server_nego & NTLMSSP_NEGOTIATE_KEY_EXCH > 0 {
            nego_flags |= NTLMSSP_NEGOTIATE_KEY_EXCH;
        }

        nego_flags |= NTLMSSP_NEGOTIATE_128;
        nego_flags |= NTLMSSP_NEGOTIATE_EXTENDED_SESSION_SECURITY;
        nego_flags |= NTLMSSP_NEGOTIATE_ALWAYS_SIGN;
        nego_flags |= NTLMSSP_NEGOTIATE_NTLM;
        nego_flags |= NTLMSSP_NEGOTIATE_SIGN;
        nego_flags |= NTLMSSP_REQUEST_TARGET;
        nego_flags |= NTLMSSP_NEGOTIATE_UNICODE;

        if !self.domain.is_empty() {
            nego_flags |= NTLMSSP_NEGOTIATE_DOMAIN_SUPPLIED;
        }

        let mut offset = 64;

        if nego_flags & NTLMSSP_NEGOTIATE_VERSION > 0 {
            offset += 8; /* Version (8 bytes) */
        }

        if self.use_mic {
            offset += 16; /* Message Integrity Check (16 bytes) */
        }

        let domain_offset = offset;
        let user_offset = domain_offset + self.domain.len() * 2;
        let hostname_offset = user_offset + self.username.len() * 2;
        let lm_offset = hostname_offset + self.hostname.len() * 2;
        let nt_offset = lm_offset + self.auth_info.get("lmv2").unwrap().len();
        let random_key_offset = nt_offset + self.auth_info.get("ntlmv2").unwrap().len();
        let mut mic_offset = 0;

        /* Message Header (12 bytes) */
        /* signature */
        sw.write_slice(NTML_MAGIC.as_bytes());
        /* type */
        sw.write_u32_le(3);

        /* LmChallengeResponseFields (8 bytes) */
        sw.write_u16_le(self.auth_info.get("lmv2").unwrap().len() as u16);
        sw.write_u16_le(self.auth_info.get("lmv2").unwrap().len() as u16);
        sw.write_u32_le(lm_offset as u32);

        /* NtChallengeResponseFields (8 bytes) */
        sw.write_u16_le(self.auth_info.get("ntlmv2").unwrap().len() as u16);
        sw.write_u16_le(self.auth_info.get("ntlmv2").unwrap().len() as u16);
        sw.write_u32_le(nt_offset as u32);

        /* DomainNameFields (8 bytes) */
        sw.write_u16_le((self.domain.len() * 2) as u16);
        sw.write_u16_le((self.domain.len() * 2) as u16);
        sw.write_u32_le(domain_offset as u32);

        /* UserNameFields (8 bytes) */
        sw.write_u16_le((self.username.len() * 2) as u16);
        sw.write_u16_le((self.username.len() * 2) as u16);
        sw.write_u32_le(user_offset as u32);

        /* WorkstationFields (8 bytes) */
        sw.write_u16_le((self.hostname.len() * 2) as u16);
        sw.write_u16_le((self.hostname.len() * 2) as u16);
        sw.write_u32_le(hostname_offset as u32);

        /* EncryptedRandomSessionKeyFields (8 bytes) */
        sw.write_u16_le(self.auth_info.get("encrypt_session_key").unwrap().len() as u16);
        sw.write_u16_le(self.auth_info.get("encrypt_session_key").unwrap().len() as u16);
        sw.write_u32_le(random_key_offset as u32);

        /* NegotiateFlags (4 bytes) */
        sw.write_u32_le(nego_flags);

        /* Version (8 bytes) */
        if nego_flags & NTLMSSP_NEGOTIATE_VERSION > 0 {
            sw.write_u8(6); // dwMajorVersion
            sw.write_u8(1); // dwMinorVersion
            sw.write_u16_le(7601); // dwBuildNumber
            sw.write_slice(&[0; 3]); // reserved zero
            sw.write_u8(0x0f); // NTLMSSP_REVISION_W2K3
        }

        /* Message Integrity Check (16 bytes) */
        if self.use_mic {
            mic_offset = sw.get_inner().len();
            sw.write_slice(&[0; 16]); // reserved zero
        }

        /* DomainName */
        if nego_flags & NTLMSSP_NEGOTIATE_DOMAIN_SUPPLIED > 0 {
            sw.write_slice(&to_unicode(&self.domain));
        }

        /* UserName */
        sw.write_slice(&to_unicode(&self.username));

        /* Workstation */
        if nego_flags & NTLMSSP_NEGOTIATE_WORKSTATION_SUPPLIED > 0 {
            sw.write_slice(&to_unicode(&self.hostname));
        }

        /* LmChallengeResponse */
        sw.write_slice(self.auth_info.get("lmv2").unwrap());

        /* NtChallengeResponse */
        sw.write_slice(self.auth_info.get("ntlmv2").unwrap());

        if nego_flags & NTLMSSP_NEGOTIATE_KEY_EXCH > 0 {
            /* EncryptedRandomSessionKey */
            sw.write_slice(self.auth_info.get("encrypt_session_key").unwrap());
        }

        self.chal_msg = sw.into_inner();
        if self.use_mic {
            /* Message Integrity Check */
            let mic = hmac_md5(
                self.auth_info.get("exported_session_key").unwrap(),
                &[&self.nego_msg[..], server_chal, &self.chal_msg[..]].concat(),
            );
            for (i, m) in mic.into_iter().enumerate() {
                self.chal_msg[i + mic_offset] = m;
            }
        }
    }

    pub fn generate_client_auth(&mut self) {
        unimplemented!()
    }

    pub fn encrypt(&mut self, data: &[u8], seq: u32) -> Vec<u8> {
        let digest = hmac_md5(
            self.auth_info.get("client_sign_key").unwrap(),
            &[&seq.to_le_bytes(), data].concat(),
        );

        let to_be_encrypted = if self.confidentiality {
            if self.encrypt.is_none() {
                self.encrypt = Some(Rc4::new(self.auth_info.get("client_seal_key").unwrap()))
            }
            let mut encrypted = vec![0; data.len()];
            self.encrypt.as_mut().unwrap().process(data, &mut encrypted);
            encrypted
        } else {
            // copy as is
            data.to_vec()
        };

        // /* RC4-encrypt first 8 unsigned chars of digest */
        let mut chksum = vec![0; 8];
        self.encrypt
            .as_mut()
            .unwrap()
            .process(&digest[..8], &mut chksum);
        [
            &1_u32.to_le_bytes(),
            &chksum[..],
            &seq.to_le_bytes(),
            &to_be_encrypted,
        ]
        .concat()
    }

    pub fn get_nego_msg(&self) -> &[u8] {
        &self.nego_msg
    }

    pub fn get_chal_msg(&self) -> &[u8] {
        &self.chal_msg
    }

    fn construct_authenticate_target_info(&mut self) -> Vec<u8> {
        let nb_domain_name = self.server_info.server_domain_name.as_ref();
        let nb_computer_name = self.server_info.server_computer_name.as_ref();
        let dns_domain_name = self.server_info.server_dns_domain_name.as_ref();
        let dns_computer_name = self.server_info.server_dns_computer_name.as_ref();
        let dns_tree_name = self.server_info.server_dns_tree_name.as_ref();
        let timestamp = self.server_info.server_timestamp.as_ref();
        let ret = Vec::new();
        let mut sw = StreamWriter::new(ret);

        if let Some(nb_domain_name) = nb_domain_name {
            sw.write_u16_le(MsvAvNbDomainName);
            sw.write_u16_le(nb_domain_name.len() as u16 * 2);
            sw.write_slice(&to_unicode(nb_domain_name));
        }

        if let Some(nb_computer_name) = nb_computer_name {
            sw.write_u16_le(MsvAvNbComputerName);
            sw.write_u16_le(nb_computer_name.len() as u16 * 2);
            sw.write_slice(&to_unicode(nb_computer_name));
        }

        if let Some(dns_domain_name) = dns_domain_name {
            sw.write_u16_le(MsvAvDnsDomainName);
            sw.write_u16_le(dns_domain_name.len() as u16 * 2);
            sw.write_slice(&to_unicode(dns_domain_name));
        }

        if let Some(dns_computer_name) = dns_computer_name {
            sw.write_u16_le(MsvAvDnsComputerName);
            sw.write_u16_le(dns_computer_name.len() as u16 * 2);
            sw.write_slice(&to_unicode(dns_computer_name));
        }

        if let Some(dns_tree_name) = dns_tree_name {
            sw.write_u16_le(MsvAvDnsTreeName);
            sw.write_u16_le(dns_tree_name.len() as u16 * 2);
            sw.write_slice(&to_unicode(dns_tree_name));
        }

        if let Some(timestamp) = timestamp {
            sw.write_u16_le(MsvAvTimestamp);
            sw.write_u16_le(8);
            sw.write_slice(timestamp);
        }

        if self.use_mic {
            sw.write_u16_le(MsvAvFlags);
            sw.write_u16_le(4);
            sw.write_u32_le(MSV_AV_FLAGS_MESSAGE_INTEGRITY_CHECK);
        }

        /*
         * Extended Protection for Authentication:
         * http://blogs.technet.com/b/srd/archive/2009/12/08/extended-protection-for-authentication.aspx
         */
        if !self.suppress_extended_protection {
            /*
             * SEC_CHANNEL_BINDINGS structure
             * http://msdn.microsoft.com/en-us/library/windows/desktop/dd919963/
             */
            sw.write_u16_le(MsvChannelBindings);
            sw.write_u16_le(16);
            sw.write_slice(&[0; 16]);

            if !self.hostname.is_empty() {
                sw.write_u16_le(MsvAvTargetName);
                sw.write_u16_le(self.hostname.len() as u16 * 2);
                sw.write_slice(&to_unicode(&self.hostname));
            }
        }

        if self.ntlm_v2 {
            sw.write_u16_le(MsvAvEOL);
            sw.write_u16_le(0);
        }
        sw.into_inner()
    }

    fn compute_ntlm_v2_hash(&mut self) -> Vec<u8> {
        if self.password.len() > 255 {
            panic!("Password too long");
        } else {
            let mut md4_ctx = Md4::new();
            md4_ctx.update(to_unicode(&self.password));
            let passwd_hash = md4_ctx.finalize();

            hmac_md5(
                &passwd_hash,
                &to_unicode(&(self.username.to_uppercase() + &self.domain)),
            )
        }
    }

    fn compute_lm_v2_response(&mut self) -> Vec<u8> {
        /* Compute the NTLMv2 hash */
        let ntlm_v2_hash = self.compute_ntlm_v2_hash();

        /* Concatenate the server and client ServerChallengechallenges */
        let mut msg = hmac_md5(
            &ntlm_v2_hash,
            &[
                &self.server_info.server_chal[..],
                &self.auth_info.get("challenge").unwrap()[..],
            ]
            .concat(),
        );

        /* Concatenate the resulting HMAC-MD5 hash and the client challenge, giving
         * us the LMv2 response (24 bytes) */
        msg.extend_from_slice(self.auth_info.get("challenge").unwrap());
        msg
    }

    fn compute_ntlm_v2_response(&mut self) -> Vec<u8> {
        let blob_c = Vec::new();
        let mut sw_c = StreamWriter::new(blob_c);
        let blob_s = Vec::new();
        let mut sw_s = StreamWriter::new(blob_s);
        let nt_proof = Vec::new();
        let mut sw_nt = StreamWriter::new(nt_proof);
        let ntlm_v2_hash = self.compute_ntlm_v2_hash();

        /* Construct */
        sw_c.write_u8(1); /* RespType (1 byte) */
        sw_c.write_u8(1); /* HighRespType (1 byte) */
        sw_c.write_slice(&[0; 2]); /* Reserved1 (2 bytes) */
        sw_c.write_slice(&[0; 4]); /* Reserved2 (4 bytes) */
        sw_c.write_slice(&self.auth_info.get("timestamp").unwrap()[..8]); /* Timestamp (8 bytes) */
        sw_c.write_slice(self.auth_info.get("challenge").unwrap()); /* ClientChallenge (8 bytes) */
        sw_c.write_slice(&[0; 4]); /* Reserved3 (4 bytes) */
        sw_c.write_slice(self.auth_info.get("target_info").unwrap());

        /* Concatenate server challenge with temp */
        sw_s.write_slice(&self.server_info.server_chal);
        sw_s.write_slice(sw_c.get_inner());

        let nt_proof_msg = hmac_md5(&ntlm_v2_hash, sw_s.get_inner());

        /* NtChallengeResponse, Concatenate NTProofStr with temp */
        sw_nt.write_slice(&nt_proof_msg);
        sw_nt.write_slice(sw_c.get_inner());

        /* Compute SessionBaseKey, the HMAC-MD5 hash of NTProofStr using the NTLMv2
         * hash as the key */
        self.auth_info
            .insert("session_base_key", hmac_md5(&ntlm_v2_hash, &nt_proof_msg));

        sw_nt.into_inner()
    }

    fn generate_key_exchange_key(&mut self) -> Vec<u8> {
        self.auth_info.get("session_base_key").unwrap().clone()
    }

    fn generate_random_session_key(&mut self) -> Vec<u8> {
        let mut rand = [0_u8; 16];
        for i in &mut rand {
            #[cfg(not(test))]
            {
                let x = ((random() as u32 * 24635) & 0xff) as u8;
                *i = x;
            }
            #[cfg(test)]
            {
                *i = 0;
            }
        }
        rand.to_vec()
    }

    fn generate_exported_session_key(&mut self) -> Vec<u8> {
        self.auth_info.get("random_session_key").unwrap().clone()
    }

    fn encrypt_random_session_key(&mut self) -> Vec<u8> {
        rc4k(
            self.auth_info.get("exchange_key").unwrap(),
            self.auth_info.get("exported_session_key").unwrap(),
        )
    }

    /* Generate signing keys */
    fn generate_signing_key(&mut self, magic: &[u8]) -> Vec<u8> {
        /* Concatenate ExportedSessionKey with sign magic */
        let mut hasher = Md5::new();
        hasher.update(
            [
                &self.auth_info.get("exported_session_key").unwrap()[..],
                magic,
            ]
            .concat(),
        );
        hasher.finalize().to_vec()
    }

    fn generate_client_signing_key(&mut self) -> Vec<u8> {
        self.generate_signing_key(CLIENT_SIGN_MAGIC)
    }

    fn generate_server_signing_key(&mut self) -> Vec<u8> {
        self.generate_signing_key(SERVER_SIGN_MAGIC)
    }

    fn generate_client_sealing_key(&mut self) -> Vec<u8> {
        self.generate_signing_key(CLIENT_SEAL_MAGIC)
    }

    fn generate_server_sealing_key(&mut self) -> Vec<u8> {
        self.generate_signing_key(SERVER_SEAL_MAGIC)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_ntlm() {
        let mut ntlm = Ntlm::new("sonicwall", "sonicwall", "", "SRA-HTML5-RDP");
        ntlm.init(ISC_REQ_MUTUAL_AUTH | ISC_REQ_CONFIDENTIALITY | ISC_REQ_USE_SESSION_KEY);
        ntlm.generate_nego();
        ntlm.generate_client_chal(&[
            0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x02, 0x00, 0x00, 0x00, 0x1e, 0x00,
            0x1e, 0x00, 0x38, 0x00, 0x00, 0x00, 0x35, 0x82, 0x8a, 0xe2, 0xde, 0xf9, 0x8e, 0xbc,
            0x63, 0xf8, 0xa3, 0x51, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x98, 0x00,
            0x98, 0x00, 0x56, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x61, 0x4a, 0x00, 0x00, 0x00, 0x0f,
            0x44, 0x00, 0x45, 0x00, 0x53, 0x00, 0x4b, 0x00, 0x54, 0x00, 0x4f, 0x00, 0x50, 0x00,
            0x2d, 0x00, 0x4d, 0x00, 0x55, 0x00, 0x48, 0x00, 0x4d, 0x00, 0x42, 0x00, 0x51, 0x00,
            0x31, 0x00, 0x02, 0x00, 0x1e, 0x00, 0x44, 0x00, 0x45, 0x00, 0x53, 0x00, 0x4b, 0x00,
            0x54, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00, 0x4d, 0x00, 0x55, 0x00, 0x48, 0x00,
            0x4d, 0x00, 0x42, 0x00, 0x51, 0x00, 0x31, 0x00, 0x01, 0x00, 0x1e, 0x00, 0x44, 0x00,
            0x45, 0x00, 0x53, 0x00, 0x4b, 0x00, 0x54, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00,
            0x4d, 0x00, 0x55, 0x00, 0x48, 0x00, 0x4d, 0x00, 0x42, 0x00, 0x51, 0x00, 0x31, 0x00,
            0x04, 0x00, 0x1e, 0x00, 0x44, 0x00, 0x45, 0x00, 0x53, 0x00, 0x4b, 0x00, 0x54, 0x00,
            0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00, 0x4d, 0x00, 0x55, 0x00, 0x48, 0x00, 0x4d, 0x00,
            0x42, 0x00, 0x51, 0x00, 0x31, 0x00, 0x03, 0x00, 0x1e, 0x00, 0x44, 0x00, 0x45, 0x00,
            0x53, 0x00, 0x4b, 0x00, 0x54, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00, 0x4d, 0x00,
            0x55, 0x00, 0x48, 0x00, 0x4d, 0x00, 0x42, 0x00, 0x51, 0x00, 0x31, 0x00, 0x07, 0x00,
            0x08, 0x00, 0xd4, 0x0f, 0xe9, 0x0e, 0x3d, 0xe4, 0xd8, 0x01, 0x00, 0x00, 0x00, 0x00,
        ]);

        let out_put = ntlm.get_chal_msg();

        assert_eq!(
            out_put,
            &[
                0x4e, 0x54, 0x4c, 0x4d, 0x53, 0x53, 0x50, 0x00, 0x03, 0x00, 0x00, 0x00, 0x18, 0x00,
                0x18, 0x00, 0x84, 0x00, 0x00, 0x00, 0xfe, 0x00, 0xfe, 0x00, 0x9c, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x58, 0x00, 0x00, 0x00, 0x12, 0x00, 0x12, 0x00, 0x58, 0x00,
                0x00, 0x00, 0x1a, 0x00, 0x1a, 0x00, 0x6a, 0x00, 0x00, 0x00, 0x10, 0x00, 0x10, 0x00,
                0x9a, 0x01, 0x00, 0x00, 0x35, 0xa2, 0x88, 0xe2, 0x06, 0x01, 0xb1, 0x1d, 0x00, 0x00,
                0x00, 0x0f, 0x41, 0x43, 0x6d, 0x1b, 0x0a, 0x90, 0x26, 0xf2, 0x19, 0x8b, 0xca, 0x5c,
                0xb7, 0xf5, 0x91, 0x3d, 0x73, 0x00, 0x6f, 0x00, 0x6e, 0x00, 0x69, 0x00, 0x63, 0x00,
                0x77, 0x00, 0x61, 0x00, 0x6c, 0x00, 0x6c, 0x00, 0x53, 0x00, 0x52, 0x00, 0x41, 0x00,
                0x2d, 0x00, 0x48, 0x00, 0x54, 0x00, 0x4d, 0x00, 0x4c, 0x00, 0x35, 0x00, 0x2d, 0x00,
                0x52, 0x00, 0x44, 0x00, 0x50, 0x00, 0x78, 0xb6, 0x42, 0xb8, 0xe1, 0x90, 0x21, 0xdf,
                0x21, 0xb6, 0x68, 0x79, 0xc9, 0x3c, 0x2b, 0x6d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x17, 0xa8, 0x35, 0x50, 0x59, 0x07, 0x77, 0x2e, 0x1b, 0x09, 0xf6, 0x62,
                0xea, 0x97, 0x13, 0xce, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0x0f,
                0xe9, 0x0e, 0x3d, 0xe4, 0xd8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x1e, 0x00, 0x44, 0x00, 0x45, 0x00, 0x53, 0x00,
                0x4b, 0x00, 0x54, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00, 0x4d, 0x00, 0x55, 0x00,
                0x48, 0x00, 0x4d, 0x00, 0x42, 0x00, 0x51, 0x00, 0x31, 0x00, 0x01, 0x00, 0x1e, 0x00,
                0x44, 0x00, 0x45, 0x00, 0x53, 0x00, 0x4b, 0x00, 0x54, 0x00, 0x4f, 0x00, 0x50, 0x00,
                0x2d, 0x00, 0x4d, 0x00, 0x55, 0x00, 0x48, 0x00, 0x4d, 0x00, 0x42, 0x00, 0x51, 0x00,
                0x31, 0x00, 0x04, 0x00, 0x1e, 0x00, 0x44, 0x00, 0x45, 0x00, 0x53, 0x00, 0x4b, 0x00,
                0x54, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00, 0x4d, 0x00, 0x55, 0x00, 0x48, 0x00,
                0x4d, 0x00, 0x42, 0x00, 0x51, 0x00, 0x31, 0x00, 0x03, 0x00, 0x1e, 0x00, 0x44, 0x00,
                0x45, 0x00, 0x53, 0x00, 0x4b, 0x00, 0x54, 0x00, 0x4f, 0x00, 0x50, 0x00, 0x2d, 0x00,
                0x4d, 0x00, 0x55, 0x00, 0x48, 0x00, 0x4d, 0x00, 0x42, 0x00, 0x51, 0x00, 0x31, 0x00,
                0x07, 0x00, 0x08, 0x00, 0xd4, 0x0f, 0xe9, 0x0e, 0x3d, 0xe4, 0xd8, 0x01, 0x06, 0x00,
                0x04, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x00,
                0x1a, 0x00, 0x53, 0x00, 0x52, 0x00, 0x41, 0x00, 0x2d, 0x00, 0x48, 0x00, 0x54, 0x00,
                0x4d, 0x00, 0x4c, 0x00, 0x35, 0x00, 0x2d, 0x00, 0x52, 0x00, 0x44, 0x00, 0x50, 0x00,
                0x00, 0x00, 0x00, 0x00, 0xf5, 0x7a, 0xcd, 0x84, 0x54, 0x55, 0x29, 0xfc, 0x8c, 0x0b,
                0x1d, 0x45, 0xb0, 0xbf, 0xba, 0x0a
            ]
        );

        let encrypt_key = ntlm.encrypt(
            [
                0x30, 0x82, 0x01, 0x0a, 0x02, 0x82, 0x01, 0x01, 0x00, 0xd0, 0xe1, 0x5f, 0xf5, 0x6d,
                0xbf, 0xa2, 0xbe, 0x52, 0x21, 0x13, 0x99, 0xae, 0x0b, 0x56, 0x08, 0x84, 0x46, 0x41,
                0xb1, 0x6a, 0x81, 0xbb, 0xe8, 0xff, 0x6e, 0xea, 0xa4, 0xc5, 0x57, 0x58, 0x85, 0xf7,
                0x12, 0xef, 0xaa, 0x3b, 0x57, 0x24, 0xf5, 0x2d, 0x59, 0xaa, 0xd6, 0xd7, 0x6b, 0xcd,
                0x62, 0x9f, 0x29, 0x7a, 0x65, 0x98, 0x42, 0xf0, 0x3e, 0xd8, 0x13, 0x45, 0x1d, 0xe0,
                0xd8, 0x19, 0x8c, 0x57, 0xd9, 0x91, 0xb8, 0xda, 0xed, 0x2e, 0x83, 0xf0, 0x34, 0x21,
                0xae, 0x36, 0x92, 0x0b, 0x3b, 0xa8, 0x01, 0x7a, 0xdd, 0x60, 0xd2, 0x17, 0x57, 0x2b,
                0x5e, 0xac, 0xf0, 0x5c, 0x3d, 0x73, 0x2a, 0x1e, 0xdd, 0x7c, 0xbc, 0x70, 0xeb, 0xdd,
                0x63, 0x58, 0x00, 0x16, 0x36, 0xf3, 0x0b, 0x48, 0x40, 0x79, 0xce, 0x6f, 0x52, 0xee,
                0x42, 0xfa, 0x0f, 0xed, 0xd0, 0xf4, 0x50, 0x73, 0xa6, 0x88, 0xce, 0x6e, 0x1a, 0x3b,
                0x69, 0x73, 0x86, 0x1d, 0x89, 0x21, 0x35, 0x97, 0x1e, 0x94, 0xab, 0xbe, 0xc4, 0x2b,
                0x4b, 0x42, 0x5e, 0x25, 0x26, 0xe5, 0x0e, 0x4e, 0x31, 0xfc, 0x7f, 0xf6, 0xfe, 0xda,
                0x44, 0x27, 0xe3, 0xde, 0xfa, 0xf1, 0xdd, 0x58, 0x66, 0x4a, 0x35, 0xf8, 0x03, 0x34,
                0x2b, 0x7a, 0xa9, 0x42, 0xfa, 0x46, 0xb2, 0xbd, 0xfb, 0x4c, 0x78, 0x66, 0xd9, 0xd1,
                0xac, 0x47, 0xf3, 0x02, 0xff, 0x44, 0xa7, 0x87, 0x26, 0x0c, 0xd3, 0xe6, 0x2c, 0xeb,
                0x4c, 0x4b, 0x51, 0x3f, 0xc6, 0x25, 0x8c, 0x22, 0x4a, 0xd2, 0xaa, 0x86, 0x73, 0xc4,
                0x90, 0x2d, 0xd3, 0xe3, 0x7a, 0xa8, 0x2b, 0x37, 0xb1, 0x5e, 0x0e, 0x31, 0x0b, 0x27,
                0x14, 0x0a, 0x8d, 0x1f, 0xed, 0xde, 0xb3, 0x19, 0xa8, 0x08, 0x63, 0x3d, 0xaf, 0x52,
                0xff, 0x38, 0xef, 0x54, 0x0d, 0xb9, 0x7e, 0xc9, 0x6f, 0x07, 0x30, 0x67, 0xe9, 0x02,
                0x03, 0x01, 0x00, 0x01,
            ]
            .as_ref(),
            0,
        );

        assert_eq!(
            &encrypt_key,
            &[
                0x01, 0x00, 0x00, 0x00, 0xa4, 0xe3, 0x81, 0x3f, 0xff, 0x39, 0x8d, 0xb4, 0x00, 0x00,
                0x00, 0x00, 0x21, 0x95, 0xdd, 0xca, 0x85, 0x92, 0xd4, 0x13, 0x20, 0x66, 0x44, 0xf8,
                0x52, 0x39, 0x53, 0x29, 0xea, 0x75, 0x57, 0x90, 0x6e, 0x54, 0x33, 0x2b, 0x09, 0xd7,
                0x52, 0x18, 0xf8, 0xde, 0x2b, 0x85, 0x13, 0x4f, 0x06, 0x57, 0x17, 0x6b, 0x89, 0x35,
                0xe0, 0x9d, 0x1c, 0x41, 0xad, 0xf1, 0xaf, 0xd2, 0x9c, 0xe7, 0x67, 0x3e, 0xe5, 0x2a,
                0xd0, 0xa3, 0x9e, 0x53, 0x12, 0xca, 0xc3, 0x74, 0x19, 0x58, 0x7f, 0x55, 0x5f, 0x71,
                0x2d, 0x5f, 0x88, 0x1e, 0x63, 0xc1, 0x98, 0xba, 0xfd, 0x2f, 0x42, 0x15, 0xe8, 0xf6,
                0xaf, 0xc3, 0x48, 0xa8, 0x4f, 0x8c, 0xab, 0x8a, 0x0f, 0xef, 0x12, 0xa6, 0x53, 0x5d,
                0x01, 0x80, 0xf6, 0xc7, 0x61, 0x71, 0x39, 0xb4, 0x09, 0xbb, 0x5d, 0xc9, 0x24, 0x07,
                0x66, 0x73, 0xa9, 0x03, 0x8d, 0x68, 0x8f, 0xf4, 0x78, 0xe5, 0x71, 0x5a, 0x69, 0x7f,
                0x06, 0xae, 0x76, 0x92, 0x6e, 0x58, 0xc7, 0xdc, 0xeb, 0x8a, 0x8a, 0x84, 0xa4, 0x22,
                0x54, 0x58, 0x05, 0xfd, 0x4c, 0xf0, 0xb8, 0x68, 0x41, 0x52, 0x64, 0xc6, 0xc7, 0x41,
                0x54, 0x0f, 0xb8, 0xa0, 0x36, 0xe2, 0x86, 0xa9, 0x8b, 0x98, 0x50, 0x6d, 0x69, 0xad,
                0xd2, 0xdc, 0x0b, 0x1a, 0xff, 0x9e, 0xca, 0xd7, 0x74, 0x39, 0x7a, 0xc0, 0xa8, 0x30,
                0x9b, 0xed, 0x52, 0x5d, 0xc1, 0xde, 0x53, 0x87, 0x1d, 0x13, 0xd4, 0xc0, 0x54, 0xa9,
                0x0a, 0x11, 0xbe, 0xc1, 0x74, 0x8e, 0x4a, 0x99, 0xdf, 0xe7, 0x33, 0x8a, 0xd9, 0x2b,
                0x70, 0x10, 0x9e, 0xa4, 0xd9, 0x42, 0x99, 0xfe, 0x6c, 0x49, 0x6c, 0xf0, 0x37, 0x99,
                0xfc, 0x4f, 0x51, 0x35, 0x56, 0x47, 0x7e, 0xa7, 0x17, 0x25, 0xee, 0x6e, 0x35, 0xea,
                0x9f, 0x5f, 0xf3, 0xd7, 0xd0, 0x31, 0xd2, 0x79, 0x68, 0xbc, 0x4f, 0xd7, 0x4e, 0xae,
                0x28, 0x24, 0xb8, 0x2c, 0xe2, 0x6e, 0xb6, 0x3f, 0xee, 0x8f, 0xa9, 0x33, 0x57, 0xf0,
                0x39, 0x65, 0x4f, 0xfe, 0xf7, 0x40
            ]
        )
    }
}
