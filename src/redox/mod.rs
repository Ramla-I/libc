pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type c_char = i8;
pub type c_long = i64;
pub type c_ulong = u64;

pub type wchar_t = i32;
pub type wint_t = u32;
pub type wctype_t = i64;

pub type regoff_t = size_t;
pub type off_t = c_long;
pub type mode_t = c_int;
pub type time_t = c_long;
pub type pid_t = c_int;
pub type id_t = c_uint;
pub type gid_t = c_int;
pub type uid_t = c_int;
pub type dev_t = c_long;
pub type ino_t = c_ulong;
pub type nlink_t = c_ulong;
pub type blksize_t = c_long;
pub type blkcnt_t = c_ulong;

pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;

pub type useconds_t = c_uint;
pub type suseconds_t = c_int;

pub type clock_t = c_long;
pub type clockid_t = c_int;
pub type timer_t = *mut c_void;

pub type nfds_t = c_ulong;

s! {
    pub struct fd_set {
        fds_bits: [::c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

    pub struct pollfd {
        pub fd: ::c_int,
        pub events: ::c_short,
        pub revents: ::c_short,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_mode: ::mode_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,

        pub st_atime: ::timespec,
        pub st_mtime: ::timespec,
        pub st_ctime: ::timespec,

        _pad: [c_char; 24],
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }
}

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;

pub const FD_SETSIZE: usize = 1024;

pub const MAP_SHARED: ::c_int = 1;
pub const MAP_PRIVATE: ::c_int = 2;
pub const MAP_ANONYMOUS: ::c_int = 4;
pub const MAP_ANON: ::c_int = MAP_ANONYMOUS;

pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

pub const POLLIN: ::c_short = 0x001;
pub const POLLPRI: ::c_short = 0x002;
pub const POLLOUT: ::c_short = 0x004;
pub const POLLERR: ::c_short = 0x008;
pub const POLLHUP: ::c_short = 0x010;
pub const POLLNVAL: ::c_short = 0x020;

pub const PROT_NONE: ::c_int = 0;
pub const PROT_EXEC: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_READ: ::c_int = 4;

pub const S_ISUID: ::c_int = 0x800;
pub const S_ISGID: ::c_int = 0x400;
pub const S_ISVTX: ::c_int = 0x200;

pub const S_IFIFO: mode_t = 0x1000;
pub const S_IFCHR: mode_t = 0x2000;
pub const S_IFBLK: mode_t = 0x6000;
pub const S_IFDIR: mode_t = 0x4000;
pub const S_IFREG: mode_t = 0x8000;
pub const S_IFLNK: mode_t = 0xA000;
pub const S_IFSOCK: mode_t = 0xC000;
pub const S_IFMT: mode_t = 0xF000;
pub const S_IEXEC: mode_t = 0x40;
pub const S_IWRITE: mode_t = 0x80;
pub const S_IREAD: mode_t = 0x100;
pub const S_IRWXU: mode_t = 0x1C0;
pub const S_IXUSR: mode_t = 0x40;
pub const S_IWUSR: mode_t = 0x80;
pub const S_IRUSR: mode_t = 0x100;
pub const S_IRWXG: mode_t = 0x38;
pub const S_IXGRP: mode_t = 0x8;
pub const S_IWGRP: mode_t = 0x10;
pub const S_IRGRP: mode_t = 0x20;
pub const S_IRWXO: mode_t = 0x7;
pub const S_IXOTH: mode_t = 0x1;
pub const S_IWOTH: mode_t = 0x2;
pub const S_IROTH: mode_t = 0x4;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;

pub const FD_CLOEXEC: ::c_int =   0x0100_0000;

pub const O_RDONLY: ::c_int =     0x0001_0000;
pub const O_WRONLY: ::c_int =     0x0002_0000;
pub const O_RDWR: ::c_int =       0x0003_0000;
pub const O_NONBLOCK: ::c_int =   0x0004_0000;
pub const O_APPEND: ::c_int =     0x0008_0000;
pub const O_SHLOCK: ::c_int =     0x0010_0000;
pub const O_EXLOCK: ::c_int =     0x0020_0000;
pub const O_ASYNC: ::c_int =      0x0040_0000;
pub const O_FSYNC: ::c_int =      0x0080_0000;
pub const O_CLOEXEC: ::c_int =    0x0100_0000;
pub const O_CREAT: ::c_int =      0x0200_0000;
pub const O_TRUNC: ::c_int =      0x0400_0000;
pub const O_EXCL: ::c_int =       0x0800_0000;
pub const O_DIRECTORY: ::c_int =  0x1000_0000;
pub const O_STAT: ::c_int =       0x2000_0000;
pub const O_SYMLINK: ::c_int =    0x4000_0000;
pub const O_NOFOLLOW: ::c_int =   0x8000_0000;
pub const O_ACCMODE: ::c_int =    O_RDONLY | O_WRONLY | O_RDWR;

pub const SIGHUP:    ::c_int = 1;
pub const SIGINT:    ::c_int = 2;
pub const SIGQUIT:   ::c_int = 3;
pub const SIGILL:    ::c_int = 4;
pub const SIGTRAP:   ::c_int = 5;
pub const SIGABRT:   ::c_int = 6;
pub const SIGBUS:    ::c_int = 7;
pub const SIGFPE:    ::c_int = 8;
pub const SIGKILL:   ::c_int = 9;
pub const SIGUSR1:   ::c_int = 10;
pub const SIGSEGV:   ::c_int = 11;
pub const SIGUSR2:   ::c_int = 12;
pub const SIGPIPE:   ::c_int = 13;
pub const SIGALRM:   ::c_int = 14;
pub const SIGTERM:   ::c_int = 15;
pub const SIGSTKFLT: ::c_int = 16;
pub const SIGCHLD:   ::c_int = 17;
pub const SIGCONT:   ::c_int = 18;
pub const SIGSTOP:   ::c_int = 19;
pub const SIGTSTP:   ::c_int = 20;
pub const SIGTTIN:   ::c_int = 21;
pub const SIGTTOU:   ::c_int = 22;
pub const SIGURG:    ::c_int = 23;
pub const SIGXCPU:   ::c_int = 24;
pub const SIGXFSZ:   ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF:   ::c_int = 27;
pub const SIGWINCH:  ::c_int = 28;
pub const SIGIO:     ::c_int = 29;
pub const SIGPWR:    ::c_int = 30;
pub const SIGSYS:    ::c_int = 31;

pub enum FILE {}
pub enum fpos_t {} // TODO: fill this out with a struct

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

extern {
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn freopen(filename: *const c_char, mode: *const c_char,
                   file: *mut FILE) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int,
                   size: size_t) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nobj: size_t,
                 stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t,
                  stream: *mut FILE) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn atoi(s: *const c_char) -> c_int;
    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtol(s: *const c_char, endp: *mut *mut c_char,
                  base: c_int) -> c_long;
    pub fn strtoul(s: *const c_char, endp: *mut *mut c_char,
                   base: c_int) -> c_ulong;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    pub fn atexit(cb: extern fn()) -> c_int;
    pub fn system(s: *const c_char) -> c_int;
    pub fn getenv(s: *const c_char) -> *mut c_char;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char,
                   n: size_t) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(s: *mut c_char, ct: *const c_char,
                   n: size_t) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char,
                   n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(s1: *const c_char, s2: *const c_char,
                       n: size_t) -> c_int;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;
    pub fn wcslen(buf: *const wchar_t) -> size_t;
    pub fn wcstombs(dest: *mut c_char, src: *const wchar_t,
                    n: size_t) -> ::size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void,
                  n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void,
                   n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn abs(i: c_int) -> c_int;
    pub fn atof(s: *const c_char) -> c_double;
    pub fn labs(i: c_long) -> c_long;
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);

    pub fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> ::c_int;
    pub fn close(fd: ::c_int) -> ::c_int;
    pub fn fchown(fd: ::c_int, uid: ::uid_t, gid: ::gid_t) -> ::c_int;
    pub fn fcntl(fd: ::c_int, cmd: ::c_int, ...) -> ::c_int;
    pub fn fstat(fd: ::c_int, buf: *mut stat) -> ::c_int;
    pub fn fsync(fd: ::c_int) -> ::c_int;
    pub fn gethostname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
    pub fn getpid() -> pid_t;
    pub fn memalign(align: ::size_t, size: ::size_t) -> *mut ::c_void;
    pub fn mmap(addr: *mut ::c_void,
         len: ::size_t,
         prot: ::c_int,
         flags: ::c_int,
         fd: ::c_int,
         offset: off_t)
         -> *mut ::c_void;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int;
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: ::c_int) -> ::c_int;
    pub fn read(fd: ::c_int, buf: *mut ::c_void, count: ::size_t) -> ::ssize_t;
    pub fn setenv(name: *const c_char, val: *const c_char, overwrite: ::c_int)
                  -> ::c_int;
    pub fn unsetenv(name: *const c_char) -> ::c_int;
    pub fn write(fd: ::c_int, buf: *const ::c_void, count: ::size_t)
                 -> ::ssize_t;
}

