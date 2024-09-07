use core::mem::size_of;
use core::ops::{Deref, DerefMut};
use core::slice;

use crate::error::{Error, Result, EINVAL};

#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
pub struct DirentHeader {
    pub inode: u64,
    // This struct intentionally not include a "next" offset field, unlike Linux, to guarantee the
    // iterator will be reasonably deterministic, even if the scheme is adversarial.
    pub record_len: u16,
    /// A `DirentKind`.
    ///
    /// May not be directly available (Unspecified), and if so needs to be looked using fstat.
    pub kind: u8,
}

impl Deref for DirentHeader {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }
}

impl DerefMut for DirentHeader {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    }
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(packed)]
pub struct DirentFooter {
    pub _rsvdz: u8,
}

impl Deref for DirentFooter {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }
}

impl DerefMut for DirentFooter {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(u8)]
pub enum DirentKind {
    #[default]
    Unspecified = 0,

    Regular = 1,
    Directory = 2,
    Symlink = 3,
    BlockDev = 4,
    CharDev = 5,
    Socket = 6,
}
impl DirentKind {
    // TODO: derive(FromPrimitive)
    pub fn try_from_raw(raw: u8) -> Option<Self> {
        Some(match raw {
            0 => Self::Unspecified,

            1 => Self::Regular,
            2 => Self::Directory,
            3 => Self::Symlink,
            4 => Self::BlockDev,
            5 => Self::CharDev,
            6 => Self::Socket,

            _ => return None,
        })
    }
}

pub struct DirentIter<'a>(&'a [u8]);

impl<'a> DirentIter<'a> {
    pub const fn new(buffer: &'a [u8]) -> Self {
        Self(buffer)
    }
}
#[derive(Debug)]
pub struct Invalid;

impl<'a> Iterator for DirentIter<'a> {
    type Item = Result<(&'a DirentHeader, &'a [u8], &'a DirentFooter), Invalid>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.len() < size_of::<DirentHeader>() {
            return None;
        }
        let header = unsafe { &*(self.0.as_ptr().cast::<DirentHeader>()) };
        if self.0.len() < usize::from(header.record_len) {
            return Some(Err(Invalid));
        }
        let (this, remaining) = self.0.split_at(usize::from(header.record_len));
        self.0 = remaining;

        let name_and_footer = &this[size_of::<DirentHeader>()..];
        let Some(off) = name_and_footer.len().checked_sub(size_of::<DirentFooter>()) else {
            return Some(Err(Invalid));
        };
        let (name, footer_bytes) = name_and_footer.split_at(off);
        let footer = unsafe { &*(footer_bytes.as_ptr().cast::<DirentFooter>()) };

        Some(Ok((header, name, footer)))
    }
}


pub struct DirentBuf<B> {
    buffer: B,
    written: usize,
}
/// Abstraction between &mut [u8] and the kernel's UserSliceWo.
pub trait Buffer<'a>: Sized + 'a {
    fn empty() -> Self;
    fn length(&self) -> usize;

    /// Split all of `self` into two disjoint contiguous subbuffers of lengths `index` and `length
    /// - index` respectively.
    ///
    /// Returns None if and only if `index > length`.
    fn split_at(self, index: usize) -> Option<[Self; 2]>;

    /// Copy from `src`, lengths must match exactly.
    fn copy_from_slice_exact(self, src: &[u8]) -> Result<()>;
}
impl<'a> Buffer<'a> for &'a mut [u8] {
    fn empty() -> Self {
        &mut []
    }
    fn length(&self) -> usize {
        self.len()
    }

    fn split_at(self, index: usize) -> Option<[Self; 2]> {
        let (a, b) = self.split_at_mut(index);
        Some([a, b])
    }
    fn copy_from_slice_exact(self, src: &[u8]) -> Result<()> {
        self.copy_from_slice(src);
        Ok(())
    }
}

impl<'a, B: Buffer<'a>> DirentBuf<B> {
    pub const fn new(buffer: B) -> Self {
        Self {
            buffer,
            written: 0,
        }
    }
    pub fn entry(&mut self, kind: DirentKind, name: &str) -> Result<()> {
        let name16 = u16::try_from(name.len()).map_err(|_| Error::new(EINVAL))?;
        let record_len = u16::try_from(size_of::<DirentHeader>() + size_of::<DirentFooter>()).unwrap().checked_add(name16).ok_or(Error::new(EINVAL))?;

        let [this, remaining] = core::mem::replace(&mut self.buffer, B::empty()).split_at(usize::from(record_len)).ok_or(Error::new(EINVAL))?;

        let [this_header, this_name_footer] = this.split_at(size_of::<DirentHeader>()).ok_or(Error::new(EINVAL))?;
        let name_footer_length = this_name_footer.length();
        let [this_name, this_footer] = this_name_footer.split_at(name_footer_length - size_of::<DirentFooter>()).ok_or(Error::new(EINVAL))?;
        this_header.copy_from_slice_exact(&DirentHeader {
            record_len,
            inode: 0,
            kind: kind as u8,
        })?;
        this_name.copy_from_slice_exact(name.as_bytes())?;
        this_footer.copy_from_slice_exact(&DirentFooter {
            _rsvdz: 0,
        })?;
        self.written += usize::from(record_len);
        self.buffer = remaining;

        Ok(())
    }
    pub fn finalize(self) -> usize {
        self.written
    }
}
