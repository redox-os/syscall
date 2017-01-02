//! Constants used to define and invoke syscalls.

// Implementation notes: values < 500 are reserved for standard Unix syscalls.
// Redox-specific syscalls should use values >= 500.

// --- Flags

pub const SYS_CLASS: usize =    0xF000_0000;
/// Flag: This syscall takes a path as argument.
pub const SYS_CLASS_PATH: usize=0x1000_0000;
/// Flag: This syscall takes a file descriptor as argument.
pub const SYS_CLASS_FILE: usize=0x2000_0000;

pub const SYS_ARG: usize =      0x0F00_0000;
pub const SYS_ARG_SLICE: usize =0x0100_0000;
pub const SYS_ARG_MSLICE: usize=0x0200_0000;
pub const SYS_ARG_PATH: usize = 0x0300_0000;

pub const SYS_RET: usize =      0x00F0_0000;
/// Flag: This syscall returns a file descriptor.
pub const SYS_RET_FILE: usize = 0x0010_0000;

/// --- Syscalls

pub const SYS_LINK: usize =     SYS_CLASS_PATH | SYS_ARG_PATH | 9;
pub const SYS_OPEN: usize =     SYS_CLASS_PATH | SYS_RET_FILE | 5;
pub const SYS_CHMOD: usize =    SYS_CLASS_PATH | 15;
pub const SYS_RMDIR: usize =    SYS_CLASS_PATH | 84;
pub const SYS_UNLINK: usize =   SYS_CLASS_PATH | 10;
/// Syscall: Bind a file descriptor exposed by a process to a local file descriptor.
///
/// The file descriptor must already have been exposed through a call to `SYS_DUP_EXPORT`..
///
/// Prototype: `sys_dup_from(pid, key: [u8]) -> fd`
pub const SYS_DUP_FROM: usize = SYS_CLASS_PATH | 501;

pub const SYS_CLOSE: usize =    SYS_CLASS_FILE | 6;
pub const SYS_DUP: usize =      SYS_CLASS_FILE | SYS_RET_FILE | 41;
pub const SYS_DUP2: usize =      SYS_CLASS_FILE | SYS_RET_FILE | 63;
/// Syscall: Expose a file descriptor to a process.
///
/// Once this syscall has been executed, the recipient process may use
/// syscall `SYS_DUP_FROM` to bind the exposed file descriptor to a
/// (new) local file descriptor. The scheme itself receives
/// call `dup()` with a hint of `HINT_DUP_EXPORT`.
///
/// Prototype: `sys_dup_export(pid, fd, key: [u8])`
pub const SYS_DUP_EXPORT: usize =   SYS_CLASS_FILE | 502;
pub const SYS_READ: usize =     SYS_CLASS_FILE | SYS_ARG_MSLICE | 3;
pub const SYS_WRITE: usize =    SYS_CLASS_FILE | SYS_ARG_SLICE | 4;
pub const SYS_LSEEK: usize =    SYS_CLASS_FILE | 19;
pub const SYS_FCNTL: usize =    SYS_CLASS_FILE | 55;
pub const SYS_FEVENT: usize =   SYS_CLASS_FILE | 927;
pub const SYS_FMAP: usize =     SYS_CLASS_FILE | 90;
pub const SYS_FUNMAP: usize =   SYS_CLASS_FILE | 91;
pub const SYS_FPATH: usize =    SYS_CLASS_FILE | SYS_ARG_MSLICE | 928;
pub const SYS_FSTAT: usize =    SYS_CLASS_FILE | SYS_ARG_MSLICE | 28;
pub const SYS_FSTATVFS: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 100;
pub const SYS_FSYNC: usize =    SYS_CLASS_FILE | 118;
pub const SYS_FTRUNCATE: usize =SYS_CLASS_FILE | 93;

pub const SYS_BRK: usize =      45;
pub const SYS_CHDIR: usize =    12;
pub const SYS_CLOCK_GETTIME: usize = 265;
pub const SYS_CLONE: usize =    120;
pub const SYS_EXECVE: usize =   11;
pub const SYS_EXIT: usize =     1;
pub const SYS_FUTEX: usize =    240;
pub const SYS_GETCWD: usize =   183;
pub const SYS_GETEGID: usize =  202;
pub const SYS_GETENS: usize =   951;
pub const SYS_GETEUID: usize =  201;
pub const SYS_GETGID: usize =   200;
pub const SYS_GETNS: usize =    950;
pub const SYS_GETPID: usize =   20;
pub const SYS_GETUID: usize =   199;
pub const SYS_IOPL: usize =     110;
pub const SYS_KILL: usize =     37;
pub const SYS_MKNS: usize =     984;
pub const SYS_NANOSLEEP: usize =162;
pub const SYS_PHYSALLOC: usize =945;
pub const SYS_PHYSFREE: usize = 946;
pub const SYS_PHYSMAP: usize =  947;
pub const SYS_PHYSUNMAP: usize =948;
pub const SYS_VIRTTOPHYS: usize=949;
pub const SYS_PIPE2: usize =    331;
pub const SYS_SETREGID: usize = 204;
pub const SYS_SETRENS: usize =  952;
pub const SYS_SETREUID: usize = 203;
pub const SYS_WAITPID: usize =  7;
pub const SYS_YIELD: usize =    158;

// --- Hints passed to `dup`.

/// Hint: The duplication took place because of a call to `exec`.
pub const HINT_DUP_EXEC: &'static [u8] = b"exec";

/// Hint: The duplication took place because of a call to `clone`.
pub const HINT_DUP_CLONE: &'static [u8] = b"clone";

/// Hint: The duplication took place because of a call to `listen`.
pub const HINT_DUP_LISTEN: &'static [u8] = b"listen";

/// Hint: The duplication took place because of a call to `dup_export`.
pub const HINT_DUP_EXPORT: &'static [u8] = b"export";
