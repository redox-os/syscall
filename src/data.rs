use core::{
    mem,
    ops::{Deref, DerefMut},
    slice,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::flag::{EventFlags, MapFlags, PtraceFlags, SigcontrolFlags};

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct Event {
    pub id: usize,
    pub flags: EventFlags,
    pub data: usize,
}

impl Deref for Event {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Event as *const u8, mem::size_of::<Event>()) }
    }
}

impl DerefMut for Event {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Event as *mut u8, mem::size_of::<Event>()) }
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
            slice::from_raw_parts(
                self as *const ITimerSpec as *const u8,
                mem::size_of::<ITimerSpec>(),
            )
        }
    }
}

impl DerefMut for ITimerSpec {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut ITimerSpec as *mut u8,
                mem::size_of::<ITimerSpec>(),
            )
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
        unsafe { slice::from_raw_parts(self as *const Map as *const u8, mem::size_of::<Map>()) }
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Map as *mut u8, mem::size_of::<Map>()) }
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
    pub d: usize,
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
        unsafe { slice::from_raw_parts(self as *const Stat as *const u8, mem::size_of::<Stat>()) }
    }
}

impl DerefMut for Stat {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Stat as *mut u8, mem::size_of::<Stat>()) }
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
            slice::from_raw_parts(
                self as *const StatVfs as *const u8,
                mem::size_of::<StatVfs>(),
            )
        }
    }
}

impl DerefMut for StatVfs {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut StatVfs as *mut u8, mem::size_of::<StatVfs>())
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
            slice::from_raw_parts(
                self as *const TimeSpec as *const u8,
                mem::size_of::<TimeSpec>(),
            )
        }
    }
}

impl DerefMut for TimeSpec {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut TimeSpec as *mut u8, mem::size_of::<TimeSpec>())
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
    pub f: usize,
}

impl Deref for PtraceEvent {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                self as *const PtraceEvent as *const u8,
                mem::size_of::<PtraceEvent>(),
            )
        }
    }
}

impl DerefMut for PtraceEvent {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut PtraceEvent as *mut u8,
                mem::size_of::<PtraceEvent>(),
            )
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
            slice::from_raw_parts(
                self as *const GrantDesc as *const u8,
                mem::size_of::<GrantDesc>(),
            )
        }
    }
}

impl DerefMut for GrantDesc {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe {
            slice::from_raw_parts_mut(
                self as *mut GrantDesc as *mut u8,
                mem::size_of::<GrantDesc>(),
            )
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct SetSighandlerData {
    pub user_handler: usize,
    pub excp_handler: usize,
    pub thread_control_addr: usize,
    pub proc_control_addr: usize,
}

impl Deref for SetSighandlerData {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self as *const Self as *const u8, mem::size_of::<Self>()) }
    }
}

impl DerefMut for SetSighandlerData {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self as *mut Self as *mut u8, mem::size_of::<Self>()) }
    }
}
/// Signal runtime struct for the entire process
#[derive(Debug, Default)]
#[repr(C)]
pub struct SigProcControl {
    // composed of [lo "pending" | lo "unmasked", hi "pending" | hi "unmasked"]
    pub word: [AtomicU64; 2],
}

/// Signal runtime struct for a thread
#[derive(Debug, Default)]
#[repr(C)]
pub struct Sigcontrol {
    // composed of [lo "pending" | lo "unmasked", hi "pending" | hi "unmasked"]
    pub word: [AtomicU64; 2],

    pub control_flags: SigatomicUsize,

    pub saved_ip: NonatomicUsize, // rip/eip/pc
    pub saved_archdep_reg: NonatomicUsize, // rflags/eflags/x0
}
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct SigatomicUsize(AtomicUsize);

impl SigatomicUsize {
    #[inline]
    pub fn load(&self, ordering: Ordering) -> usize {
        let value = self.0.load(Ordering::Relaxed);
        if ordering != Ordering::Relaxed {
            core::sync::atomic::compiler_fence(ordering);
        }
        value
    }
    #[inline]
    pub fn store(&self, value: usize, ordering: Ordering) {
        if ordering != Ordering::Relaxed {
            core::sync::atomic::compiler_fence(ordering);
        }
        self.0.store(value, Ordering::Relaxed);
    }
}
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct NonatomicUsize(AtomicUsize);

