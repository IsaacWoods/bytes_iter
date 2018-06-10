#![no_std]

// TODO: can this be easily extended to support both endianesses?
pub struct ByteReader<'a, T>(T) where T: Iterator<Item=&'a u8>;

impl<'a, T> ByteReader<'a, T>
where
    T: Iterator<Item=&'a u8>
{
    pub fn new(iter: T) -> ByteReader<'a, T> {
        ByteReader(iter)
    }

    pub fn next_u8(&mut self) -> Option<u8> {
        self.0.next().map(|&x| x)
    }

    pub fn next_u16(&mut self) -> Option<u16> {
        let first = self.next_u8()? as u16;
        let second = self.next_u8()? as u16;

        Some((second << 8) | first)
    }

    pub fn next_u32(&mut self) -> Option<u32> {
        let first = self.next_u16()? as u32;
        let second = self.next_u16()? as u32;

        Some((second << 16) | first)
    }

    pub fn next_u64(&mut self) -> Option<u64> {
        let first = self.next_u32()? as u64;
        let second = self.next_u32()? as u64;

        Some((second << 32) | first)
    }
}

#[cfg(test)]
mod tests {
    use ByteReader;

    #[test]
    fn consume_slice() {
        let slice = &[0x45, 0xf6, 0xde, 0x34, 0x98, 0x77];
        let mut reader = ByteReader::new(slice.iter());
        assert_eq!(reader.next_u8(), Some(0x45));
        assert_eq!(reader.next_u32(), Some(0x98_34_de_f6));
        assert_eq!(reader.next_u16(), None);
    }
}
