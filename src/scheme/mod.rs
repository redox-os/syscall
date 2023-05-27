use core::{slice, str};

use crate::Result;

pub use self::scheme::Scheme;
pub use self::scheme_mut::SchemeMut;
pub use self::scheme_block::SchemeBlock;
pub use self::scheme_block_mut::SchemeBlockMut;
pub use self::seek::*;

unsafe fn str_from_raw_parts(ptr: *const u8, len: usize) -> Option<&'static str> {
    let slice = slice::from_raw_parts(ptr, len);
    str::from_utf8(slice).ok()
}

mod scheme;
mod scheme_mut;
mod scheme_block;
mod scheme_block_mut;
mod seek;

pub struct CallerCtx {
    pub pid: usize,
    pub uid: u32,
    pub gid: u32,
}

pub enum OpenResult {
    ThisScheme { number: usize },
    OtherScheme { fd: usize },
}

pub(crate) fn convert_to_this_scheme(r: Result<usize>) -> Result<OpenResult> {
    r.map(|number| OpenResult::ThisScheme { number })
}
pub(crate) fn convert_to_this_scheme_block(r: Result<Option<usize>>) -> Result<Option<OpenResult>> {
    r.map(|o| o.map(|number| OpenResult::ThisScheme { number }))
}
