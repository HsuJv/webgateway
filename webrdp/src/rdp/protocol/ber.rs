#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::rdp::StreamWriter;

/* Class - bits 8 and 7 */
const BER_CLASS_UNIV: u8 = 0x00;
const BER_CLASS_MASK: u8 = 0xC0;
const BER_CLASS_APPL: u8 = 0x40;
const BER_CLASS_CTXT: u8 = 0x80;
const BER_CLASS_PRIV: u8 = 0xc0;
/* P/C - bit 6 */
const BER_PC_MASK: u8 = 0x20;
const BER_PRIMITIVE: u8 = 0x00; /* 0 */
const BER_CONSTRUCT: u8 = 0x20; /* 1 */
/* Tag - bits 5 to 1 */
const BER_TAG_MASK: u8 = 0x1F;
const BER_TAG_BOOLEAN: u8 = 0x01;
const BER_TAG_INTEGER: u8 = 0x02;
const BER_TAG_BIT_STRING: u8 = 0x03;
const BER_TAG_OCTET_STRING: u8 = 0x04;
const BER_TAG_OBJECT_IDENFIER: u8 = 0x06;
const BER_TAG_ENUMERATED: u8 = 0x0A;
const BER_TAG_SEQUENCE: u8 = 0x10;
const BER_TAG_SEQUENCE_OF: u8 = 0x10;

/// Enum all possible value
/// In an ASN 1 tree
#[derive(Debug)]
pub enum ASN1Type<'a> {
    /// A list of ASN1 node equivalent to component
    // Sequence(Cow<'a, &'a [BerObj<'a>]>),
    Sequence(&'a [BerObj<'a>]),
    SequenceOwned(Vec<BerObj<'a>>),
    /// Unsigned 32 bits type
    U32(u32),
    /// Octet string
    OctetString(&'a [u8]),
    /// Boolean
    Bool(bool),
    /// Enumerate
    Enumerate(i64),
    // Private Type
    Priv(&'a BerObj<'a>),
    PrivOwned(Box<BerObj<'a>>),
}

impl<'a> From<u32> for ASN1Type<'a> {
    fn from(v: u32) -> Self {
        Self::U32(v)
    }
}

impl<'a> From<&'a [u8]> for ASN1Type<'a> {
    fn from(b: &'a [u8]) -> Self {
        Self::OctetString(b)
    }
}

impl<'a> From<&'a [BerObj<'a>]> for ASN1Type<'a> {
    fn from(seq: &'a [BerObj<'a>]) -> Self {
        Self::Sequence(seq)
    }
}

impl<'a> From<Vec<BerObj<'a>>> for ASN1Type<'a> {
    fn from(seq: Vec<BerObj<'a>>) -> Self {
        Self::SequenceOwned(seq)
    }
}

impl<'a> From<BerObj<'a>> for ASN1Type<'a> {
    fn from(ber: BerObj<'a>) -> Self {
        Self::PrivOwned(Box::new(ber))
    }
}

fn BER_PC(pc: bool) -> u8 {
    if pc {
        BER_CONSTRUCT
    } else {
        BER_PRIMITIVE
    }
}

#[derive(Debug)]
pub struct BerObj<'a> {
    value: ASN1Type<'a>,
    tag: u8,
}

// fn make_outlive<'a>(anchor: &'a [u8], value: Vec<BerObj<'a>>) -> &'a [BerObj<'a>] {
//     value.as_slice()
// }

impl<'a> BerObj<'a> {
    pub fn new_with_tag(tag: u8, value: &'a BerObj<'a>) -> Self {
        Self {
            value: ASN1Type::Priv(value),
            tag: (BER_CLASS_CTXT | BER_PC(true)) | (BER_TAG_MASK & tag),
        }
    }

    pub fn new(value: ASN1Type<'a>) -> Self {
        fn universal_tag(ty: u8, pc: bool) -> u8 {
            (BER_CLASS_UNIV | BER_PC(pc)) | (BER_TAG_MASK & ty)
        }
        let tag = match value {
            ASN1Type::U32(_) => universal_tag(BER_TAG_INTEGER, false),
            ASN1Type::Sequence(_) => universal_tag(BER_TAG_SEQUENCE, true),
            ASN1Type::SequenceOwned(_) => universal_tag(BER_TAG_SEQUENCE, true),
            ASN1Type::OctetString(_) => universal_tag(BER_TAG_OCTET_STRING, false),
            _ => unreachable!(),
        };
        Self { value, tag }
    }

