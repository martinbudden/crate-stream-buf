#![allow(unused)]

use core::mem;
use core::ops::Index;

/// Simple deserializer.
pub struct StreamBufReader<'a> {
    pos: usize,
    buf: &'a [u8],
}

/*The 'a notation in Rust is a lifetime parameter that tells the compiler how long a reference remains valid.

It starts with an apostrophe (e.g., 'a, 'b).
It ensures references don't outlive the data they point to.
Used in functions, structs, and generics to link the lifetimes of multiple references.
The name 'a is conventional; you can use others like 'b, but 'a is standard for the first lifetime
*/
impl<'a> StreamBufReader<'a> {
    #[must_use]
    pub const fn new(buf: &'a [u8]) -> Self {
        Self {
            pos: 0,
            //size: buf.len(),
            buf,
        }
    }

    #[must_use]
    pub fn get_data(&self) -> &'a [u8] {
        self.buf
    }

    #[must_use]
    pub fn get_data_slice(&self) -> &'a [u8] {
        &self.buf[..self.pos]
    }

    #[must_use]
    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pos == 0
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        self.pos >= self.buf.len()
    }

    #[must_use]
    pub fn bytes_remaining(&self) -> usize {
        //let rem: isize = self.size as isize - self.pos as isize;
        let rem: isize = self.buf.len().cast_signed() - self.pos.cast_signed();
        if rem <= 0 { 0_usize } else { rem.cast_unsigned() }
    }

    #[must_use]
    pub fn is_remaining(&self, size: usize) -> bool {
        self.pos + size <= self.buf.len()
    }

    #[must_use]
    pub fn bytes_read(&self) -> usize {
        self.pos
    }

    pub fn advance(&mut self, n: usize) {
        self.pos = (self.pos + n).min(self.buf.len());
    }

    #[must_use]
    pub fn get_ref(&self) -> &[u8] {
        &self.buf[..self.pos]
    }

    #[must_use]
    pub fn at(&self, index: usize) -> u8 {
        self.buf[index]
    }

    /// Return a u8 read from the `StreamBuf`.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let v = sbuf_reader.read_u8();
    ///
    /// assert_eq!(0x0a, v);
    /// ```
    pub fn read_u8(&mut self) -> u8 {
        const READ_SIZE: usize = size_of::<u8>();
        if !self.is_remaining(READ_SIZE) {
            return 0;
        }
        let pos = self.pos;
        self.advance(READ_SIZE);
        self.buf[pos]
    }

    /// Return a u16 read from the `StreamBuf`.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let v = sbuf_reader.read_u16();
    ///
    /// assert_eq!(0x1b0a, v);
    /// ```
    pub fn read_u16(&mut self) -> u16 {
        const READ_SIZE: usize = size_of::<u16>();
        if !self.is_remaining(READ_SIZE) {
            return 0;
        }
        let pos = self.pos;
        self.advance(READ_SIZE);
        u16::from_le_bytes([self.buf[pos], self.buf[pos + 1]])
    }

    /// Return a u32 read from the `StreamBuf`.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let v = sbuf_reader.read_u32();
    ///
    /// assert_eq!(0x3d2c1b0a, v);
    /// ```
    pub fn read_u32(&mut self) -> u32 {
        const READ_SIZE: usize = size_of::<u32>();
        if !self.is_remaining(READ_SIZE) {
            return 0;
        }
        let pos = self.pos;
        self.advance(READ_SIZE);
        u32::from_le_bytes([self.buf[pos], self.buf[pos + 1], self.buf[pos + 2], self.buf[pos + 3]])
        /*
        Alternatively:
        u32::from_le_bytes(self.buf[pos..pos+4].try_into().unwrap())
        let result = self.buf[pos..pos+4].try_into();
        match result {
            Ok(bytes) => { u32::from_le_bytes(bytes) },
            Err(error) => { 0 },
        }
        */
    }

    /// Return a u16 read from the `StreamBuf`.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let v = sbuf_reader.read_u16_big_endian();
    ///
    /// assert_eq!(0x0a1b, v);
    /// ```
    pub fn read_u16_big_endian(&mut self) -> u16 {
        const READ_SIZE: usize = size_of::<u16>();
        if !self.is_remaining(READ_SIZE) {
            return 0;
        }
        let pos = self.pos;
        self.advance(READ_SIZE);
        u16::from_be_bytes([self.buf[pos], self.buf[pos + 1]])
    }

    /// Return a u16 read from the `StreamBuf`.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let v = sbuf_reader.read_u32_big_endian();
    ///
    /// assert_eq!(0x0a1b2c3d, v);
    /// ```
    pub fn read_u32_big_endian(&mut self) -> u32 {
        const READ_SIZE: usize = size_of::<u32>();
        if !self.is_remaining(READ_SIZE) {
            return 0;
        }
        let pos = self.pos;
        self.advance(READ_SIZE);
        u32::from_be_bytes([self.buf[pos], self.buf[pos + 1], self.buf[pos + 2], self.buf[pos + 3]])
    }

    /// Return an f32 read from the `StreamBuf`.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0xec, 0x51, 0x9a, 0x44];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let v = sbuf_reader.read_f32();
    ///
    /// assert_eq!(1234.56, v);
    /// ```
    pub fn read_f32(&mut self) -> f32 {
        const READ_SIZE: usize = size_of::<f32>();
        if !self.is_remaining(READ_SIZE) {
            return 0.0;
        }
        let bits = self.read_u32();
        f32::from_bits(bits)
    }

    /// Read an array from the `StreamBuf`.
    /// Return the length read.
    /// ```
    /// # use stream_buf::StreamBufReader;
    ///
    /// let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
    /// let mut sbuf_reader = StreamBufReader::new(&buf);
    ///
    /// let mut data: [u8; 5] = [0; 5];
    /// let len = sbuf_reader.read(&mut data);
    ///
    /// assert_eq!(5, len);
    /// assert_eq!([0x0a, 0x1b, 0x2c, 0x3d, 0x4e], data);
    /// ```
    pub fn read(&mut self, dst: &mut [u8]) -> usize {
        let read_size = dst.len();
        if !self.is_remaining(read_size) {
            return 0;
        }
        dst.copy_from_slice(&self.buf[self.pos..self.pos + read_size]);
        self.pos += read_size;
        read_size
    }
}

