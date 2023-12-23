use std::net::Ipv4Addr;

use super::{Header, Name, RData, Side};

const NULL_MASK: u16 = 0x0000;
const QR_MASK: u16 = 0x8000;
const OP_MASK: u16 = 0x7800;
const AUTH_MASK: u16 = 0x0400;
const TRUNC_MASK: u16 = 0x0200;
const RD_MASK: u16 = 0x0100;
const RA_MASK: u16 = 0x0080;
const RCODE: u16 = 0x000F;

#[allow(dead_code)]
impl Header {
    pub fn side(&self) -> Side {
        if self.block & QR_MASK == 0 {
            Side::Query
        } else {
            Side::Response
        }
    }

    pub fn set_side(&mut self, side: Side) {
        let s = match side {
            Side::Query => NULL_MASK,
            Side::Response => QR_MASK,
        };

        self.block &= !QR_MASK;
        self.block |= s
    }

    pub fn opcode(&self) -> u8 {
        ((self.block & OP_MASK) >> 11) as u8
    }

    pub fn set_opcode(&mut self, opcode: u8) {
        self.block &= !OP_MASK;
        self.block |= (opcode as u16 & 0x0F) << 11
    }

    pub fn authoritative(&self) -> bool {
        self.block & AUTH_MASK != 0
    }

    pub fn truncated(&self) -> bool {
        self.block & TRUNC_MASK != 0
    }

    pub fn recursion_desired(&self) -> bool {
        self.block & RD_MASK != 0
    }

    pub fn set_recursion_desired(&mut self, desired: bool) {
        self.block &= !RD_MASK;
        if desired {
            self.block |= RD_MASK
        }
    }

    pub fn recursion_available(&self) -> bool {
        self.block & RA_MASK != 0
    }

    pub fn set_rcode(&mut self, rcode: u8) {
        self.block &= !RCODE;
        self.block |= rcode as u16 & RCODE;
    }

    pub fn rcode(&self) -> u8 {
        (self.block & RCODE) as u8
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        for name in &self.name {
            let Ok(name) = std::str::from_utf8(&name) else {
                return self.name.fmt(f);
            };

            res.push_str(name);
            res.push('.');
        }

        res.fmt(f)
    }
}

impl From<Ipv4Addr> for RData {
    fn from(value: Ipv4Addr) -> Self {
        RData(value.octets().to_vec())
    }
}

impl Name {
    pub fn abs(&mut self, packet: &[u8]) {
        if let Some(offset) = self.offset {
            let mut offset = offset as usize;
            self.offset = None;

            loop {
                let Some(len) = packet.get(offset) else {
                    return;
                };

                if *len == 0 {
                    break;
                }

                let Some(part) = packet.get(offset + 1..offset + 1 + *len as usize) else {
                    return;
                };

                self.name.push(part.to_vec());

                offset += 1 + *len as usize;
            }
        }
    }
}
