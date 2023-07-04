pub struct AllocatedRegion {
    start: Option<usize>,
    end: Option<usize>,
    length: Option<usize>,
}

impl AllocatedRegion {
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            start: Some(start),
            end: Some(start + size - 1),
            length: Some(size),
        }
    }

    pub const fn new_uninit() -> Self {
        Self {
            start: None,
            end: None,
            length: None,
        }
    }

    pub const fn init(&self) -> bool {
        self.start.is_some() && self.end.is_some() && self.length.is_some()
    }

    pub fn size(&self) -> usize {
        self.length.unwrap()
    }

    pub fn start(&self) -> usize {
        self.start.unwrap()
    }

    pub fn end(&self) -> usize {
        self.end.unwrap()
    }

    pub fn contains(&self, val: usize) -> bool {
        val >= self.start() && val <= self.end()
    }
}
