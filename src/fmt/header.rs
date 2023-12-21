use nom::IResult;

#[derive(Default)]
pub struct Header {
    pub id: u16,
    block: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Query = 0,
    Response = 1,
}

const NULL_MASK: u16 = 0x0000;
const QR_MASK: u16 = 0x8000;
const OP_MASK: u16 = 0x7800;
const AUTH_MASK: u16 = 0x0400;
const TRUNC_MASK: u16 = 0x0200;
const RD_MASK: u16 = 0x0100;
const RA_MASK: u16 = 0x0080;

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
        self.block & 0x0400 != 0
    }

    pub fn truncated(&self) -> bool {
        self.block & 0x0200 != 0
    }

    pub fn recursion_desired(&self) -> bool {
        self.block & 0x0100 != 0
    }

    pub fn recursion_available(&self) -> bool {
        self.block & 0x0080 != 0
    }

    pub fn rcode(&self) -> u8 {
        (self.block & 0x000F) as u8
    }

    pub fn write_to(&self, buf: &mut [u8]) -> usize {
        buf[0..2].copy_from_slice(&self.id.to_be_bytes());
        buf[2..4].copy_from_slice(&self.block.to_be_bytes());
        buf[4..6].copy_from_slice(&self.qd_count.to_be_bytes());
        buf[6..8].copy_from_slice(&self.an_count.to_be_bytes());
        buf[8..10].copy_from_slice(&self.ns_count.to_be_bytes());
        buf[10..12].copy_from_slice(&self.ar_count.to_be_bytes());

        12
    }
}

impl Header {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Header> {
        use nom::number::streaming::u16;
        use nom::number::Endianness;
        let word = u16(Endianness::Big);
        let mut header = nom::sequence::tuple((word, word, word, word, word, word));

        let (input, (id, block, qd_count, an_count, ns_count, ar_count)) = header(input)?;

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

#[test]
fn test_op() {
    let mut header = Header::default();
    header.set_opcode(3);

    assert_eq!(header.opcode(), 3);
}
