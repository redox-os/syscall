//TODO: close

#[test]
fn clock_gettime() {
    let mut tp = crate::TimeSpec::default();
    assert_eq!(dbg!(
        crate::clock_gettime(crate::CLOCK_MONOTONIC, &mut tp)
    ), Ok(0));
    assert_ne!(dbg!(tp), crate::TimeSpec::default());

    tp = crate::TimeSpec::default();
    assert_eq!(dbg!(
        crate::clock_gettime(crate::CLOCK_REALTIME, &mut tp)
    ), Ok(0));
    assert_ne!(dbg!(tp), crate::TimeSpec::default());
}

//TODO: dup

//TODO: dup2

//TODO: exit (handled by clone?)

//TODO: fchmod

//TODO: fcntl

#[test]
fn fmap() {
    use std::slice;

    let fd = dbg!(
        crate::open(
            "file:/tmp/syscall-tests-fmap",
            crate::O_CREAT | crate::O_RDWR | crate::O_CLOEXEC
        )
    ).unwrap();

    let size = 128;

    let map = unsafe {
        slice::from_raw_parts_mut(
            dbg!(
                crate::fmap(fd, &crate::Map {
                    address: 0,
                    offset: 0,
                    size,
                    flags: crate::PROT_READ | crate::PROT_WRITE
                })
            ).unwrap() as *mut u8,
            128
        )
    };

    // Maps should be available after closing
    assert_eq!(dbg!(crate::close(fd)), Ok(0));

    for i in 0..128 {
        map[i as usize] = i;
        assert_eq!(map[i as usize], i);
    }

    //TODO: add msync
    unsafe {
        assert_eq!(dbg!(
            crate::funmap(map.as_mut_ptr() as usize, size)
        ), Ok(0));
    }
}

// funmap tested by fmap

#[test]
fn fpath() {
    use std::str;

    let path = "file:/tmp/syscall-tests-fpath";
    let fd = dbg!(
        crate::open(
            dbg!(path),
            crate::O_CREAT | crate::O_RDWR | crate::O_CLOEXEC
        )
    ).unwrap();

    let mut buf = [0; 4096];
    let count = dbg!(
        crate::fpath(fd, &mut buf)
    ).unwrap();

    assert_eq!(dbg!(str::from_utf8(&buf[..count])), Ok(path));

    assert_eq!(dbg!(crate::close(fd)), Ok(0));
}

//TODO: frename

#[test]
fn fstat() {
    let path = "file:/tmp/syscall-tests-fstat";
    let fd = dbg!(
        crate::open(
            dbg!(path),
            crate::O_CREAT | crate::O_RDWR | crate::O_CLOEXEC
        )
    ).unwrap();

    let mut stat = crate::Stat::default();
    assert_eq!(dbg!(crate::fstat(fd, &mut stat)), Ok(0));
    assert_ne!(dbg!(stat), crate::Stat::default());

    assert_eq!(dbg!(crate::close(fd)), Ok(0));
}

#[test]
fn fstatvfs() {
    let path = "file:/tmp/syscall-tests-fstatvfs";
    let fd = dbg!(
        crate::open(
            dbg!(path),
            crate::O_CREAT | crate::O_RDWR | crate::O_CLOEXEC
        )
    ).unwrap();

    let mut statvfs = crate::StatVfs::default();
    assert_eq!(dbg!(crate::fstatvfs(fd, &mut statvfs)), Ok(0));
    assert_ne!(dbg!(statvfs), crate::StatVfs::default());

    assert_eq!(dbg!(crate::close(fd)), Ok(0));
}

//TODO: fsync

//TODO: ftruncate

//TODO: futimens

//TODO: futex

#[test]
fn getegid() {
    assert_eq!(crate::getegid(), Ok(0));
}

#[test]
fn getens() {
    assert_eq!(crate::getens(), Ok(1));
}

#[test]
fn geteuid() {
    assert_eq!(crate::geteuid(), Ok(0));
}

#[test]
fn getgid() {
    assert_eq!(crate::getgid(), Ok(0));
}

#[test]
fn getns() {
    assert_eq!(crate::getns(), Ok(1));
}

//TODO: getpid

//TODO: getpgid

//TODO: getppid

#[test]
fn getuid() {
    assert_eq!(crate::getuid(), Ok(0));
}

//TODO: iopl

//TODO: kill

//TODO: link (probably will not work)

#[test]
fn lseek() {
    let path = "file:/tmp/syscall-tests-lseek";
    let fd = dbg!(
        crate::open(
            dbg!(path),
            crate::O_CREAT | crate::O_RDWR | crate::O_CLOEXEC
        )
    ).unwrap();

    {
        let mut buf = [0; 256];
        for i in 0..buf.len() {
            buf[i] = i as u8;
        }
        assert_eq!(dbg!(crate::write(fd, &buf)), Ok(buf.len()));

        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_CUR)), Ok(buf.len()));
        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_SET)), Ok(0));
        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_END)), Ok(buf.len()));
        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_SET)), Ok(0));
    }

    {
        let mut buf = [0; 256];
        assert_eq!(dbg!(crate::read(fd, &mut buf)), Ok(buf.len()));
        for i in 0..buf.len() {
            assert_eq!(buf[i], i as u8);
        }

        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_CUR)), Ok(buf.len()));
        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_SET)), Ok(0));
        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_END)), Ok(buf.len()));
        assert_eq!(dbg!(crate::lseek(fd, 0, crate::SEEK_SET)), Ok(0));
    }

    assert_eq!(dbg!(crate::close(fd)), Ok(0));
}

