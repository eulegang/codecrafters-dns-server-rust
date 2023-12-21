mod bincode;
mod impls;

pub trait Bincode: Sized {
    fn encode(&self, buf: &mut Vec<u8>);
    fn decode(buf: &[u8]) -> nom::IResult<&[u8], Self>;
}

#[derive(Default, Debug, PartialEq)]
pub struct Header {
    pub id: u16,
    pub block: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Query,
    Response,
}

#[derive(Debug, PartialEq)]
pub struct Name {
    name: Vec<Vec<u8>>,
}

pub enum Type {
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    NULL,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
}

pub enum Class {
    In,
}

pub struct Question {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
}
