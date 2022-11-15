mod decoder;
use decoder::decode_block;
mod util;
use util::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst1() {
        let data = compress([0u32, 0, 0]);
        assert_eq!(data, [0, 0, 0, 0]);
    }

    #[test]
    fn tst2() {
        let data = compress([128, 0, 0]);
        assert_eq!(data, [0, 128, 0, 0]);
    }

    #[test]
    fn tst3() {
        let data = compress([255, 255, 255]);
        assert_eq!(data, [1 << 6, 0, 0, 0]);
    }

    #[test]
    fn tst4() {
        let data = compress([255 * 2, 255, 255]);
        assert_eq!(data, [1 << 6, 255, 0, 0]);
    }

    #[test]
    fn test_lots_of_numbers() {
        // create lots of input data, data can be grouped by three
        let mut data: Vec<u32> = (0..256).map(|i| i * i * i * i).collect();
        data.extend(0..256);
        data.extend((0..256).map(|i| i << 12));

        let cmpr = compress(data.iter().cloned());
        let result = decompress(&cmpr);

        assert_eq!(
            result, data,
            "expect data to be the same before and after compression/decompression"
        );
    }
}

struct DataBlockIter<'a> {
    data: &'a [u8],
}

impl<'a> DataBlockIter<'a> {
    fn to_vec(self) -> Vec<u32> {
        let mut v = Vec::new();

        for [a, b, c] in self {
            v.push(a);
            v.push(b);
            v.push(c);
        }

        v
    }
}

fn get_offset(index: u8) -> u32 {
    match index {
        0b00 => 0,
        0b01 => 0xff,
        0b10 => 0xffff,
        0b11 => 0xffffff,
        _ => panic!("expect number ranging from 0..=3"),
    }
}

impl<'a> Iterator for DataBlockIter<'a> {
    type Item = [u32; 3];

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        let v = self.data[0];
        let offset = get_offset(v >> 6);
        let data = &self.data[1..];

        let (mut a, mut b, mut c, bytes_consumed) = decode_block(v, data);

        a += offset;
        b += offset;
        c += offset;

        self.data = &data[bytes_consumed..];

        Some([a, b, c])
    }
}

pub fn decompress(data: &[u8]) -> Vec<u32> {
    DataBlockIter { data }.to_vec()
}

pub fn compress(iter: impl IntoIterator<Item = u32>) -> Vec<u8> {
    let mut buffer = Vec::new();
    let iter = iter.into_iter();
    for mut chunk in (Chunk { iter }) {
        // We append zeros instead of storing the length extra.
        while chunk.len() < 3 {
            chunk.push(0);
        }

        compress_block(&mut buffer, to_block(chunk));
    }

    buffer.shrink_to_fit();

    buffer
}

fn to_block(v: Vec<u32>) -> [u32; 3] {
    if v.len() != 3 {
        unreachable!("length of vector must be 3");
    }

    [v[0], v[1], v[2]]
}

/// Computes how many bytes can be stripped
/// from each numbers, without them under flowing
fn max_viable_offset(chunk: [u32; 3]) -> u8 {
    for i in [3, 2, 1] {
        let offset = get_offset(i);

        let [a, b, c] = chunk.map(|elem| elem >= offset);

        if a & b & c {
            return i;
        }
    }

    0
}

fn compress_block(buffer: &mut Vec<u8>, chunk: [u32; 3]) {
    // first, apply offset to chunk
    let offset_index = max_viable_offset(chunk);
    let offset = get_offset(offset_index);

    // subtract offset from number.
    let chunk = chunk.map(|elem| elem - offset);

    let mut mask = offset_index << 6; //bits0 | bits1 << 2 | bits2 << 4 | offset_index << 6;
    let maskidx = buffer.len();
    buffer.push(0);

    // loop over every integer in the chunk
    for i in 0..3u8 {
        let elem = chunk[i as usize];

        let bits = var_bits(elem);
        mask |= bits << (i << 1);

        // the first byte uses less instructions to encode.
        buffer.push((elem & 0xff) as u8);
        for byte_index in 1..=bits {
            let byte_index = byte_index * 8;
            let byte = (elem >> byte_index) & 0xff;
            buffer.push(byte as u8);
        }
    }

    // apply mask
    buffer[maskidx] = mask;
}

use smallvec::SmallVec;

/// Compressed list of u32 integers.
pub struct ListUInt32 {
    data: Vec<u8>,
    head: SmallVec<[u32; 3]>,
}

impl ListUInt32 {
    pub fn new() -> Self {
        ListUInt32 {
            data: Vec::new(),
            head: SmallVec::new(),
        }
    }

    pub fn push(&mut self, value: u32) {
        if self.head.len() == 2 {
            let chunk = [self.head[0], self.head[1], self.head[2]];
            compress_block(&mut self.data, chunk);
        } else {
            self.head.push(value);
        }
    }

    pub fn to_vec(&self) -> Vec<u32> {
        let i = DataBlockIter { data: &self.data };
        let mut v = i.to_vec();
        for i in &self.head {
            v.push(*i);
        }
        v
    }
}

impl Default for ListUInt32 {
    fn default() -> Self {
        Self::new()
    }
}