impl NonatomicUsize {
    #[inline]
    pub fn get(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
    #[inline]
    pub fn set(&self, value: usize) {
        self.0.store(value, Ordering::Relaxed);
    }
}

pub fn sig_bit(sig: usize) -> u64 {
    1 << (sig - 1)
}
impl SigProcControl {
    pub fn signal_will_stop(&self, sig: usize) -> bool {
        let bit = if sig == crate::SIGTSTP {
            crate::SIGW0_TSTP_IS_STOP_BIT
        } else if sig == crate::SIGTTIN {
            crate::SIGW0_TTIN_IS_STOP_BIT
        } else if sig == crate::SIGTTOU {
            crate::SIGW0_TTOU_IS_STOP_BIT
        } else {
            panic!()
        };

        self.word[0].load(Ordering::SeqCst) & bit == bit
    }
}

#[cfg(not(target_arch = "x86"))]
pub use core::sync::atomic::AtomicU64;

#[cfg(target_arch = "x86")]
pub use self::atomic::AtomicU64;

#[cfg(target_arch = "x86")]
mod atomic {
    use core::{cell::UnsafeCell, sync::atomic::Ordering};

    #[derive(Debug, Default)]
    pub struct AtomicU64(UnsafeCell<u64>);

    unsafe impl Send for AtomicU64 {}
    unsafe impl Sync for AtomicU64 {}

    impl AtomicU64 {
        pub const fn new(inner: u64) -> Self {
            Self(UnsafeCell::new(inner))
        }
        pub fn compare_exchange(
            &self,
            old: u64,
            new: u64,
            _success: Ordering,
            _failure: Ordering,
        ) -> Result<u64, u64> {
            let old_hi = (old >> 32) as u32;
            let old_lo = old as u32;
            let new_hi = (new >> 32) as u32;
            let new_lo = new as u32;
            let mut out_hi;
            let mut out_lo;

            unsafe {
                core::arch::asm!("lock cmpxchg8b [{}]", in(reg) self.0.get(), inout("edx") old_hi => out_hi, inout("eax") old_lo => out_lo, in("ecx") new_hi, in("ebx") new_lo);
            }

            if old_hi == out_hi && old_lo == out_lo {
                Ok(old)
            } else {
                Ok(u64::from(out_lo) | (u64::from(out_hi) << 32))
            }
        }
        pub fn load(&self, ordering: Ordering) -> u64 {
            match self.compare_exchange(0, 0, ordering, ordering) {
                Ok(new) => new,
                Err(new) => new,
            }
        }
        pub fn store(&self, new: u64, ordering: Ordering) {
            let mut old = 0;

            loop {
                match self.compare_exchange(old, new, ordering, Ordering::Relaxed) {
                    Ok(_) => break,
                    Err(new) => {
                        old = new;
                        core::hint::spin_loop();
                    }
                }
            }
        }
        pub fn fetch_update(
            &self,
            set_order: Ordering,
            fetch_order: Ordering,
            mut f: impl FnMut(u64) -> Option<u64>,
        ) -> Result<u64, u64> {
            let mut old = self.load(fetch_order);

            loop {
                let new = f(old).ok_or(old)?;
                match self.compare_exchange(old, new, set_order, Ordering::Relaxed) {
                    Ok(_) => return Ok(new),
                    Err(changed) => {
                        old = changed;
                        core::hint::spin_loop();
                    }
                }
            }
        }
        pub fn fetch_or(&self, bits: u64, order: Ordering) -> u64 {
            self.fetch_update(order, Ordering::Relaxed, |b| Some(b | bits))
                .unwrap()
        }
        pub fn fetch_and(&self, bits: u64, order: Ordering) -> u64 {
            self.fetch_update(order, Ordering::Relaxed, |b| Some(b & bits))
                .unwrap()
        }
        pub fn fetch_add(&self, term: u64, order: Ordering) -> u64 {
            self.fetch_update(order, Ordering::Relaxed, |b| Some(b.wrapping_add(term)))
                .unwrap()
        }
    }
}
