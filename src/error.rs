use core::fmt;
use core::result;

/// An error returned by the kernel,  identified by an `errno`.
#[derive(Eq, PartialEq)]
pub struct Error {
    pub errno: i32,
}

impl Error {
    /// Creates a new `Error`.
    ///
    /// # Panics
    /// Panics if `errno` not in range `1..=131`
    pub fn new(errno: i32) -> Error {
        if errno < MIN_ERRNO || errno > MAX_ERRNO {
            panic!("Only errnos between 1 and 131 are allowed. Use constants in error.rs");
        }
        Error { errno: errno }
    }

    /// Returns a text representation of this error.
    ///
    /// ```
    /// use syscall::*;
    /// assert_eq!("Operation not permitted", Error::new(EPERM).text());
    /// ```
    pub fn text(&self) -> &'static str {
        #[allow(deprecated)]
        STR_ERROR.get(self.errno as usize).map(|&x| x).expect("SHOULD NOT HAPPEN (bad errno)")
    }
}

impl fmt::Debug for Error {
    /// Uses the text representation of this error for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.write_str(self.text())
    }
}

impl fmt::Display for Error {
    /// Uses the text representation of this error for debug.
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        f.write_str(self.text())
    }
}

/// Operation not permitted
pub const EPERM: i32 = 1;

/// No such file or directory
pub const ENOENT: i32 = 2;

/// No such process
pub const ESRCH: i32 = 3;

/// Interrupted system call
pub const EINTR: i32 = 4;

/// I/O error
pub const EIO: i32 = 5;

/// No such device or address
pub const ENXIO: i32 = 6;

/// Argument list too long
pub const E2BIG: i32 = 7;

/// Exec format error
pub const ENOEXEC: i32 = 8;

/// Bad file number
pub const EBADF: i32 = 9;

/// No child processes
pub const ECHILD: i32 = 10;

/// Try again
pub const EAGAIN: i32 = 11;

/// Out of memory
pub const ENOMEM: i32 = 12;

/// Permission denied
pub const EACCES: i32 = 13;

/// Bad address
pub const EFAULT: i32 = 14;

/// Block device required
pub const ENOTBLK: i32 = 15;

/// Device or resource busy
pub const EBUSY: i32 = 16;

/// File exists
pub const EEXIST: i32 = 17;

/// Cross-device link
pub const EXDEV: i32 = 18;

/// No such device
pub const ENODEV: i32 = 19;

/// Not a directory
pub const ENOTDIR: i32 = 20;

/// Is a directory
pub const EISDIR: i32 = 21;

/// Invalid argument
pub const EINVAL: i32 = 22;

/// File table overflow
pub const ENFILE: i32 = 23;

/// Too many open files
pub const EMFILE: i32 = 24;

/// Not a typewriter
pub const ENOTTY: i32 = 25;

/// Text file busy
pub const ETXTBSY: i32 = 26;

/// File too large
pub const EFBIG: i32 = 27;

/// No space left on device
pub const ENOSPC: i32 = 28;

/// Illegal seek
pub const ESPIPE: i32 = 29;

/// Read-only file system
pub const EROFS: i32 = 30;

/// Too many links
pub const EMLINK: i32 = 31;

/// Broken pipe
pub const EPIPE: i32 = 32;

/// Math argument out of domain of func
pub const EDOM: i32 = 33;

/// Math result not representable
pub const ERANGE: i32 = 34;

/// Resource deadlock would occur
pub const EDEADLK: i32 = 35;

/// File name too long
pub const ENAMETOOLONG: i32 = 36;

/// No record locks available
pub const ENOLCK: i32 = 37;

/// Function not implemented
pub const ENOSYS: i32 = 38;

/// Directory not empty
pub const ENOTEMPTY: i32 = 39;

/// Too many symbolic links encountered
pub const ELOOP: i32 = 40;

/// Operation would block
pub const EWOULDBLOCK: i32 = 41;

/// No message of desired type
pub const ENOMSG: i32 = 42;

/// Identifier removed
pub const EIDRM: i32 = 43;

/// Channel number out of range
pub const ECHRNG: i32 = 44;

/// Level 2 not synchronized
pub const EL2NSYNC: i32 = 45;

/// Level 3 halted
pub const EL3HLT: i32 = 46;

/// Level 3 reset
pub const EL3RST: i32 = 47;

/// Link number out of range
pub const ELNRNG: i32 = 48;

/// Protocol driver not attached
pub const EUNATCH: i32 = 49;

/// No CSI structure available
pub const ENOCSI: i32 = 50;

/// Level 2 halted
pub const EL2HLT: i32 = 51;

/// Invalid exchange
pub const EBADE: i32 = 52;

/// Invalid request descriptor
pub const EBADR: i32 = 53;

/// Exchange full
pub const EXFULL: i32 = 54;

/// No anode
pub const ENOANO: i32 = 55;

/// Invalid request code
pub const EBADRQC: i32 = 56;

/// Invalid slot
pub const EBADSLT: i32 = 57;

/// Resource deadlock would occur
pub const EDEADLOCK: i32 = 58;

