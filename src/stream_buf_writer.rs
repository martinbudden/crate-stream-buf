#![allow(unused)]

use crate::stream_buf_reader::StreamBufReader;
use core::mem;
use core::ops::{Index, IndexMut};

/// Simple serializer/deserializer
pub struct StreamBufWriter<'a> {
    pos: usize,
    buf: &'a mut [u8],
}

/*The 'a notation in Rust is a lifetime parameter that tells the compiler how long a reference remains valid.

It starts with an apostrophe (e.g., 'a, 'b).
It ensures references don't outlive the data they point to.
Used in functions, structs, and generics to link the lifetimes of multiple references.
The name 'a is conventional; you can use others like 'b, but 'a is standard for the first lifetime
*/
impl<'a> StreamBufWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { pos: 0, buf }
    }

    pub fn get_data(&self) -> &[u8] {
        self.buf
    }

    pub fn get_data_slice(&self) -> &[u8] {
        &self.buf[..self.pos]
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.pos == 0
    }

    pub fn is_full(&self) -> bool {
        //self.bytes_remaining() == 0
        //self.is_available(1)
        self.pos >= self.buf.len()
    }

    pub fn bytes_remaining(&self) -> usize {
        let rem: isize = self.buf.len().cast_signed() - self.pos.cast_signed();
        if rem <= 0 { 0_usize } else { rem.cast_unsigned() }
    }

    pub fn is_available(&self, size: usize) -> bool {
        self.pos + size <= self.buf.len()
    }

    pub fn bytes_written(&self) -> usize {
        self.pos
    }

    pub fn advance(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.buf.len());
    }

    pub fn get_ref(&self) -> &[u8] {
        &self.buf[..self.pos]
    }

    pub fn at(&self, index: usize) -> u8 {
        self.buf[index]
    }

    pub fn write_u8(&mut self, value: u8) {
        const WRITE_SIZE: usize = size_of::<u8>();
        if self.is_available(WRITE_SIZE) {
            self.buf[self.pos] = value;
            self.pos += 1;
        }
    }

    /// Write a u16 to the streambuf.
    /// ```
    /// # use stream_buf::StreamBufWriter;
    /// const BUF_SIZE: usize = 8;
    /// let mut data = [0u8; BUF_SIZE];
    /// let mut sbuf_writer = StreamBufWriter::new(&mut data);
    ///
    /// sbuf_writer.write_u16(0x0a1b);
    ///
    /// assert_eq!([0x1b,0x0a], data[0..2]);
    /// ```
    pub fn write_u16(&mut self, value: u16) {
        const WRITE_SIZE: usize = size_of::<u16>();
        if self.is_available(WRITE_SIZE) {
            let bytes = value.to_le_bytes();
            self.buf[self.pos] = bytes[0];
            self.buf[self.pos + 1] = bytes[1];
            self.pos += 2;
        }
    }

    /// Write an u32 to the streambuf.
    /// ```
    /// # use stream_buf::StreamBufWriter;
    /// const BUF_SIZE: usize = 8;
    /// let mut data = [0u8; BUF_SIZE];
    /// let mut sbuf_writer = StreamBufWriter::new(&mut data);
    ///
    /// sbuf_writer.write_u32(0x0a1b2c3d);
    ///
    /// assert_eq!([0x3d,0x2c,0x1b,0x0a], data[0..4]);
    /// ```
    pub fn write_u32(&mut self, value: u32) {
        //let value: u32 = 0x12345678;
        //let bytes: [u8; 4] = value.to_le_bytes(); // [0x78, 0x56, 0x34, 0x12]
        const WRITE_SIZE: usize = size_of::<u32>();
        if self.is_available(WRITE_SIZE) {
            value.to_le_bytes().iter().for_each(|&byte| {
                self.buf[self.pos] = byte;
                self.pos += 1;
            });
        }
    }

    /// Write a u16 to the streambuf, big endian.
    /// ```
    /// # use stream_buf::StreamBufWriter;
    /// const BUF_SIZE: usize = 8;
    /// let mut data = [0u8; BUF_SIZE];
    /// let mut sbuf_writer = StreamBufWriter::new(&mut data);
    ///
    /// sbuf_writer.write_u16_big_endian(0x0a1b);
    ///
    /// assert_eq!([0x0a,0x1b], data[0..2]);
    /// ```
    pub fn write_u16_big_endian(&mut self, value: u16) {
        const WRITE_SIZE: usize = size_of::<u16>();
        if self.is_available(WRITE_SIZE) {
            value.to_be_bytes().iter().for_each(|&byte| {
                self.buf[self.pos] = byte;
                self.pos += 1;
            });
        }
    }

    /// Write an u32 to the streambuf, big endian.
    /// ```
    /// # use stream_buf::StreamBufWriter;
    /// const BUF_SIZE: usize = 8;
    /// let mut data = [0u8; BUF_SIZE];
    /// let mut sbuf_writer = StreamBufWriter::new(&mut data);
    ///
    /// sbuf_writer.write_u32_big_endian(0x0a1b2c3d);
    ///
    /// assert_eq!([0x0a,0x1b,0x2c,0x3d], data[0..4]);
    /// ```
    pub fn write_u32_big_endian(&mut self, value: u32) {
        const WRITE_SIZE: usize = size_of::<u32>();
        if self.is_available(WRITE_SIZE) {
            value.to_be_bytes().iter().for_each(|&byte| {
                self.buf[self.pos] = byte;
                self.pos += 1;
            });
        }
    }

    /// Write an f32 to the streambuf.
    /// ```
    /// # use stream_buf::StreamBufWriter;
    /// const BUF_SIZE: usize = 8;
    /// let mut data = [0u8; BUF_SIZE];
    /// let mut sbuf_writer = StreamBufWriter::new(&mut data);
    ///
    /// sbuf_writer.write_f32(1234.56);
    ///
    /// assert_eq!([0xec, 0x51, 0x9a, 0x44], data[0..4]);
    /// ```
    pub fn write_f32(&mut self, value: f32) {
        let bits = value.to_bits().cast_signed();
        self.write_u32(bits.cast_unsigned());
    }

    pub fn fill_without_advancing(&mut self, data: u8, len: usize) -> bool {
        if (self.pos + len > self.buf.len()) {
            return false;
        }
        self.buf[self.pos..self.pos + len].fill(data);
        true
    }

    pub fn fill(&mut self, data: u8, len: usize) {
        if self.fill_without_advancing(data, len) {
            self.pos += len;
        }
    }

    pub fn write(&mut self, src: &[u8]) -> usize {
        let write_size = src.len();
        if self.is_available(write_size) {
            self.buf[self.pos..self.pos + write_size].copy_from_slice(src);
            self.pos += write_size;
            return write_size;
        }
        0
    }

    pub fn write_str(&mut self, src: &str) -> usize {
        let write_size = src.len();
        if self.is_available(write_size) {
            #[allow(clippy::useless_conversion)]
            let result = src.as_bytes().try_into();
            match result {
                Ok(bytes) => {
                    self.buf[self.pos..self.pos + write_size].copy_from_slice(bytes);
                    self.pos += write_size;
                    return write_size;
                }
                Err(error) => {
                    return 0;
                }
            }
        }
        0
    }

    pub fn write_str_with_zero_terminator(&mut self, src: &str) -> usize {
        let write_size = src.len() + 1;
        if self.is_available(write_size) {
            _ = self.write_str(src);
            self.write_u8(0);
            return write_size;
        }
        0
    }
}

