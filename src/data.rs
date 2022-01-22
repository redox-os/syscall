use core::ops::{Deref, DerefMut};
use core::{mem, slice};
use crate::flag::{EventFlags, MapFlags, PtraceFlags, SigActionFlags};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Event {
    pub id: usize,
    pub flags: EventFlags,
    pub data: usize
}

impl Deref for Event {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Event as *const u8, mem::size_of::<Event>())
        }
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Event as *mut u8, mem::size_of::<Event>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct ITimerSpec {
    pub it_interval: TimeSpec,
    pub it_value: TimeSpec,
}

impl Deref for ITimerSpec {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const ITimerSpec as *const u8,
                                  mem::size_of::<ITimerSpec>())
        }
    }
}

impl DerefMut for ITimerSpec {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut ITimerSpec as *mut u8,
                                      mem::size_of::<ITimerSpec>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct OldMap {
    pub offset: usize,
    pub size: usize,
    pub flags: MapFlags,
}

impl Deref for OldMap {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const OldMap as *const u8, mem::size_of::<OldMap>())
        }
    }
}

impl DerefMut for OldMap {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut OldMap as *mut u8, mem::size_of::<OldMap>())
        }
    }
}
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Map {
    /// The offset inside the file that is being mapped.
    pub offset: usize,

    /// The size of the memory map.
    pub size: usize,

    /// Contains both prot and map flags.
    pub flags: MapFlags,

    /// Functions as a hint to where in the virtual address space of the running process, to place
    /// the memory map. If [`MapFlags::MAP_FIXED`] is set, then this address must be the address to
    /// map to.
    pub address: usize,
}

impl Deref for Map {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Map as *const u8, mem::size_of::<Map>())
        }
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Map as *mut u8, mem::size_of::<Map>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Packet {
    pub id: u64,
    pub pid: usize,
    pub uid: u32,
    pub gid: u32,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize
}

impl Deref for Packet {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Packet as *const u8, mem::size_of::<Packet>())
        }
    }
}

impl DerefMut for Packet {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Packet as *mut u8, mem::size_of::<Packet>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct SigAction {
    pub sa_handler: Option<extern "C" fn(usize)>,
    pub sa_mask: [u64; 2],
    pub sa_flags: SigActionFlags,
}

#[allow(dead_code)]
unsafe fn _assert_size_of_function_is_sane() {
    // Transmuting will complain *at compile time* if sizes differ.
    // Rust forbids a fn-pointer from being 0 so to allow SIG_DFL to
    // exist, we use Option<extern "C" fn(usize)> which will mean 0
    // becomes None
    let _ = mem::transmute::<Option<extern "C" fn(usize)>, usize>(None);
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_mode: u16,
    pub st_nlink: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub st_size: u64,
    pub st_blksize: u32,
    pub st_blocks: u64,
    pub st_mtime: u64,
    pub st_mtime_nsec: u32,
    pub st_atime: u64,
    pub st_atime_nsec: u32,
    pub st_ctime: u64,
    pub st_ctime_nsec: u32,
}

impl Deref for Stat {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Stat as *const u8,
                                  mem::size_of::<Stat>())
        }
    }
}

impl DerefMut for Stat {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Stat as *mut u8,
                                      mem::size_of::<Stat>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct StatVfs {
    pub f_bsize: u32,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
}

impl Deref for StatVfs {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const StatVfs as *const u8,
                                  mem::size_of::<StatVfs>())
        }
    }
}

impl DerefMut for StatVfs {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut StatVfs as *mut u8,
                                      mem::size_of::<StatVfs>())
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[repr(C)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

impl Deref for TimeSpec {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const TimeSpec as *const u8,
                                  mem::size_of::<TimeSpec>())
        }
    }
}

impl DerefMut for TimeSpec {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut TimeSpec as *mut u8,
                                      mem::size_of::<TimeSpec>())
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct PtraceEvent {
    pub cause: PtraceFlags,
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub d: usize,
    pub e: usize,
    pub f: usize
}

impl Deref for PtraceEvent {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const PtraceEvent as *const u8, mem::size_of::<PtraceEvent>())
        }
    }
}

impl DerefMut for PtraceEvent {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut PtraceEvent as *mut u8, mem::size_of::<PtraceEvent>())
        }
    }
}

#[macro_export]
macro_rules! ptrace_event {
    ($cause:expr $(, $a:expr $(, $b:expr $(, $c:expr)?)?)?) => {
        $crate::data::PtraceEvent {
            cause: $cause,
            $(a: $a,
              $(b: $b,
                $(c: $c,)?
              )?
            )?
            ..Default::default()
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ExecMemRange {
    /// The address where the range is intended to be.
    pub address: usize,
    /// The size of the memory range.
    pub size: usize,
    /// Flags describing permissions (i.e. R/W/X)
    pub flags: usize,
    /// If this equals [`address`], the range is kept untouched although flags can change, and the
    /// range can be shortened or partly zeroed. If it is different, it will move `[old_address,
    /// old_address+size)` to `[address, address+size]`.
    pub old_address: usize,
}
impl Deref for ExecMemRange {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const ExecMemRange as *const u8, mem::size_of::<ExecMemRange>())
        }
    }
}

impl DerefMut for ExecMemRange {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut ExecMemRange as *mut u8, mem::size_of::<ExecMemRange>())
        }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CloneInfo  {
    /// The newly allocated stack to use for the child process when [`CLONE_VM`] is set. Otherwise
    /// it is ignored.
    pub target_stack: usize,
    /// The newly allocated signal stack, when [`CLONE_VM`] is set, otherwise ignored. If no signal
    /// stack is desired (i.e. all sigactions are SIG_DFL), then this may be set to `usize::MAX`.
    pub target_sigstack: usize,
}
impl Deref for CloneInfo {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
        }
    }
}

impl DerefMut for CloneInfo {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of::<Self>())
        }
    }
}