/// Bad font file format
pub const EBFONT: i32 = 59;

/// Device not a stream
pub const ENOSTR: i32 = 60;

/// No data available
pub const ENODATA: i32 = 61;

/// Timer expired
pub const ETIME: i32 = 62;

/// Out of streams resources
pub const ENOSR: i32 = 63;

/// Machine is not on the network
pub const ENONET: i32 = 64;

/// Package not installed
pub const ENOPKG: i32 = 65;

/// Object is remote
pub const EREMOTE: i32 = 66;

/// Link has been severed
pub const ENOLINK: i32 = 67;

/// Advertise error
pub const EADV: i32 = 68;

/// Srmount error
pub const ESRMNT: i32 = 69;

/// Communication error on send
pub const ECOMM: i32 = 70;

/// Protocol error
pub const EPROTO: i32 = 71;

/// Multihop attempted
pub const EMULTIHOP: i32 = 72;

/// RFS specific error
pub const EDOTDOT: i32 = 73;

/// Not a data message
pub const EBADMSG: i32 = 74;

/// Value too large for defined data type
pub const EOVERFLOW: i32 = 75;

/// Name not unique on network
pub const ENOTUNIQ: i32 = 76;

/// File descriptor in bad state
pub const EBADFD: i32 = 77;

/// Remote address changed
pub const EREMCHG: i32 = 78;

/// Can not access a needed shared library
pub const ELIBACC: i32 = 79;

/// Accessing a corrupted shared library
pub const ELIBBAD: i32 = 80;

/// .lib section in a.out corrupted
pub const ELIBSCN: i32 = 81;

/// Attempting to link in too many shared libraries
pub const ELIBMAX: i32 = 82;

/// Cannot exec a shared library directly
pub const ELIBEXEC: i32 = 83;

/// Illegal byte sequence
pub const EILSEQ: i32 = 84;

/// Interrupted system call should be restarted
pub const ERESTART: i32 = 85;

/// Streams pipe error
pub const ESTRPIPE: i32 = 86;

/// Too many users
pub const EUSERS: i32 = 87;

/// Socket operation on non-socket
pub const ENOTSOCK: i32 = 88;

/// Destination address required
pub const EDESTADDRREQ: i32 = 89;

/// Message too long
pub const EMSGSIZE: i32 = 90;

/// Protocol wrong type for socket
pub const EPROTOTYPE: i32 = 91;

/// Protocol not available
pub const ENOPROTOOPT: i32 = 92;

/// Protocol not supported
pub const EPROTONOSUPPORT: i32 = 93;

/// Socket type not supported
pub const ESOCKTNOSUPPORT: i32 = 94;

/// Operation not supported on transport endpoint
pub const EOPNOTSUPP: i32 = 95;

/// Protocol family not supported
pub const EPFNOSUPPORT: i32 = 96;

/// Address family not supported by protocol
pub const EAFNOSUPPORT: i32 = 97;

/// Address already in use
pub const EADDRINUSE: i32 = 98;

/// Cannot assign requested address
pub const EADDRNOTAVAIL: i32 = 99;

/// Network is down
pub const ENETDOWN: i32 = 100;

/// Network is unreachable
pub const ENETUNREACH: i32 = 101;

/// Network dropped connection because of reset
pub const ENETRESET: i32 = 102;

/// Software caused connection abort
pub const ECONNABORTED: i32 = 103;

/// Connection reset by peer
pub const ECONNRESET: i32 = 104;

/// No buffer space available
pub const ENOBUFS: i32 = 105;

/// Transport endpoint is already connected
pub const EISCONN: i32 = 106;

/// Transport endpoint is not connected
pub const ENOTCONN: i32 = 107;

/// Cannot send after transport endpoint shutdown
pub const ESHUTDOWN: i32 = 108;

/// Too many references: cannot splice
pub const ETOOMANYREFS: i32 = 109;

/// Connection timed out
pub const ETIMEDOUT: i32 = 110;

/// Connection refused
pub const ECONNREFUSED: i32 = 111;

/// Host is down
pub const EHOSTDOWN: i32 = 112;

/// No route to host
pub const EHOSTUNREACH: i32 = 113;

/// Operation already in progress
pub const EALREADY: i32 = 114;

/// Operation now in progress
pub const EINPROGRESS: i32 = 115;

/// Stale NFS file handle
pub const ESTALE: i32 = 116;

/// Structure needs cleaning
pub const EUCLEAN: i32 = 117;

/// Not a XENIX named type file
pub const ENOTNAM: i32 = 118;

/// No XENIX semaphores available
pub const ENAVAIL: i32 = 119;

/// Is a named type file
pub const EISNAM: i32 = 120;

/// Remote I/O error
pub const EREMOTEIO: i32 = 121;

/// Quota exceeded
pub const EDQUOT: i32 = 122;

/// No medium found
pub const ENOMEDIUM: i32 = 123;

/// Wrong medium type
pub const EMEDIUMTYPE: i32 = 124;

/// Operation Canceled
pub const ECANCELED: i32 = 125;

/// Required key not available
pub const ENOKEY: i32 = 126;

/// Key has expired
pub const EKEYEXPIRED: i32 = 127;

