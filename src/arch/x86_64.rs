use super::result::{self, Result};

pub unsafe fn syscall0(mut a: usize) -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a)
        : "memory"
        : "intel", "volatile");

    result::demux(a)
}

pub unsafe fn syscall1(mut a: usize, b: usize) -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a), "{rbx}"(b)
        : "memory"
        : "intel", "volatile");

    result::demux(a)
}

// Clobbers all registers - special for clone
pub unsafe fn syscall1_clobber(mut a: usize, b: usize) -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a), "{rbx}"(b)
        : "memory", "rbx", "rcx", "rdx", "rsi", "rdi", "r8",
          "r9", "r10", "r11", "r12", "r13", "r14", "r15"
        : "intel", "volatile");

    result::demux(a)
}

pub unsafe fn syscall2(mut a: usize, b: usize, c: usize) -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a), "{rbx}"(b), "{rcx}"(c)
        : "memory"
        : "intel", "volatile");

    result::demux(a)
}

pub unsafe fn syscall3(mut a: usize, b: usize, c: usize, d: usize) -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a), "{rbx}"(b), "{rcx}"(c), "{rdx}"(d)
        : "memory"
        : "intel", "volatile");

    result::demux(a)
}

pub unsafe fn syscall4(mut a: usize, b: usize, c: usize, d: usize, e: usize) -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a), "{rbx}"(b), "{rcx}"(c), "{rdx}"(d), "{rsi}"(e)
        : "memory"
        : "intel", "volatile");

    result::demux(a)
}

pub unsafe fn syscall5(mut a: usize, b: usize, c: usize, d: usize, e: usize, f: usize)
                       -> Result<usize> {
    asm!("int 0x80"
        : "={rax}"(a)
        : "{rax}"(a), "{rbx}"(b), "{rcx}"(c), "{rdx}"(d), "{rsi}"(e), "{rdi}"(f)
        : "memory"
        : "intel", "volatile");

    result::demux(a)
}
