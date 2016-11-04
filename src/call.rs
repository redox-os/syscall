use super::arch::*;
use super::data::{Stat, TimeSpec};
use super::error::Result;
use super::number::*;

use core::mem;

/// Set the end of the process's heap
///
/// When `addr` is `0`, this function will return the current break.
///
/// When `addr` is nonzero, this function will attempt to set the end of the process's
/// heap to `addr` and return the new program break. The new program break should be
/// checked by the allocator, it may not be exactly `addr`, as it may be aligned to a page
/// boundary.
///
/// On error, `Err(ENOMEM)` will be returned indicating that no memory is available
pub unsafe fn brk(addr: usize) -> Result<usize> {
    syscall1(SYS_BRK, addr)
}

/// Change the process's working directory
///
/// This function will attempt to set the process's working directory to `path`, which can be
/// either a relative, scheme relative, or absolute path.
///
/// On success, `Ok(0)` will be returned. On error, one of the following errors will be returned.
///
/// # Errors
///
/// * `EACCES` - permission is denied for one of the components of `path`, or `path`
/// * `EFAULT` - `path` does not point to the process's addressible memory
/// * `EIO` - an I/O error occured
/// * `ENOENT` - `path` does not exit
/// * `ENOTDIR` - `path` is not a directory
pub fn chdir(path: &str) -> Result<usize> {
    unsafe { syscall2(SYS_CHDIR, path.as_ptr() as usize, path.len()) }
}

pub unsafe fn clone(flags: usize) -> Result<usize> {
    syscall1_clobber(SYS_CLONE, flags)
}

pub fn close(fd: usize) -> Result<usize> {
    unsafe { syscall1(SYS_CLOSE, fd) }
}

pub fn clock_gettime(clock: usize, tp: &mut TimeSpec) -> Result<usize> {
    unsafe { syscall2(SYS_CLOCK_GETTIME, clock, tp as *mut TimeSpec as usize) }
}

pub fn dup(fd: usize, buf: &[u8]) -> Result<usize> {
    unsafe { syscall3(SYS_DUP, fd, buf.as_ptr() as usize, buf.len()) }
}

pub fn execve(path: &str, args: &[[usize; 2]]) -> Result<usize> {
    unsafe { syscall4(SYS_EXECVE, path.as_ptr() as usize, path.len(), args.as_ptr() as usize, args.len()) }
}

pub fn exit(status: usize) -> Result<usize> {
    unsafe { syscall1(SYS_EXIT, status) }
}

pub fn fevent(fd: usize, flags: usize) -> Result<usize> {
    unsafe { syscall2(SYS_FEVENT, fd, flags) }
}

pub unsafe fn fmap(fd: usize, offset: usize, size: usize) -> Result<usize> {
    syscall3(SYS_FMAP, fd, offset, size)
}

pub unsafe fn funmap(addr: usize) -> Result<usize> {
    syscall1(SYS_FUNMAP, addr)
}

pub fn fpath(fd: usize, buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall3(SYS_FPATH, fd, buf.as_mut_ptr() as usize, buf.len()) }
}

pub fn fstat(fd: usize, stat: &mut Stat) -> Result<usize> {
    unsafe { syscall3(SYS_FSTAT, fd, stat as *mut Stat as usize, mem::size_of::<Stat>()) }
}

pub fn fsync(fd: usize) -> Result<usize> {
    unsafe { syscall1(SYS_FSYNC, fd) }
}

pub fn ftruncate(fd: usize, len: usize) -> Result<usize> {
    unsafe { syscall2(SYS_FTRUNCATE, fd, len) }
}

pub unsafe fn futex(addr: *mut i32, op: usize, val: i32, val2: usize, addr2: *mut i32) -> Result<usize> {
    syscall5(SYS_FUTEX, addr as usize, op, (val as isize) as usize, val2, addr2 as usize)
}

pub fn getcwd(buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall2(SYS_GETCWD, buf.as_mut_ptr() as usize, buf.len()) }
}

pub fn getegid() -> Result<usize> {
    unsafe { syscall0(SYS_GETEGID) }
}

pub fn geteuid() -> Result<usize> {
    unsafe { syscall0(SYS_GETEUID) }
}

pub fn getgid() -> Result<usize> {
    unsafe { syscall0(SYS_GETGID) }
}

pub fn getpid() -> Result<usize> {
    unsafe { syscall0(SYS_GETPID) }
}

