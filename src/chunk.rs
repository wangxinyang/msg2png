use crate::chunk_type::ChunkType;
use crate::error::MyError;
use crate::Result;
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt::Display;

/// A validated PNG chunk. See the PNG Spec for more details
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[allow(dead_code)]
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc_data = chunk_type
            .item
            .clone()
            .into_iter()
            .chain(data.clone().into_iter())
            .collect::<Vec<u8>>();
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc: generate_crc(crc_data.as_ref()),
        }
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        self.data.as_ref()
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        todo!()
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

fn generate_crc(bytes: &[u8]) -> u32 {
    let crc_data = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    crc_data.checksum(bytes)
}

impl TryFrom<&[u8]> for Chunk {
    type Error = MyError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let vec_val = value.to_vec();
        let length = u32::from_be_bytes(vec_val.get(0..5).unwrap().try_into().unwrap());
        let chunk_type_num = vec_val.get(5..10).unwrap();
        let item = chunk_type_num.to_owned();
        let data_num = vec_val.get(10..length as usize).unwrap().to_vec();
        let crc = u32::from_be_bytes(vec_val.get(length as usize..).unwrap().try_into().unwrap());
        Ok(Chunk {
            length,
            chunk_type: ChunkType { item },
            data: data_num,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