#[link(name = "c")]
#[link(name = "m")]
extern {}

pub use self::net::*;

mod net;

cfg_if! {
    if #[cfg(core_cvoid)] {
        pub use core::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}


// errno implementation for Redox from relibc, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/errno.h.html

// pub const EPERM:            c_int = 1; /* Operation not permitted */
// pub const ENOENT:           c_int = 2; /* No such file or directory */
// pub const ESRCH:            c_int = 3; /* No such process */
// pub const EINTR:            c_int = 4; /* Interrupted system call */
// pub const EIO:              c_int = 5; /* I/O error */
// pub const ENXIO:            c_int = 6; /* No such device or address */
// pub const E2BIG:            c_int = 7; /* Argument list too long */
// pub const ENOEXEC:          c_int = 8; /* Exec format error */
// pub const EBADF:            c_int = 9; /* Bad file number */
// pub const ECHILD:           c_int = 10; /* No child processes */
// pub const EAGAIN:           c_int = 11; /* Try again */
// pub const ENOMEM:           c_int = 12; /* Out of memory */
// pub const EACCES:           c_int = 13; /* Permission denied */
// pub const EFAULT:           c_int = 14; /* Bad address */
// pub const ENOTBLK:          c_int = 15; /* Block device required */
// pub const EBUSY:            c_int = 16; /* Device or resource busy */
// pub const EEXIST:           c_int = 17; /* File exists */
// pub const EXDEV:            c_int = 18; /* Cross-device link */
// pub const ENODEV:           c_int = 19; /* No such device */
// pub const ENOTDIR:          c_int = 20; /* Not a directory */
// pub const EISDIR:           c_int = 21; /* Is a directory */
// pub const EINVAL:           c_int = 22; /* Invalid argument */
// pub const ENFILE:           c_int = 23; /* File table overflow */
// pub const EMFILE:           c_int = 24; /* Too many open files */
// pub const ENOTTY:           c_int = 25; /* Not a typewriter */
// pub const ETXTBSY:          c_int = 26; /* Text file busy */
// pub const EFBIG:            c_int = 27; /* File too large */
// pub const ENOSPC:           c_int = 28; /* No space left on device */
// pub const ESPIPE:           c_int = 29; /* Illegal seek */
// pub const EROFS:            c_int = 30; /* Read-only file system */
// pub const EMLINK:           c_int = 31; /* Too many links */
// pub const EPIPE:            c_int = 32; /* Broken pipe */
// pub const EDOM:             c_int = 33; /* Math argument out of domain of func */
// pub const ERANGE:           c_int = 34; /* Math result not representable */
// pub const EDEADLK:          c_int = 35; /* Resource deadlock would occur */
// pub const ENAMETOOLONG:     c_int = 36; /* File name too long */
// pub const ENOLCK:           c_int = 37; /* No record locks available */
// pub const ENOSYS:           c_int = 38; /* Function not implemented */
// pub const ENOTEMPTY:        c_int = 39; /* Directory not empty */
// pub const ELOOP:            c_int = 40; /* Too many symbolic links encountered */
// pub const EWOULDBLOCK:      c_int = 41; /* Operation would block */
// pub const ENOMSG:           c_int = 42; /* No message of desired type */
// pub const EIDRM:            c_int = 43; /* Identifier removed */
// pub const ECHRNG:           c_int = 44; /* Channel number out of range */
// pub const EL2NSYNC:         c_int = 45; /* Level 2 not synchronized */
// pub const EL3HLT:           c_int = 46; /* Level 3 halted */
// pub const EL3RST:           c_int = 47; /* Level 3 reset */
// pub const ELNRNG:           c_int = 48; /* Link number out of range */
// pub const EUNATCH:          c_int = 49; /* Protocol driver not attached */
// pub const ENOCSI:           c_int = 50; /* No CSI structure available */
// pub const EL2HLT:           c_int = 51; /* Level 2 halted */
// pub const EBADE:            c_int = 52; /* Invalid exchange */
// pub const EBADR:            c_int = 53; /* Invalid request descriptor */
// pub const EXFULL:           c_int = 54; /* Exchange full */
// pub const ENOANO:           c_int = 55; /* No anode */
// pub const EBADRQC:          c_int = 56; /* Invalid request code */
// pub const EBADSLT:          c_int = 57; /* Invalid slot */
// pub const EDEADLOCK:        c_int = 58; /* Resource deadlock would occur */
// pub const EBFONT:           c_int = 59; /* Bad font file format */
// pub const ENOSTR:           c_int = 60; /* Device not a stream */
// pub const ENODATA:          c_int = 61; /* No data available */
// pub const ETIME:            c_int = 62; /* Timer expired */
// pub const ENOSR:            c_int = 63; /* Out of streams resources */
// pub const ENONET:           c_int = 64; /* Machine is not on the network */
// pub const ENOPKG:           c_int = 65; /* Package not installed */
// pub const EREMOTE:          c_int = 66; /* Object is remote */
// pub const ENOLINK:          c_int = 67; /* Link has been severed */
// pub const EADV:             c_int = 68; /* Advertise error */
// pub const ESRMNT:           c_int = 69; /* Srmount error */
// pub const ECOMM:            c_int = 70; /* Communication error on send */
// pub const EPROTO:           c_int = 71; /* Protocol error */
// pub const EMULTIHOP:        c_int = 72; /* Multihop attempted */
// pub const EDOTDOT:          c_int = 73; /* RFS specific error */
// pub const EBADMSG:          c_int = 74; /* Not a data message */
// pub const EOVERFLOW:        c_int = 75; /* Value too large for defined data type */
// pub const ENOTUNIQ:         c_int = 76; /* Name not unique on network */
// pub const EBADFD:           c_int = 77; /* File descriptor in bad state */
// pub const EREMCHG:          c_int = 78; /* Remote address changed */
// pub const ELIBACC:          c_int = 79; /* Can not access a needed shared library */
// pub const ELIBBAD:          c_int = 80; /* Accessing a corrupted shared library */
// pub const ELIBSCN:          c_int = 81; /* .lib section in a.out corrupted */
// pub const ELIBMAX:          c_int = 82; /* Attempting to link in too many shared libraries */
// pub const ELIBEXEC:         c_int = 83; /* Cannot exec a shared library directly */
// pub const EILSEQ:           c_int = 84; /* Illegal byte sequence */
// pub const ERESTART:         c_int = 85; /* Interrupted system call should be restarted */
// pub const ESTRPIPE:         c_int = 86; /* Streams pipe error */
// pub const EUSERS:           c_int = 87; /* Too many users */
// pub const ENOTSOCK:         c_int = 88; /* Socket operation on non-socket */
// pub const EDESTADDRREQ:     c_int = 89; /* Destination address required */
// pub const EMSGSIZE:         c_int = 90; /* Message too long */
// pub const EPROTOTYPE:       c_int = 91; /* Protocol wrong type for socket */
// pub const ENOPROTOOPT:      c_int = 92; /* Protocol not available */
// pub const EPROTONOSUPPORT:  c_int = 93; /* Protocol not supported */
// pub const ESOCKTNOSUPPORT:  c_int = 94; /* Socket type not supported */
// pub const EOPNOTSUPP:       c_int = 95; /* Operation not supported on transport endpoint */
// pub const EPFNOSUPPORT:     c_int = 96; /* Protocol family not supported */
// pub const EAFNOSUPPORT:     c_int = 97; /* Address family not supported by protocol */
// pub const EADDRINUSE:       c_int = 98; /* Address already in use */
// pub const EADDRNOTAVAIL:    c_int = 99; /* Cannot assign requested address */
// pub const ENETDOWN:         c_int = 100; /* Network is down */
// pub const ENETUNREACH:      c_int = 101; /* Network is unreachable */
// pub const ENETRESET:        c_int = 102; /* Network dropped connection because of reset */
// pub const ECONNABORTED:     c_int = 103; /* Software caused connection abort */
// pub const ECONNRESET:       c_int = 104; /* Connection reset by peer */
// pub const ENOBUFS:          c_int = 105; /* No buffer space available */
// pub const EISCONN:          c_int = 106; /* Transport endpoint is already connected */
// pub const ENOTCONN:         c_int = 107; /* Transport endpoint is not connected */
// pub const ESHUTDOWN:        c_int = 108; /* Cannot send after transport endpoint shutdown */
// pub const ETOOMANYREFS:     c_int = 109; /* Too many references: cannot splice */
// pub const ETIMEDOUT:        c_int = 110; /* Connection timed out */
// pub const ECONNREFUSED:     c_int = 111; /* Connection refused */
// pub const EHOSTDOWN:        c_int = 112; /* Host is down */
// pub const EHOSTUNREACH:     c_int = 113; /* No route to host */
// pub const EALREADY:         c_int = 114; /* Operation already in progress */
// pub const EINPROGRESS:      c_int = 115; /* Operation now in progress */
// pub const ESTALE:           c_int = 116; /* Stale NFS file handle */
// pub const EUCLEAN:          c_int = 117; /* Structure needs cleaning */
// pub const ENOTNAM:          c_int = 118; /* Not a XENIX named type file */
// pub const ENAVAIL:          c_int = 119; /* No XENIX semaphores available */
// pub const EISNAM:           c_int = 120; /* Is a named type file */
// pub const EREMOTEIO:        c_int = 121; /* Remote I/O error */
// pub const EDQUOT:           c_int = 122; /* Quota exceeded */
// pub const ENOMEDIUM:        c_int = 123; /* No medium found */
// pub const EMEDIUMTYPE:      c_int = 124; /* Wrong medium type */
// pub const ECANCELED:        c_int = 125; /* Operation Canceled */
// pub const ENOKEY:           c_int = 126; /* Required key not available */
// pub const EKEYEXPIRED:      c_int = 127; /* Key has expired */
// pub const EKEYREVOKED:      c_int = 128; /* Key has been revoked */
// pub const EKEYREJECTED:     c_int = 129; /* Key was rejected by service */
// pub const EOWNERDEAD:       c_int = 130; /* Owner died */
// pub const ENOTRECOVERABLE:  c_int = 131; /* State not recoverable */