//TODO: mkns

//TODO: mprotect

#[test]
fn nanosleep() {
    let req = crate::TimeSpec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut rem = crate::TimeSpec::default();
    assert_eq!(crate::nanosleep(&req, &mut rem), Ok(0));
    assert_eq!(rem, crate::TimeSpec::default());
}

//TODO: open

//TODO: physalloc

//TODO: physfree

//TODO: physmap

//TODO: physunmap

//TODO: read

#[test]
fn rmdir() {
    let path = "file:/tmp/syscall-tests-rmdir";
    let fd = dbg!(
        crate::open(
            dbg!(path),
            crate::O_CREAT | crate::O_DIRECTORY | crate::O_CLOEXEC
        )
    ).unwrap();

    assert_eq!(dbg!(crate::close(fd)), Ok(0));

    assert_eq!(dbg!(crate::rmdir(path)), Ok(0));
}

//TODO: setpgid

//TODO: setregid

//TODO: setrens

//TODO: setreuid

//TODO: sigaction

//TODO: sigprocmask

//TODO: sigreturn

#[test]
fn umask() {
    let old = dbg!(crate::umask(0o244)).unwrap();
    assert_eq!(dbg!(crate::umask(old)), Ok(0o244));
}

#[test]
fn unlink() {
    let path = "file:/tmp/syscall-tests-unlink";
    let fd = dbg!(
        crate::open(
            dbg!(path),
            crate::O_CREAT | crate::O_RDWR | crate::O_CLOEXEC
        )
    ).unwrap();

    assert_eq!(dbg!(crate::close(fd)), Ok(0));

    assert_eq!(dbg!(crate::unlink(path)), Ok(0));
}

//TODO: virttophys

// waitpid tested by clone

//TODO: write

#[test]
fn sched_yield() {
    assert_eq!(dbg!(crate::sched_yield()), Ok(0));
}

// TODO: fix?
/*
#[test]
fn sigaction() {
    use std::{
        mem,
        sync::atomic::{AtomicBool, Ordering}
    };

    static SA_HANDLER_WAS_RAN: AtomicBool = AtomicBool::new(false);
    static SA_HANDLER_2_WAS_IGNORED: AtomicBool = AtomicBool::new(false);

    let child = unsafe { crate::clone(crate::CLONE_VM).unwrap() };

    if child == 0 {
        let pid = crate::getpid().unwrap();

        extern "C" fn hello_im_a_signal_handler(signal: usize) {
            assert_eq!(signal, crate::SIGUSR1);
            SA_HANDLER_WAS_RAN.store(true, Ordering::SeqCst);
        }

        let my_signal_handler = crate::SigAction {
            sa_handler: Some(hello_im_a_signal_handler),
            ..Default::default()
        };
        crate::sigaction(crate::SIGUSR1, Some(&my_signal_handler), None).unwrap();

        crate::kill(pid, crate::SIGUSR1).unwrap(); // calls handler

        let mut old_signal_handler = crate::SigAction::default();
        crate::sigaction(
            crate::SIGUSR1,
            Some(&crate::SigAction {
                sa_handler: unsafe { mem::transmute::<usize, Option<extern "C" fn(usize)>>(crate::SIG_IGN) },
                ..Default::default()
            }),
            Some(&mut old_signal_handler)
        ).unwrap();
        assert_eq!(my_signal_handler, old_signal_handler);

        crate::kill(pid, crate::SIGUSR1).unwrap(); // does nothing

        SA_HANDLER_2_WAS_IGNORED.store(true, Ordering::SeqCst);

        crate::sigaction(
            crate::SIGUSR1,
            Some(&crate::SigAction {
                sa_handler: unsafe { mem::transmute::<usize, Option<extern "C" fn(usize)>>(crate::SIG_DFL) },
                ..Default::default()
            }),
            Some(&mut old_signal_handler)
        ).unwrap();

        crate::kill(pid, crate::SIGUSR1).unwrap(); // actually exits
    } else {
        let mut status = 0;
        dbg!(crate::waitpid(child, &mut status, crate::WaitFlags::empty())).unwrap();

        assert!(crate::wifsignaled(status));
        assert_eq!(crate::wtermsig(status), crate::SIGUSR1);

        assert!(SA_HANDLER_WAS_RAN.load(Ordering::SeqCst));
        assert!(SA_HANDLER_2_WAS_IGNORED.load(Ordering::SeqCst));
    }
}
*/
