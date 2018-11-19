use core::result;
use error::{Error, MAX_ERRNO};

/// Max ok value
const MAX_OK_VALUE: usize = -(MAX_ERRNO + 1) as usize;

/// A `Result` specialization for results in the kernel.
pub type Result<T> = result::Result<T, Error>;

/// Result multiplexer: encodes a `Result` into an `usize`. Returns `n` if
/// result is `Ok(n)`, else `-error.errno` if result is `Err(error)`:
///
/// ```
/// use syscall::*;
/// assert_eq!(5 as usize,  mux(Ok(5)));
/// assert_eq!(0 as usize,  mux(Ok(0)));
/// assert_eq!(-132i32 as usize, mux(Ok(-132i32 as usize)));
/// assert_eq!(-EIO as usize,  mux(Err(Error::new(EIO))));
/// ```
///
/// * `Ok`s are mapped to the numbers between `0` and
///    `2^[bits] - 132` inclusive.
/// * `Err`s are mapped to the numbers between `2^[bits] - 131`
///    and `2^[bits] - 1` inclusive.
///
/// # Panics
/// Panics if `Ok` value is greater than `2^[bits] - 132`.
pub fn mux(result: Result<usize>) -> usize {
    match result {
        Ok(value) if value <= MAX_OK_VALUE => value,
        Ok(_) => panic!("Huge Ok value can't be multiplexed."),
        Err(error) => -error.errno as usize,
    }
}

/// Result demultiplexer: decodes an `usize` into an `Result`. Returns
/// `Err(Error::new(-value))` if value (as i32) is in range 1..=131,
/// else `Ok(value)`.
///
/// ```
/// use syscall::*;
/// assert_eq!(Ok(5),  demux(5));
/// assert_eq!(Ok(0),  demux(0));
/// assert_eq!(Err(Error::new(EIO)), demux(-EIO as usize));
/// ```
pub fn demux(value: usize) -> Result<usize> {
    let errno = -(value as i32);
    if errno >= 1 && errno <= MAX_ERRNO {
        Err(Error::new(errno))
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn mux_huge_value() {
        let _ = mux(Ok(-131i32 as usize));
    }
}


