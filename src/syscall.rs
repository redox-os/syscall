pub trait SyscallMem {
    unsafe fn to_slice<'a, T>(ptr: usize, len: usize) -> &'a [T];
    unsafe fn to_slice_mut<'a, T>(ptr: usize, len: usize) -> &'a mut [T];
}

#[derive(Syscall, Debug)]
#[allow(non_camel_case_types)]
pub enum Syscall<'a> {
    #[number = 45]
    brk(usize),
    #[number = 12]
    chdir(&'a [u8]),
    #[number = 0x1000_000f]
    chmod(&'a [u8], usize),
    #[number = 120]
    clone(usize),
    #[number = 0x2000_0006]
    close(usize),
    //#[number = 265] FIXME
    //clock_gettime(usize, &'a mut TimeSpec)
    #[number = 0x2010_0029]
    dup(usize, &'a [u8]),
    #[number = 0x2010_003f]
    dup2(usize, usize, &'a [u8]),
    #[number = 11]
    execve(&'a [u8], &'a [[usize; 2]]),
    #[number = 1]
    exit(usize),
    #[number = 0x2000_0037]
    fcntl(usize, usize, usize),
    #[number = 0x2000_039f]
    fevent(usize, usize),
    #[number = 0x2000_005a]
    fmap(usize, usize, usize),
    #[number = 0x2000_005b]
    funmap(usize),
    #[number = 0x2200_03a0]
    fpath(usize, &'a mut [u8]),
    //#[number = 0x2200_001c] FIXME
    //fstat(usize, &'a mut Stat),
    //#[number = 0x2200_0064] FIXME
    //fstatvfs(usize, &'a mut StatVfs),
    #[number = 0x2000_0076]
    fsync(usize),
    #[number = 0x2000_005d]
    ftruncate(usize, usize),
    #[number = 240]
    futex(*mut i32, usize, i32, usize, *mut i32), // XXX Raw pointer?
    #[number = 183]
    getcwd(&'a mut [u8]),
    #[number = 202]
    getegid(),
    #[number = 951]
    getens(),
    #[number = 201]
    geteuid(),
    #[number = 200]
    getgid(),
    #[number = 950]
    getns(),
    #[number = 20]
    getpid(),
    #[number = 64]
    getppid(),
    #[number = 199]
    getuid(),
    #[number = 110]
    iopl(usize),
    #[number = 37]
    kill(usize, usize),
    #[number = 0x1300_0009]
    link(*const u8, *const u8),
    #[number = 0x2000_0013]
    lseek(usize, isize, usize),
    #[number = 984]
    mkns(&'a [[usize; 2]]),
    //#[number = 162] FIXME
    //nanosleep(&'a TimeSpec, &'a mut TimeSpec),
    #[number = 0x1010_0005]
    open(&'a [u8], usize),
    #[number = 945]
    physalloc(usize),
    #[number = 946]
    physfree(usize, usize),
    #[number = 947]
    physmap(usize, usize, usize),
    #[number = 948]
    physunmap(usize),
    //#[number = 331] FIXME
    //pipe2(&'a mut [usize; 2], usize),
    #[number = 0x2200_0003]
    read(usize, &'a mut [u8]),
    #[number = 0x1000_0054]
    rmdir(&'a [u8]),
    #[number = 204]
    setregid(usize, usize),
    #[number = 952]
    setrens(usize, usize),
    #[number = 203]
    setreuid(usize, usize),
    #[number = 0x1000_000a]
    unlink(&'a [u8]),
    #[number = 949]
    virttophys(usize),
    //#[number = 7] FIXME
    //waitpid(usize, &'a mut usize, usize),
    #[number = 0x2100_0004]
    write(usize, &'a [u8]),
    #[number = 158]
    sched_yield(),
}
