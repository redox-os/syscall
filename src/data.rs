use core::ops::{Deref, DerefMut};
use core::{mem, slice};
use crate::IntRegisters;
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
    pub sa_mask: u64,
    pub sa_flags: SigActionFlags,
}
impl Deref for SigAction {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const SigAction as *const u8,
                                  mem::size_of::<SigAction>())
        }
    }
}

impl DerefMut for SigAction {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut SigAction as *mut u8,
                                      mem::size_of::<SigAction>())
        }
    }
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

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    pub struct GrantFlags: usize {
        const GRANT_READ = 0x0000_0001;
        const GRANT_WRITE = 0x0000_0002;
        const GRANT_EXEC = 0x0000_0004;

        const GRANT_SHARED = 0x0000_0008;
        const GRANT_LAZY = 0x0000_0010;
        const GRANT_SCHEME = 0x0000_0020;
        const GRANT_PHYS = 0x0000_0040;
        const GRANT_PINNED = 0x0000_0080;
        const GRANT_PHYS_CONTIGUOUS = 0x0000_0100;
    }
}

impl GrantFlags {
    #[deprecated = "use the safe `from_bits_retain` method instead"]
    pub unsafe fn from_bits_unchecked(bits: usize) -> Self {
        Self::from_bits_retain(bits)
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct GrantDesc {
    pub base: usize,
    pub size: usize,
    pub flags: GrantFlags,
    pub offset: u64,
}

impl Deref for GrantDesc {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const GrantDesc as *const u8, mem::size_of::<GrantDesc>())
        }
    }
}

impl DerefMut for GrantDesc {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut GrantDesc as *mut u8, mem::size_of::<GrantDesc>())
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(C, align(64))]
pub struct SignalStack {
    pub intregs: IntRegisters,
    pub old_procmask: u64,
    pub sa_mask: u64,
    pub sa_flags: u32,
    pub sig_num: u32,
    pub sa_handler: usize,
    // offset = 3*64 bytes from this point.
    //
    // NOTE: If any new fields are added, make sure 64 byte alignment is maintained (for x86_64
    // XSAVE, other arches may not necessarily need that alignment).
}

impl Deref for SignalStack {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
        }
    }
}

impl DerefMut for SignalStack {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of::<Self>())
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SetSighandlerData {
    pub entry: usize,
    pub altstack_base: usize,
    pub altstack_len: usize,
}

impl Deref for SetSighandlerData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>())
        }
    }
}

impl DerefMut for SetSighandlerData {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of::<Self>())
        }
    }
}