// pub static STR_ERROR: [&'static str; 132] = [
//     "Success",
//     "Operation not permitted",
//     "No such file or directory",
//     "No such process",
//     "Interrupted system call",
//     "I/O error",
//     "No such device or address",
//     "Argument list too long",
//     "Exec format error",
//     "Bad file number",
//     "No child processes",
//     "Try again",
//     "Out of memory",
//     "Permission denied",
//     "Bad address",
//     "Block device required",
//     "Device or resource busy",
//     "File exists",
//     "Cross-device link",
//     "No such device",
//     "Not a directory",
//     "Is a directory",
//     "Invalid argument",
//     "File table overflow",
//     "Too many open files",
//     "Not a typewriter",
//     "Text file busy",
//     "File too large",
//     "No space left on device",
//     "Illegal seek",
//     "Read-only file system",
//     "Too many links",
//     "Broken pipe",
//     "Math argument out of domain of func",
//     "Math result not representable",
//     "Resource deadlock would occur",
//     "File name too long",
//     "No record locks available",
//     "Function not implemented",
//     "Directory not empty",
//     "Too many symbolic links encountered",
//     "Operation would block",
//     "No message of desired type",
//     "Identifier removed",
//     "Channel number out of range",
//     "Level 2 not synchronized",
//     "Level 3 halted",
//     "Level 3 reset",
//     "Link number out of range",
//     "Protocol driver not attached",
//     "No CSI structure available",
//     "Level 2 halted",
//     "Invalid exchange",
//     "Invalid request descriptor",
//     "Exchange full",
//     "No anode",
//     "Invalid request code",
//     "Invalid slot",
//     "Resource deadlock would occur",
//     "Bad font file format",
//     "Device not a stream",
//     "No data available",
//     "Timer expired",
//     "Out of streams resources",
//     "Machine is not on the network",
//     "Package not installed",
//     "Object is remote",
//     "Link has been severed",
//     "Advertise error",
//     "Srmount error",
//     "Communication error on send",
//     "Protocol error",
//     "Multihop attempted",
//     "RFS specific error",
//     "Not a data message",
//     "Value too large for defined data type",
//     "Name not unique on network",
//     "File descriptor in bad state",
//     "Remote address changed",
//     "Can not access a needed shared library",
//     "Accessing a corrupted shared library",
//     ".lib section in a.out corrupted",
//     "Attempting to link in too many shared libraries",
//     "Cannot exec a shared library directly",
//     "Illegal byte sequence",
//     "Interrupted system call should be restarted",
//     "Streams pipe error",
//     "Too many users",
//     "Socket operation on non-socket",
//     "Destination address required",
//     "Message too long",
//     "Protocol wrong type for socket",
//     "Protocol not available",
//     "Protocol not supported",
//     "Socket type not supported",
//     "Operation not supported on transport endpoint",
//     "Protocol family not supported",
//     "Address family not supported by protocol",
//     "Address already in use",
//     "Cannot assign requested address",
//     "Network is down",
//     "Network is unreachable",
//     "Network dropped connection because of reset",
//     "Software caused connection abort",
//     "Connection reset by peer",
//     "No buffer space available",
//     "Transport endpoint is already connected",
//     "Transport endpoint is not connected",
//     "Cannot send after transport endpoint shutdown",
//     "Too many references: cannot splice",
//     "Connection timed out",
//     "Connection refused",
//     "Host is down",
//     "No route to host",
//     "Operation already in progress",
//     "Operation now in progress",
//     "Stale NFS file handle",
//     "Structure needs cleaning",
//     "Not a XENIX named type file",
//     "No XENIX semaphores available",
//     "Is a named type file",
//     "Remote I/O error",
//     "Quota exceeded",
//     "No medium found",
//     "Wrong medium type",
//     "Operation Canceled",
//     "Required key not available",
//     "Key has expired",
//     "Key has been revoked",
//     "Key was rejected by service",
//     "Owner died",
//     "State not recoverable",
// ];
