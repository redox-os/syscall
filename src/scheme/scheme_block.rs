use core::{mem, slice};

use data::*;
use error::*;
use number::*;

pub trait SchemeBlock {
    fn handle(&self, packet: &Packet) -> Option<usize> {
        let res = match packet.a {
            SYS_OPEN => self.open(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) }, packet.d, packet.uid, packet.gid),
            SYS_CHMOD => self.chmod(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) }, packet.d as u16, packet.uid, packet.gid),
            SYS_RMDIR => self.rmdir(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) }, packet.uid, packet.gid),
            SYS_UNLINK => self.unlink(unsafe { slice::from_raw_parts(packet.b as *const u8, packet.c) }, packet.uid, packet.gid),

            SYS_DUP => self.dup(packet.b, unsafe { slice::from_raw_parts(packet.c as *const u8, packet.d) }),
            SYS_READ => self.read(packet.b, unsafe { slice::from_raw_parts_mut(packet.c as *mut u8, packet.d) }),
            SYS_WRITE => self.write(packet.b, unsafe { slice::from_raw_parts(packet.c as *const u8, packet.d) }),
            SYS_LSEEK => self.seek(packet.b, packet.c, packet.d),
            SYS_FCHMOD => self.fchmod(packet.b, packet.c as u16),
            SYS_FCHOWN => self.fchown(packet.b, packet.c as u32, packet.d as u32),
            SYS_FCNTL => self.fcntl(packet.b, packet.c, packet.d),
            SYS_FEVENT => self.fevent(packet.b, packet.c),
            SYS_FMAP => self.fmap(packet.b, packet.c, packet.d),
            SYS_FPATH => self.fpath(packet.b, unsafe { slice::from_raw_parts_mut(packet.c as *mut u8, packet.d) }),
            SYS_FRENAME => self.frename(packet.b, unsafe { slice::from_raw_parts(packet.c as *const u8, packet.d) }, packet.uid, packet.gid),
            SYS_FSTAT => if packet.d >= mem::size_of::<Stat>() {
                self.fstat(packet.b, unsafe { &mut *(packet.c as *mut Stat) })
            } else {
                Some(Err(Error::new(EFAULT)))
            },
            SYS_FSTATVFS => if packet.d >= mem::size_of::<StatVfs>() {
                self.fstatvfs(packet.b, unsafe { &mut *(packet.c as *mut StatVfs) })
            } else {
                Some(Err(Error::new(EFAULT)))
            },
            SYS_FSYNC => self.fsync(packet.b),
            SYS_FTRUNCATE => self.ftruncate(packet.b, packet.c),
            SYS_FUTIMENS => if packet.d >= mem::size_of::<TimeSpec>() {
                self.futimens(packet.b, unsafe { slice::from_raw_parts(packet.c as *const TimeSpec, packet.d / mem::size_of::<TimeSpec>()) })
            } else {
                Some(Err(Error::new(EFAULT)))
            },
            SYS_CLOSE => self.close(packet.b),
            _ => Some(Err(Error::new(ENOSYS)))
        };

        res.map(Error::mux)
    }

    /* Scheme operations */

    #[allow(unused_variables)]
    fn open(&self, path: &[u8], flags: usize, uid: u32, gid: u32) -> Option<Result<usize>> {
        Some(Err(Error::new(ENOENT)))
    }

    #[allow(unused_variables)]
    fn chmod(&self, path: &[u8], mode: u16, uid: u32, gid: u32) -> Option<Result<usize>> {
        Some(Err(Error::new(ENOENT)))
    }

    #[allow(unused_variables)]
    fn rmdir(&self, path: &[u8], uid: u32, gid: u32) -> Option<Result<usize>> {
        Some(Err(Error::new(ENOENT)))
    }

    #[allow(unused_variables)]
    fn unlink(&self, path: &[u8], uid: u32, gid: u32) -> Option<Result<usize>> {
        Some(Err(Error::new(ENOENT)))
    }

    /* Resource operations */
    #[allow(unused_variables)]
    fn dup(&self, old_id: usize, buf: &[u8]) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn read(&self, id: usize, buf: &mut [u8]) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn write(&self, id: usize, buf: &[u8]) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn seek(&self, id: usize, pos: usize, whence: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fchmod(&self, id: usize, mode: u16) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fchown(&self, id: usize, uid: u32, gid: u32) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fcntl(&self, id: usize, cmd: usize, arg: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fevent(&self, id: usize, flags: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fmap(&self, id: usize, offset: usize, size: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fpath(&self, id: usize, buf: &mut [u8]) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn frename(&self, id: usize, path: &[u8], uid: u32, gid: u32) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fstat(&self, id: usize, stat: &mut Stat) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fstatvfs(&self, id: usize, stat: &mut StatVfs) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn fsync(&self, id: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn ftruncate(&self, id: usize, len: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn futimens(&self, id: usize, times: &[TimeSpec]) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }

    #[allow(unused_variables)]
    fn close(&self, id: usize) -> Option<Result<usize>> {
        Some(Err(Error::new(EBADF)))
    }
}