/// Access `StreamBufWriter` component by index
impl Index<usize> for StreamBufWriter<'_> {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 {
        &self.buf[index]
    }
}

/// Set `StreamBufWriter` component by index
impl IndexMut<usize> for StreamBufWriter<'_> {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.buf[index]
    }
}

impl<'a> From<StreamBufWriter<'a>> for StreamBufReader<'a> {
    fn from(sbuf: StreamBufWriter<'a>) -> Self {
        Self::new(&sbuf.buf[..sbuf.pos()])
        //Self::new(&sbuf.buf[..sbuf.pos()], sbuf.bytes_written())
        //Self::new(&sbuf.buf[..], sbuf.bytes_written())
    }
}

/*
StreamBufWriter<'a> uses a lifetime 'a,
buf: &'a mut [u8] borrows the array/slice passed in.
new() takes &'a mut [u8] — works with arrays like &mut [u8; 32].

let mut data = [0u8; 64];
let mut buf = SafeStreamBuf::new(&mut data);
buf.write_u16(0x1234);
*/
#[cfg(any(debug_assertions, test))]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;
    
    #[test]
    fn new() {
        const BUF_SIZE: usize = 64;
        let mut data = [0u8; BUF_SIZE];
        let mut sbuf = StreamBufWriter::new(&mut data);
        sbuf.write_u16(0x1234);
        assert_eq!(sbuf[0], 0x34);
        assert_eq!(sbuf[1], 0x12);
    }

    #[test]
    fn stream_buf() {
        const BUF_SIZE: usize = 256;
        let mut data = [0u8; BUF_SIZE];
        let mut sbuf = StreamBufWriter::new(&mut data);

        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());

        sbuf.write_u8(1);
        assert_eq!(1, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 1, sbuf.bytes_remaining());

        sbuf.write_u16(2);
        assert_eq!(3, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 3, sbuf.bytes_remaining());

        sbuf.write_u32(3);
        assert_eq!(7, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 7, sbuf.bytes_remaining());

        let mut sbuf_reader: StreamBufReader = sbuf.into();
        assert_eq!(0, sbuf_reader.bytes_read());
        assert_eq!(7, sbuf_reader.bytes_remaining());

        let v1 = sbuf_reader.read_u8();
        assert_eq!(1, v1);
        assert_eq!(1, sbuf_reader.bytes_read());
        assert_eq!(6, sbuf_reader.bytes_remaining());

        let v2 = sbuf_reader.read_u16();
        assert_eq!(2, v2);
        assert_eq!(3, sbuf_reader.bytes_read());
        assert_eq!(4, sbuf_reader.bytes_remaining());

        let v3 = sbuf_reader.read_u32();
        assert_eq!(3, v3);
        assert_eq!(7, sbuf_reader.bytes_read());
        assert_eq!(0, sbuf_reader.bytes_remaining());
    }

    #[test]
    fn stream_buf_big_endian() {
        const BUF_SIZE: usize = 256;
        let mut data = [0u8; BUF_SIZE];
        let mut sbuf = StreamBufWriter::new(&mut data);

        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());

        sbuf.write_u8(1);
        assert_eq!(BUF_SIZE - 1, sbuf.bytes_remaining());

        sbuf.write_u16_big_endian(2);
        assert_eq!(BUF_SIZE - 3, sbuf.bytes_remaining());

        sbuf.write_u32_big_endian(3);
        assert_eq!(BUF_SIZE - 7, sbuf.bytes_remaining());

        let mut sbuf_reader: StreamBufReader = sbuf.into();
        assert_eq!(0, sbuf_reader.bytes_read());
        assert_eq!(7, sbuf_reader.bytes_remaining());

        let v1 = sbuf_reader.read_u8();
        assert_eq!(1, v1);
        assert_eq!(1, sbuf_reader.bytes_read());
        assert_eq!(6, sbuf_reader.bytes_remaining());

        let v2 = sbuf_reader.read_u16_big_endian();
        assert_eq!(2, v2);
        assert_eq!(3, sbuf_reader.bytes_read());
        assert_eq!(4, sbuf_reader.bytes_remaining());

        let v3 = sbuf_reader.read_u32_big_endian();
        assert_eq!(3, v3);
        assert_eq!(7, sbuf_reader.bytes_read());
        assert_eq!(0, sbuf_reader.bytes_remaining());
    }

    #[test]
    fn stream_buf_size() {
        const BUF_SIZE: usize = 2;
        let mut buf = [0u8; BUF_SIZE];
        buf.fill(0xFF);
        assert_eq!(0xFF, buf[0]);
        assert_eq!(0xFF, buf[1]);

        let mut sbuf = StreamBufWriter::new(&mut buf);

        assert!(sbuf.is_empty());
        //assert!(sbuf.is_full());
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        assert_eq!(0, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(sbuf.is_available(1));
        assert!(sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        sbuf.write_u16(0xABCD);

        assert!(!sbuf.is_empty());
        assert_eq!(2, sbuf.pos());
        assert!(sbuf.is_full());
        assert_eq!(0, sbuf.bytes_remaining());
        assert_eq!(2, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(!sbuf.is_available(1));
        assert!(!sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        assert_eq!(0xCD, sbuf[0]);
        assert_eq!(0xAB, sbuf[1]);

        sbuf.reset();
        assert!(sbuf.is_empty());
        assert!(!sbuf.is_full());
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        assert_eq!(0, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(sbuf.is_available(1));
        assert!(sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        // try and write a u32. this will fail since there is not enough room
        sbuf.write_u32(0xABCD_1234);
        assert!(sbuf.is_empty());
        assert!(!sbuf.is_full());
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        assert_eq!(0, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(sbuf.is_available(1));
        assert!(sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        sbuf.write_u8(0xAB);
        assert!(!sbuf.is_empty());
        assert!(!sbuf.is_full());
        assert_eq!(1, sbuf.bytes_remaining());
        assert_eq!(1, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(sbuf.is_available(1));
        assert!(!sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        // try and write a u16, this will fail since there is not enough room
        sbuf.write_u16(0xABCD);
        assert!(!sbuf.is_empty());
        assert!(!sbuf.is_full());
        assert_eq!(1, sbuf.bytes_remaining());
        assert_eq!(1, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(sbuf.is_available(1));
        assert!(!sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        sbuf.write_u8(0xAB);
        assert!(!sbuf.is_empty());
        assert!(sbuf.is_full());
        assert_eq!(0, sbuf.bytes_remaining());
        assert_eq!(2, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(!sbuf.is_available(1));
        assert!(!sbuf.is_available(2));
        assert!(!sbuf.is_available(3));

        // try and write a u8, this will fail since there is not enough room
        sbuf.write_u8(0xAB);
        assert!(!sbuf.is_empty());
        assert!(sbuf.is_full());
        assert_eq!(0, sbuf.bytes_remaining());
        assert_eq!(2, sbuf.bytes_written());
        assert!(sbuf.is_available(0));
        assert!(!sbuf.is_available(1));
        assert!(!sbuf.is_available(2));
        assert!(!sbuf.is_available(3));
    }

    #[test]
    fn stream_buf_strings() {
        const BUF_SIZE: usize = 6;
        let mut buf = [0u8; BUF_SIZE];
        let mut sbuf = StreamBufWriter::new(&mut buf);

        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        _ = sbuf.fill_without_advancing(0xFF, BUF_SIZE);
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        assert_eq!(0xFF, sbuf[0]);
        assert_eq!(0xFF, sbuf[1]);
        assert_eq!(0xFF, sbuf[2]);
        assert_eq!(0xFF, sbuf[3]);
        assert_eq!(0xFF, sbuf[4]);
        assert_eq!(0xFF, sbuf[5]);

        _ = sbuf.write_str("Hello");
        assert_eq!(5, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 5, sbuf.bytes_remaining());
        assert_eq!('H', sbuf[0] as char);
        assert_eq!('e', sbuf[1] as char);
        assert_eq!('l', sbuf[2] as char);
        assert_eq!('l', sbuf[3] as char);
        assert_eq!('o', sbuf[4] as char);
        assert_eq!(0xFF, sbuf[5]);

        sbuf.reset();
        assert_eq!(0, sbuf.bytes_written());
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        _ = sbuf.fill_without_advancing(0xFF, BUF_SIZE);

        _ = sbuf.write_str_with_zero_terminator("Hello");
        assert_eq!(6, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 6, sbuf.bytes_remaining());
        assert_eq!('H', sbuf[0] as char);
        assert_eq!('e', sbuf[1] as char);
        assert_eq!('l', sbuf[2] as char);
        assert_eq!('l', sbuf[3] as char);
        assert_eq!('o', sbuf[4] as char);
        assert_eq!(0, sbuf[5]);
    }

    #[test]
    fn stream_buf_float() {
        const BUF_SIZE: usize = 256;
        let mut buf = [0u8; BUF_SIZE];
        let mut sbuf = StreamBufWriter::new(&mut buf);

        assert_eq!(0, sbuf.bytes_written());
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());
        //sbuf.fill_without_advancing(0xFF, BUF_SIZE);
        assert_eq!(0, sbuf.bytes_written());
        assert_eq!(BUF_SIZE, sbuf.bytes_remaining());

        sbuf.write_f32(18.9);
        assert_eq!(4, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 4, sbuf.bytes_remaining());

        sbuf.write_u8(7);

        sbuf.write_u16(19);

        sbuf.write_f32(3.12345);
        assert_eq!(11, sbuf.bytes_written());
        assert_eq!(BUF_SIZE - 11, sbuf.bytes_remaining());
        assert!(sbuf.is_available(BUF_SIZE - 12));
        assert!(sbuf.is_available(BUF_SIZE - 11));
        assert!(!sbuf.is_available(BUF_SIZE - 10));

        let mut sbuf_reader: StreamBufReader = sbuf.into();
        assert_eq!(0, sbuf_reader.bytes_read());
        assert_eq!(11, sbuf_reader.bytes_remaining());

        let v1 = sbuf_reader.read_f32();
        assert_eq!(18.9, v1);
        assert_eq!(7, sbuf_reader.bytes_remaining());

        let v2 = sbuf_reader.read_u8();
        assert_eq!(7, v2);
        assert_eq!(6, sbuf_reader.bytes_remaining());

        let v3 = sbuf_reader.read_u16();
        assert_eq!(19, v3);
        assert_eq!(4, sbuf_reader.bytes_remaining());

        let v4 = sbuf_reader.read_f32();
        assert_eq!(3.12345, v4);
        assert_eq!(0, sbuf_reader.bytes_remaining());
    }
    #[test]
    fn write_f32() {
        let mut data = [0u8; 8];
        let mut sbuf_writer = StreamBufWriter::new(&mut data);
        sbuf_writer.write_f32(1234.56);
        assert_eq!([0xec, 0x51, 0x9a, 0x44], data[0..4]);
    }
}