    pub fn from_der(der: &'a [u8]) -> Self {
        let mut cursor = 0;
        let tag = der[cursor];
        let pc = tag & BER_CONSTRUCT;
        let ctx = tag & BER_CLASS_CTXT;
        let tag_masked = tag & BER_TAG_MASK;
        cursor += 1;

        let mut len = der[cursor] as u16;
        cursor += 1;
        if len == 0x82 {
            let mut blen = [0; 2];
            blen[0] = der[cursor];
            cursor += 1;
            blen[1] = der[cursor];
            cursor += 1;
            len = u16::from_be_bytes(blen);
        } else if len == 0x81 {
            len = der[cursor] as u16;
            cursor += 1;
        }

        match (ctx, pc, tag_masked) {
            (BER_CLASS_UNIV, BER_CONSTRUCT, BER_TAG_SEQUENCE) => {
                let mut seq = Vec::new();
                while cursor < der.len() {
                    let subobj = BerObj::from_der(&der[cursor..]);
                    let sublen = subobj.total_length();
                    seq.push(subobj);
                    cursor += sublen as usize;
                }
                BerObj {
                    tag,
                    value: seq.into(),
                }
            }
            (BER_CLASS_UNIV, BER_PRIMITIVE, BER_TAG_INTEGER) => {
                let mut value = 0;
                for i in 0..len {
                    value <<= 8;
                    value |= der[cursor] as u32;
                    cursor += 1;
                }

                BerObj {
                    tag,
                    value: value.into(),
                }
            }
            (BER_CLASS_UNIV, BER_PRIMITIVE, BER_TAG_OCTET_STRING) => BerObj {
                tag,
                value: (&der[cursor..cursor + len as usize]).into(),
            },
            (BER_CLASS_CTXT, BER_CONSTRUCT, _) => BerObj {
                tag,
                value: BerObj::from_der(&der[cursor..]).into(),
            },
            // (BER_CLASS_CTXT, BER_CONSTRUCT, tag) => {}
            (x, y, z) => unreachable!("ctx: {}, pc {}, tag {}", x, y, z),
        }
    }

    pub fn to_der(&self) -> Vec<u8> {
        let len = self.value_len();
        let out = Vec::new();
        let mut sw = StreamWriter::new(out);

        // tag
        sw.write_u8(self.tag);

        // length
        if len > 0xff {
            sw.write_u8(0x80 ^ 2);
            sw.write_u16_be(len);
        } else if len > 0x7f {
            sw.write_u8(0x80 ^ 1);
            sw.write_u8(len.try_into().unwrap());
        } else {
            sw.write_u8(len.try_into().unwrap());
        }

        // value
        match self.value {
            ASN1Type::U32(x) => {
                if x < 0x80 {
                    sw.write_u8(x.try_into().unwrap());
                } else if x < 0x8000 {
                    sw.write_u16_be(x.try_into().unwrap());
                } else if x < 0x800000 {
                    sw.write_u8((x >> 16).try_into().unwrap());
                    sw.write_u16_be((x & 0xffff).try_into().unwrap())
                } else if x < 0x80000000 {
                    sw.write_u32_be(x);
                } else {
                    unreachable!()
                }
            }
            ASN1Type::OctetString(s) => {
                sw.write_slice(s);
            }
            ASN1Type::Priv(p) => sw.write_slice(&p.to_der()),
            ASN1Type::PrivOwned(ref p) => sw.write_slice(&p.to_der()),
            ASN1Type::Sequence(seq) => {
                for ber in seq {
                    sw.write_slice(&ber.to_der())
                }
            }
            ASN1Type::SequenceOwned(ref seq) => {
                for ber in seq {
                    sw.write_slice(&ber.to_der())
                }
            }
            _ => unreachable!(),
        };
        sw.into_inner()
    }

    pub fn get_value(&self) -> &ASN1Type {
        &self.value
    }

    fn total_length(&self) -> u16 {
        let value_len = self.value_len();
        // tag (1 byte) - length (1-3 byte) - value (value_len)
        if value_len > 0xff {
            value_len + 4
        } else if value_len > 0x7f {
            value_len + 3
        } else {
            value_len + 2
        }
    }

    fn value_len(&self) -> u16 {
        match self.value {
            ASN1Type::U32(x) => {
                if x < 0x80 {
                    1
                } else if x < 0x8000 {
                    2
                } else if x < 0x800000 {
                    3
                } else if x < 0x80000000 {
                    4
                } else {
                    unreachable!()
                }
            }
            ASN1Type::OctetString(s) => s.len().try_into().unwrap(),
            ASN1Type::Priv(p) => p.total_length(),
            ASN1Type::PrivOwned(ref p) => p.total_length(),
            ASN1Type::Sequence(seq) => {
                let mut len = 0;
                for ber in seq {
                    len += ber.total_length();
                }
                len
            }
            ASN1Type::SequenceOwned(ref seq) => {
                let mut len = 0;
                for ber in seq {
                    len += ber.total_length();
                }
                len
            }
            _ => unimplemented!(),
        }
    }
}

macro_rules! ber {
    ($tag: expr, $val:expr) => {
        BerObj::new_with_tag($tag, $val)
    };

    ($val: expr) => {
        BerObj::new($val.into())
    };
}

macro_rules! ber_seq {
    ($($bers:expr),*) => {
        BerObj::new(([$($bers), *][..]).into())
    };

    ($($bers:expr,)*) => {
        ber_seq!($($bers), *)
    };
}
