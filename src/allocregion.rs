use core::mem::MaybeUninit;

// NOTE: Option<usize> can be used too, however MaybeUninit is zero-sized.
//       Size of AllocatedRegion with Option<usize>      = 48
//       Size of AllocatedRegion with MaybeUninit<usize> = 32 (incl. init)
//       To ensure that the allocator has the least amount of overhead, MaybeUninit is used.
//       Another benefit of using MaybeUninit is that all methods can be const.
pub struct AllocatedRegion {
    pub init: bool,
    start: MaybeUninit<usize>,
    end: MaybeUninit<usize>,
    length: MaybeUninit<usize>,
}

impl AllocatedRegion {
    pub const fn new(start: usize, size: usize) -> Self {
        Self {
            init: true,
            start: MaybeUninit::new(start),
            end: MaybeUninit::new(start + size - 1),
            length: MaybeUninit::new(size),
        }
    }

    pub const fn new_uninit() -> Self {
        Self {
            init: false,
            start: MaybeUninit::uninit(),
            end: MaybeUninit::uninit(),
            length: MaybeUninit::uninit(),
        }
    }

    pub const fn size(&self) -> usize {
        if !self.init {
            return 0;
        }

        // SAFETY: We just checked if the value is initialized
        unsafe { self.length.assume_init() }
    }

    pub const fn start(&self) -> usize {
        if !self.init {
            return 0;
        }

        // SAFETY: We just checked if the value is initialized
        unsafe { self.start.assume_init() }
    }

    pub const fn end(&self) -> usize {
        if !self.init {
            return 0;
        }

        // SAFETY: We just checked if the value is initialized
        unsafe { self.end.assume_init() }
    }

    pub const fn contains(&self, val: usize) -> bool {
        if !self.init {
            return false;
        }

        val >= self.start() && val <= self.end()
    }
}