pub fn getuid() -> Result<usize> {
    unsafe { syscall0(SYS_GETUID) }
}

pub unsafe fn iopl(level: usize) -> Result<usize> {
    syscall1(SYS_IOPL, level)
}

pub fn kill(pid: usize, sig: usize) -> Result<usize> {
    unsafe { syscall2(SYS_KILL, pid, sig) }
}

pub unsafe fn link(old: *const u8, new: *const u8) -> Result<usize> {
    syscall2(SYS_LINK, old as usize, new as usize)
}

pub fn lseek(fd: usize, offset: isize, whence: usize) -> Result<usize> {
    unsafe { syscall3(SYS_LSEEK, fd, offset as usize, whence) }
}

pub fn mkdir(path: &str, mode: u16) -> Result<usize> {
    unsafe { syscall3(SYS_MKDIR, path.as_ptr() as usize, path.len(), mode as usize) }
}

pub fn nanosleep(req: &TimeSpec, rem: &mut TimeSpec) -> Result<usize> {
    unsafe { syscall2(SYS_NANOSLEEP, req as *const TimeSpec as usize, rem as *mut TimeSpec as usize) }
}

pub fn open(path: &str, flags: usize) -> Result<usize> {
    unsafe { syscall3(SYS_OPEN, path.as_ptr() as usize, path.len(), flags) }
}

pub unsafe fn physalloc(size: usize) -> Result<usize> {
    syscall1(SYS_PHYSALLOC, size)
}

pub unsafe fn physfree(physical_address: usize, size: usize) -> Result<usize> {
    syscall2(SYS_PHYSFREE, physical_address, size)
}

pub unsafe fn physmap(physical_address: usize, size: usize, flags: usize) -> Result<usize> {
    syscall3(SYS_PHYSMAP, physical_address, size, flags)
}

pub unsafe fn physunmap(virtual_address: usize) -> Result<usize> {
    syscall1(SYS_PHYSUNMAP, virtual_address)
}

pub fn pipe2(fds: &mut [usize; 2], flags: usize) -> Result<usize> {
    unsafe { syscall2(SYS_PIPE2, fds.as_ptr() as usize, flags) }
}

pub fn read(fd: usize, buf: &mut [u8]) -> Result<usize> {
    unsafe { syscall3(SYS_READ, fd, buf.as_mut_ptr() as usize, buf.len()) }
}

pub fn rmdir(path: &str) -> Result<usize> {
    unsafe { syscall2(SYS_RMDIR, path.as_ptr() as usize, path.len()) }
}

pub fn setgid(gid: usize) -> Result<usize> {
    unsafe { syscall1(SYS_SETGID, gid) }
}

pub fn setuid(uid: usize) -> Result<usize> {
    unsafe { syscall1(SYS_SETUID, uid) }
}

pub fn unlink(path: &str) -> Result<usize> {
    unsafe { syscall2(SYS_UNLINK, path.as_ptr() as usize, path.len()) }
}

pub unsafe fn virttophys(virtual_address: usize) -> Result<usize> {
    syscall1(SYS_VIRTTOPHYS, virtual_address)
}

/// Check if a child process has exited or received a signal
///
/// The kernel
pub fn waitpid(pid: usize, status: &mut usize, options: usize) -> Result<usize> {
    unsafe { syscall3(SYS_WAITPID, pid, status as *mut usize as usize, options) }
}

/// Write a buffer to a file descriptor
///
/// The kernel will attempt to write the bytes in `buf` to the file descriptor `fd`, returning
/// either an `Err`, explained below, or `Ok(count)` where `count` is the number of bytes which
/// were written.
///
/// # Errors
///
/// * `EAGAIN` - the file descriptor was opened with `O_NONBLOCK` and writing would block
/// * `EBADF` - the file descriptor is not valid or is not open for writing
/// * `EFAULT` - `buf` does not point to the process's addressible memory
/// * `EIO` - an I/O error occured
/// * `ENOSPC` - the device containing the file descriptor has no room for data
/// * `EPIPE` - the file descriptor refers to a pipe or socket whose reading end is closed
pub fn write(fd: usize, buf: &[u8]) -> Result<usize> {
    unsafe { syscall3(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len()) }
}

/// Yield the process's time slice to the kernel
///
/// This function will return Ok(0) on success
pub fn sched_yield() -> Result<usize> {
    unsafe { syscall0(SYS_YIELD) }
}
