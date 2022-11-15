pub(crate) struct Chunk<I>
where
    I: Iterator<Item = u32>,
{
    pub iter: I,
}

impl<I> Iterator for Chunk<I>
where
    I: Iterator<Item = u32>,
{
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.iter.next();
        let b = self.iter.next();
        let c = self.iter.next();

        match (a, b, c) {
            (None, _, _) => None,
            (Some(a), None, _) => Some(vec![a, 0, 0]),
            (Some(a), Some(b), None) => Some(vec![a, b, 0]),
            (Some(a), Some(b), Some(c)) => Some(vec![a, b, c]),
        }
    }
}

/// derives the byte pattern for encoding vaiable length
pub(crate) fn var_bits(v: u32) -> u8 {
    if v <= 0xffff {
        if v <= 0xff {
            0
        } else {
            1
        }
    } else if v <= 0xffffff {
        2
    } else {
        3
    }
}