/// Key has been revoked
pub const EKEYREVOKED: i32 = 128;

/// Key was rejected by service
pub const EKEYREJECTED: i32 = 129;

/// Owner died
pub const EOWNERDEAD: i32 = 130;

/// State not recoverable
pub const ENOTRECOVERABLE: i32 = 131;

/// Min error no
const MIN_ERRNO: i32 = EPERM;

/// Max error no
pub const MAX_ERRNO: i32 = ENOTRECOVERABLE;

/// Error texts
#[deprecated(note = "use error_str function")]
pub static STR_ERROR: [&'static str; MAX_ERRNO as usize+1] = [
    "Success",
    "Operation not permitted",
    "No such file or directory",
    "No such process",
    "Interrupted system call",
    "I/O error",
    "No such device or address",
    "Argument list too long",
    "Exec format error",
    "Bad file number",
    "No child processes",
    "Try again",
    "Out of memory",
    "Permission denied",
    "Bad address",
    "Block device required",
    "Device or resource busy",
    "File exists",
    "Cross-device link",
    "No such device",
    "Not a directory",
    "Is a directory",
    "Invalid argument",
    "File table overflow",
    "Too many open files",
    "Not a typewriter",
    "Text file busy",
    "File too large",
    "No space left on device",
    "Illegal seek",
    "Read-only file system",
    "Too many links",
    "Broken pipe",
    "Math argument out of domain of func",
    "Math result not representable",
    "Resource deadlock would occur",
    "File name too long",
    "No record locks available",
    "Function not implemented",
    "Directory not empty",
    "Too many symbolic links encountered",
    "Operation would block",
    "No message of desired type",
    "Identifier removed",
    "Channel number out of range",
    "Level 2 not synchronized",
    "Level 3 halted",
    "Level 3 reset",
    "Link number out of range",
    "Protocol driver not attached",
    "No CSI structure available",
    "Level 2 halted",
    "Invalid exchange",
    "Invalid request descriptor",
    "Exchange full",
    "No anode",
    "Invalid request code",
    "Invalid slot",
    "Resource deadlock would occur",
    "Bad font file format",
    "Device not a stream",
    "No data available",
    "Timer expired",
    "Out of streams resources",
    "Machine is not on the network",
    "Package not installed",
    "Object is remote",
    "Link has been severed",
    "Advertise error",
    "Srmount error",
    "Communication error on send",
    "Protocol error",
    "Multihop attempted",
    "RFS specific error",
    "Not a data message",
    "Value too large for defined data type",
    "Name not unique on network",
    "File descriptor in bad state",
    "Remote address changed",
    "Can not access a needed shared library",
    "Accessing a corrupted shared library",
    ".lib section in a.out corrupted",
    "Attempting to link in too many shared libraries",
    "Cannot exec a shared library directly",
    "Illegal byte sequence",
    "Interrupted system call should be restarted",
    "Streams pipe error",
    "Too many users",
    "Socket operation on non-socket",
    "Destination address required",
    "Message too long",
    "Protocol wrong type for socket",
    "Protocol not available",
    "Protocol not supported",
    "Socket type not supported",
    "Operation not supported on transport endpoint",
    "Protocol family not supported",
    "Address family not supported by protocol",
    "Address already in use",
    "Cannot assign requested address",
    "Network is down",
    "Network is unreachable",
    "Network dropped connection because of reset",
    "Software caused connection abort",
    "Connection reset by peer",
    "No buffer space available",
    "Transport endpoint is already connected",
    "Transport endpoint is not connected",
    "Cannot send after transport endpoint shutdown",
    "Too many references: cannot splice",
    "Connection timed out",
    "Connection refused",
    "Host is down",
    "No route to host",
    "Operation already in progress",
    "Operation now in progress",
    "Stale NFS file handle",
    "Structure needs cleaning",
    "Not a XENIX named type file",
    "No XENIX semaphores available",
    "Is a named type file",
    "Remote I/O error",
    "Quota exceeded",
    "No medium found",
    "Wrong medium type",
    "Operation Canceled",
    "Required key not available",
    "Key has expired",
    "Key has been revoked",
    "Key was rejected by service",
    "Owner died",
    "State not recoverable"];

/// Returns a text description associated with an `errorno`.
/// More robust than `Errrpr::text()`: won't panic if the
/// `errno` is not correct.
/// ```
/// use syscall::*;
/// assert_eq!("Success", error_str(0));
/// assert_eq!("State not recoverable", error_str(ENOTRECOVERABLE));
/// assert_eq!("Unknown error", error_str(-1));
/// assert_eq!("Unknown error", error_str(132));
/// ```
pub fn error_str(errno: i32) -> &'static str {
    #[allow(deprecated)]
    STR_ERROR
        .get(errno as usize)
        .map_or("Unknown error", |err| *err)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn zero_errno() {
        let _ = Error::new(0);
    }

    #[test]
    #[should_panic]
    fn negative_errno() {
        let _ = Error::new(-5);
    }

    #[test]
    #[should_panic]
    fn big_errno() {
        let _ = Error::new(200);
    }
}