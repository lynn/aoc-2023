use std::marker::PhantomData;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T> {
    bytes: Vec<u8>,
    width: usize,
    height: usize,
    marker: PhantomData<T>,
}

impl<T> Grid<T> {
    pub fn parse(string: &str) -> Grid<T> {
        let bytes = string.as_bytes().to_owned();
        let width = string.find('\n').unwrap_or(string.len());
        let n = string.len();
        let height = (n + 1) / (width + 1);
        Grid {
            bytes,
            width,
            height,
            marker: PhantomData::default(),
        }
    }

    #[allow(dead_code)]
    pub fn string(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn row(&self, y: usize) -> &[u8] {
        let start = y * (self.width + 1);
        &self.bytes[start..start + self.width]
    }

    fn to_xy(&self, index: usize) -> (i64, i64) {
        let x = (index % (self.width + 1)) as i64;
        let y = (index / (self.width + 1)) as i64;
        (x, y)
    }

    fn to_index(&self, x: i64, y: i64) -> usize {
        y as usize * (self.width + 1) + x as usize
    }

    pub fn byte_position(&self, byte: u8) -> Option<(i64, i64)> {
        self.bytes
            .iter()
            .position(|&b| b == byte)
            .map(|i| self.to_xy(i))
    }

    pub fn byte_positions(&self, byte: u8) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.bytes
            .iter()
            .enumerate()
            .filter(move |t| *t.1 == byte)
            .map(|t| self.to_xy(t.0))
    }

    pub fn get(&self, x: i64, y: i64) -> T
    where
        T: From<u8>,
    {
        self.bytes[self.to_index(x, y)].into()
    }

    pub fn set(&mut self, x: i64, y: i64, b: u8) {
        let i = self.to_index(x, y);
        self.bytes[i] = b;
    }

    pub fn in_range(&self, x: i64, y: i64) -> bool {
        x >= 0 && x < self.width as i64 && y >= 0 && y < self.height as i64
    }
}
