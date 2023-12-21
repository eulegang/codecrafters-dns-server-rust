use super::*;
use nom::bytes::streaming::take;
use nom::number::{
    streaming::{u16, u8},
    Endianness,
};
use nom::IResult;

fn word(input: &[u8]) -> IResult<&[u8], u16> {
    u16(Endianness::Big)(input)
}

impl Bincode for Header {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend(self.id.to_be_bytes());
        buf.extend(self.id.to_be_bytes());
        buf.extend(self.block.to_be_bytes());
        buf.extend(self.qd_count.to_be_bytes());
        buf.extend(self.an_count.to_be_bytes());
        buf.extend(self.ns_count.to_be_bytes());
        buf.extend(self.ar_count.to_be_bytes());
    }

    fn decode(buf: &[u8]) -> nom::IResult<&[u8], Self> {
        let mut header = nom::sequence::tuple((word, word, word, word, word, word));

        let (input, (id, block, qd_count, an_count, ns_count, ar_count)) = header(buf)?;

        Ok((
            input,
            Header {
                id,
                block,
                qd_count,
                an_count,
                ns_count,
                ar_count,
            },
        ))
    }
}

impl Bincode for Name {
    fn encode(&self, buf: &mut Vec<u8>) {
        for name in &self.name {
            let len = name.len() as u8;

            buf.push(len);
            buf.extend(name);
        }

        buf.push(0);
    }

    fn decode(mut buf: &[u8]) -> nom::IResult<&[u8], Self> {
        let mut name = Vec::new();
        let mut slice: &[u8];

        loop {
            let len: u8;
            (buf, len) = u8(buf)?;

            if len == 0 {
                return Ok((buf, Name { name }));
            }

            (buf, slice) = take(len as usize)(buf)?;

            name.push(slice.to_vec());
        }
    }
}

impl Bincode for Question {
    fn encode(&self, buf: &mut Vec<u8>) {
        self.name.encode(buf);
        self.ty.encode(buf);
        self.class.encode(buf);
    }

    fn decode(buf: &[u8]) -> nom::IResult<&[u8], Self> {
        let (buf, name) = Name::decode(buf)?;
        let (buf, ty) = Type::decode(buf)?;
        let (buf, class) = Class::decode(buf)?;

        Ok((buf, Question { name, ty, class }))
    }
}

impl Bincode for Type {
    fn encode(&self, buf: &mut Vec<u8>) {
        let tag: u16 = match self {
            Type::A => 1,
            Type::NS => 2,
            Type::MD => 3,
            Type::MF => 4,
            Type::CNAME => 5,
            Type::SOA => 6,
            Type::MB => 7,
            Type::MG => 8,
            Type::MR => 9,
            Type::NULL => 10,
            Type::WKS => 11,
            Type::PTR => 12,
            Type::HINFO => 13,
            Type::MINFO => 14,
            Type::MX => 15,
            Type::TXT => 16,
        };

        buf.extend(tag.to_be_bytes());
    }

    fn decode(buf: &[u8]) -> nom::IResult<&[u8], Self> {
        let (buf, tag) = u16(Endianness::Big)(buf)?;

        let ty = match tag {
            1 => Type::A,
            2 => Type::NS,
            3 => Type::MD,
            4 => Type::MF,
            5 => Type::CNAME,
            6 => Type::SOA,
            7 => Type::MB,
            8 => Type::MG,
            9 => Type::MR,
            10 => Type::NULL,
            11 => Type::WKS,
            12 => Type::PTR,
            13 => Type::HINFO,
            14 => Type::MINFO,
            15 => Type::MX,
            16 => Type::TXT,

            _ => {
                return Err(nom::Err::Failure(nom::error::Error {
                    input: buf,
                    code: nom::error::ErrorKind::Tag,
                }))
            }
        };

        Ok((buf, ty))
    }
}

impl Bincode for Class {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(0);
        buf.push(1);
    }

    fn decode(buf: &[u8]) -> nom::IResult<&[u8], Self> {
        let (input, tag) = u16(Endianness::Big)(buf)?;

        let class = match tag {
            1 => Class::In,

            _ => {
                return Err(nom::Err::Failure(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Tag,
                }))
            }
        };

        Ok((input, class))
    }
}
