use super::{Header, Side};

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

    pub fn recursion_available(&self) -> bool {
        self.block & RA_MASK != 0
    }

    pub fn rcode(&self) -> u8 {
        (self.block & RCODE) as u8
    }
}
