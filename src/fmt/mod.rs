mod bincode;
mod impls;

pub trait Bincode: Sized {
    fn encode(&self, buf: &mut Vec<u8>);
    fn decode(buf: &[u8]) -> nom::IResult<&[u8], Self>;
}

#[derive(Default, Debug, PartialEq, Clone)]
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

#[derive(PartialEq, Eq, Clone)]
pub struct Name {
    name: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Class {
    In,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Question {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Resource {
    pub name: Name,
    pub ty: Type,
    pub class: Class,
    pub ttl: u32,
    pub data: RData,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RData(Vec<u8>);
