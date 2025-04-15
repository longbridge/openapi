use std::io::{Cursor, Read};

use byteorder::{BE, ReadBytesExt, WriteBytesExt};
use flate2::read::GzDecoder;

use crate::{WsClientError, WsClientResult};

const PACKAGE_TYPE_REQUEST: u8 = 1;
const PACKAGE_TYPE_RESPONSE: u8 = 2;
const PACKAGE_TYPE_PUSH: u8 = 3;

#[derive(Debug)]
pub(crate) struct Signature {
    pub(crate) nonce: [u8; 8],
    pub(crate) signature: [u8; 16],
}

#[derive(Debug)]
pub(crate) struct PacketHeader {
    ty: u8,
    verify: bool,
    gzip: bool,
}

impl PacketHeader {
    #[inline]
    pub(crate) fn encode(&self) -> u8 {
        let mut data = self.ty & 0b00001111;
        if self.verify {
            data |= 0b00010000;
        }
        if self.gzip {
            data |= 0b00100000;
        }
        data
    }

    #[inline]
    pub(crate) fn decode(data: u8) -> PacketHeader {
        let ty = data & 0b00001111;
        let verify = (data & 0b00010000) > 0;
        let gzip = (data & 0b00100000) > 0;
        PacketHeader { ty, verify, gzip }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum Packet {
    Request {
        command_code: u8,
        request_id: u32,
        timeout_millis: u16,
        body: Vec<u8>,
        signature: Option<Signature>,
    },
    Response {
        command_code: u8,
        request_id: u32,
        status: u8,
        body: Vec<u8>,
        signature: Option<Signature>,
    },
    Push {
        command_code: u8,
        body: Vec<u8>,
        signature: Option<Signature>,
    },
}

impl Packet {
    pub(crate) fn encode(&self) -> Vec<u8> {
        match self {
            Packet::Request {
                command_code,
                request_id,
                timeout_millis: timeout,
                body,
                signature,
            } => {
                let header = PacketHeader {
                    ty: PACKAGE_TYPE_REQUEST,
                    verify: signature.is_some(),
                    gzip: false,
                }
                .encode();
                let mut data = vec![header, *command_code];

                let _ = data.write_u32::<BE>(*request_id);
                let _ = data.write_u16::<BE>(*timeout);
                let _ = data.write_u24::<BE>(body.len() as u32);

                data.extend(body);

                if let Some(signature) = signature {
                    data.extend_from_slice(&signature.nonce);
                    data.extend_from_slice(&signature.signature);
                }

                data
            }
            Packet::Response { .. } => unimplemented!(),
            Packet::Push { .. } => unimplemented!(),
        }
    }

    pub(crate) fn decode(data: &[u8]) -> WsClientResult<Packet> {
        if data.is_empty() {
            return Err(WsClientError::UnexpectedResponse);
        }

        let header = PacketHeader::decode(data[0]);
        let res = match header.ty {
            PACKAGE_TYPE_REQUEST => parse_request(&header, &data[1..]),
            PACKAGE_TYPE_RESPONSE => parse_response(&header, &data[1..]),
            PACKAGE_TYPE_PUSH => parse_push(&header, &data[1..]),
            _ => return Err(WsClientError::UnexpectedResponse),
        };
        res.and_then(|packet| {
            if header.gzip {
                packet.map_body(|body| {
                    let mut new_body = Vec::new();
                    let mut decoder = GzDecoder::new(&*body);
                    decoder.read_to_end(&mut new_body)?;
                    Ok(new_body)
                })
            } else {
                Ok(packet)
            }
        })
        .map_err(|_| WsClientError::UnexpectedResponse)
    }

    #[inline]
    fn map_body<F, Err>(self, f: F) -> Result<Self, Err>
    where
        F: FnOnce(Vec<u8>) -> Result<Vec<u8>, Err>,
    {
        Ok(match self {
            Packet::Request {
                command_code,
                request_id,
                timeout_millis,
                body,
                signature,
            } => Self::Request {
                command_code,
                request_id,
                timeout_millis,
                body: f(body)?,
                signature,
            },
            Packet::Response {
                command_code,
                request_id,
                status,
                body,
                signature,
            } => Self::Response {
                command_code,
                request_id,
                status,
                body: f(body)?,
                signature,
            },
            Packet::Push {
                command_code,
                body,
                signature,
            } => Self::Push {
                command_code,
                body: f(body)?,
                signature,
            },
        })
    }
}

fn parse_signature(rdr: &mut impl Read) -> std::io::Result<Signature> {
    let mut nonce = [0; 8];
    let mut signature = [0; 16];
    rdr.read_exact(&mut nonce)?;
    rdr.read_exact(&mut signature)?;
    Ok(Signature { nonce, signature })
}

fn parse_request(header: &PacketHeader, data: &[u8]) -> std::io::Result<Packet> {
    let mut cursor = Cursor::new(data);
    let command_code = cursor.read_u8()?;
    let request_id = cursor.read_u32::<BE>()?;
    let timeout = cursor.read_u16::<BE>()?;
    let body_len = cursor.read_u24::<BE>()?;

    let mut body = vec![0; body_len as usize];
    cursor.read_exact(&mut body)?;

    let signature = if header.verify {
        Some(parse_signature(&mut cursor)?)
    } else {
        None
    };

    Ok(Packet::Request {
        command_code,
        request_id,
        timeout_millis: timeout,
        body,
        signature,
    })
}

fn parse_response(header: &PacketHeader, data: &[u8]) -> std::io::Result<Packet> {
    let mut cursor = Cursor::new(data);
    let command_code = cursor.read_u8()?;
    let request_id = cursor.read_u32::<BE>()?;
    let status = cursor.read_u8()?;
    let body_len = cursor.read_u24::<BE>()?;
    let mut body = vec![0; body_len as usize];
    cursor.read_exact(&mut body)?;

    let signature = if header.verify {
        Some(parse_signature(&mut cursor)?)
    } else {
        None
    };

    debug_assert_eq!(cursor.position() as usize, data.len());

    Ok(Packet::Response {
        command_code,
        request_id,
        status,
        body,
        signature,
    })
}

fn parse_push(header: &PacketHeader, data: &[u8]) -> std::io::Result<Packet> {
    let mut cursor = Cursor::new(data);
    let command_code = cursor.read_u8()?;
    let body_len = cursor.read_u24::<BE>()?;
    let mut body = vec![0; body_len as usize];
    cursor.read_exact(&mut body)?;

    let signature = if header.verify {
        Some(parse_signature(&mut cursor)?)
    } else {
        None
    };

    Ok(Packet::Push {
        command_code,
        body,
        signature,
    })
}