/// Access `stream_buf` component by index.
impl Index<usize> for StreamBufReader<'_> {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 {
        &self.buf[index]
    }
}

#[cfg(any(debug_assertions, test))]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn new() {
        const BUF_SIZE: usize = 64;
        let mut data = [0u8; BUF_SIZE];
        let mut sbuf = StreamBufReader::new(&data);
    }

    #[test]
    fn stream_buf() {
        let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
        let buf_size: usize = buf.len();
        /*buf.fill(0xFF);
        buf[0] = 0x0a;
        buf[1] = 0x1b;
        buf[2] = 0x2c;
        buf[3] = 0x3d;
        buf[4] = 0x4e;
        buf[5] = 0x5f;
        buf[6] = 0x60;*/
        let mut sbuf_reader = StreamBufReader::new(&buf);

        assert_eq!(0, sbuf_reader.pos());
        assert_eq!(0, sbuf_reader.bytes_read());
        assert!(sbuf_reader.is_remaining(buf_size));
        assert_eq!(buf_size, sbuf_reader.bytes_remaining());

        let v1 = sbuf_reader.read_u8();
        assert_eq!(0x0a, v1);
        assert_eq!(1, sbuf_reader.bytes_read());
        assert_eq!(buf_size - 1, sbuf_reader.bytes_remaining());

        let v2 = sbuf_reader.read_u16();
        assert_eq!(0x2c1b, v2);
        assert_eq!(3, sbuf_reader.bytes_read());
        assert_eq!(buf_size - 3, sbuf_reader.bytes_remaining());

        let v3 = sbuf_reader.read_u32();
        assert_eq!(0x605f_4e3d, v3);
        assert_eq!(7, sbuf_reader.bytes_read());
        assert_eq!(buf_size - 7, sbuf_reader.bytes_remaining());

        sbuf_reader.reset();
        let mut data: [u8; 5] = [0; 5];
        let len = sbuf_reader.read(&mut data);
        assert_eq!(5, len);
        assert_eq!(0x0a, data[0]);
        assert_eq!(0x1b, data[1]);
        assert_eq!(0x2c, data[2]);
        assert_eq!(0x3d, data[3]);
        assert_eq!(0x4e, data[4]);
    }

    #[test]
    fn read_f32() {
        let buf = [0xec, 0x51, 0x9a, 0x44];
        let mut sbuf_reader = StreamBufReader::new(&buf);
        let v = sbuf_reader.read_f32();
        assert_eq!(1_234.56_f32, v);
    }

    #[test]
    fn read() {
        let buf = [0x0a, 0x1b, 0x2c, 0x3d, 0x4e, 0x5f, 0x60];
        let mut sbuf_reader = StreamBufReader::new(&buf);
        let mut data: [u8; 5] = [0; 5];
        let len = sbuf_reader.read(&mut data);
        assert_eq!(5, len);
        assert_eq!([0x0a, 0x1b, 0x2c, 0x3d, 0x4e], data);
    }
}
