#![no_std]
#![feature(decl_macro)]

pub mod consts;
pub mod syscall;

pub use crate::syscall::*;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::syscall::*;

    pub use crate::AeroSyscallError;
}

bitflags::bitflags! {
    pub struct MMapProt: usize {
        const PROT_READ = 0x1;
        const PROT_WRITE = 0x2;
        const PROT_EXEC = 0x4;
        const PROT_NONE = 0x0;
    }
}

bitflags::bitflags! {
    pub struct MMapFlags: usize {
        const MAP_PRIVATE = 0x1;
        const MAP_SHARED = 0x2;
        const MAP_FIXED = 0x4;
        const MAP_ANONYOMUS = 0x8;
    }
}

bitflags::bitflags! {
    pub struct OpenFlags: usize {
        const O_RDONLY      = 2;
        const O_RDWR        = 3;
        const O_WRONLY      = 5;
        const O_CREAT       = 0x10;
        const O_DIRECTORY   = 0x20;
        const O_EXCL        = 0x40;
        const O_NOCTTY      = 0x80;
        const O_TRUNC       = 0x0200;
        const O_CLOEXEC     = 0x4000;
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(isize)]
pub enum AeroSyscallError {
    EDOM = 1,
    EILSEQ = 2,
    ERANGE = 3,

    E2BIG = 1001,
    EACCES = 1002,
    EADDRINUSE = 1003,
    EADDRNOTAVAIL = 1004,
    EAFNOSUPPORT = 1005,
    EAGAIN = 1006,
    EALREADY = 1007,
    EBADF = 1008,
    EBADMSG = 1009,
    EBUSY = 1010,
    ECANCELED = 1011,
    ECHILD = 1012,
    ECONNABORTED = 1013,
    ECONNREFUSED = 1014,
    ECONNRESET = 1015,
    EDEADLK = 1016,
    EDESTADDRREQ = 1017,
    EDQUOT = 1018,
    EEXIST = 1019,
    EFAULT = 1020,
    EFBIG = 1021,
    EHOSTUNREACH = 1022,
    EIDRM = 1023,
    EINPROGRESS = 1024,
    EINTR = 1025,
    EINVAL = 1026,
    EIO = 1027,
    EISCONN = 1028,
    EISDIR = 1029,
    ELOOP = 1030,
    EMFILE = 1031,
    EMLINK = 1032,
    EMSGSIZE = 1034,
    EMULTIHOP = 1035,
    ENAMETOOLONG = 1036,
    ENETDOWN = 1037,
    ENETRESET = 1038,
    ENETUNREACH = 1039,
    ENFILE = 1040,
    ENOBUFS = 1041,
    ENODEV = 1042,
    ENOENT = 1043,
    ENOEXEC = 1044,
    ENOLCK = 1045,
    ENOLINK = 1046,
    ENOMEM = 1047,
    ENOMSG = 1048,
    ENOPROTOOPT = 1049,
    ENOSPC = 1050,
    ENOSYS = 1051,
    ENOTCONN = 1052,
    ENOTDIR = 1053,
    ENOTEMPTY = 1054,
    ENOTRECOVERABLE = 1055,
    ENOTSOCK = 1056,
    ENOTSUP = 1057,
    ENOTTY = 1058,
    ENXIO = 1059,
    EOPNOTSUPP = 1060,
    EOVERFLOW = 1061,
    EOWNERDEAD = 1062,
    EPERM = 1063,
    EPIPE = 1064,
    EPROTO = 1065,
    EPROTONOSUPPORT = 1066,
    EPROTOTYPE = 1067,
    EROFS = 1068,
    ESPIPE = 1069,
    ESRCH = 1070,
    ESTALE = 1071,
    ETIMEDOUT = 1072,
    ETXTBSY = 1073,
    EXDEV = 1075,
    ENODATA = 1076,
    ETIME = 1077,
    ENOKEY = 1078,
    ESHUTDOWN = 1079,
    EHOSTDOWN = 1080,
    EBADFD = 1081,
    ENOMEDIUM = 1082,
    ENOTBLK = 1083,

    Unknown = isize::MAX,
}

#[derive(Debug)]
pub enum SysFileType {
    File,
    Directory,
    Device,
}

#[repr(C, packed)]
pub struct SysDirEntry {
    pub inode: usize,
    pub offset: usize,
    pub reclen: usize,
    pub file_type: SysFileType,
    pub name: [u8; 0],
}

#[repr(C)]
pub struct Utsname {
    pub name: [u8; 65],
    pub nodename: [u8; 65],
    pub release: [u8; 65],
    pub version: [u8; 65],
    pub machine: [u8; 65],
}

impl Utsname {
    pub fn name(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.name) }
    }

    pub fn nodename(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.nodename) }
    }

    pub fn release(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.release) }
    }

    pub fn version(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.version) }
    }

    pub fn machine(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.machine) }
    }
}

impl Default for Utsname {
    fn default() -> Self {
        Self {
            name: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
        }
    }
}

pub const TIOCGWINSZ: usize = 0x5413;
pub const TCGETS: usize = 0x5401;
pub const TCSETSF: usize = 0x5404;

#[derive(Default)]
#[repr(C)]
pub struct WinSize {
    pub ws_row: u16,
    pub ws_col: u16,
    pub ws_xpixel: u16,
    pub ws_ypixel: u16,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct TermiosLFlag: u32 {
        const ECHO   = 0000010; // Enable echo
        const ECHOE  = 0000020; // Echo erase character as error-correcting backspace
        const ECHOK  = 0000040; // Echo kill
        const ECHONL = 0000100; // Echo NL
        const NOFLSH = 0000200; // Disable flush after interrupt or quit
        const TOSTOP = 0000400; // Send SIGTTOU for background output
        const ICANON = 0000002; // Canonical input (erase and kill processing)
    }
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
pub struct Termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: TermiosLFlag,
    pub c_line: u8,
    pub c_cc: [u8; 32],
    pub c_ispeed: u32,
    pub c_ospeed: u32,
}

pub const AT_FDCWD: isize = -100;

pub fn syscall_result_as_usize(result: Result<usize, AeroSyscallError>) -> usize {
    match result {
        Ok(value) => value as _,
        Err(error) => -(error as isize) as _,
    }
}

/// Inner helper function that converts the syscall result value into the
/// Rust [`Result`] type.
fn isize_as_syscall_result(value: isize) -> Result<usize, AeroSyscallError> {
    if value >= 0 {
        Ok(value as usize)
    } else {
        let err: AeroSyscallError = unsafe { core::mem::transmute((-value) as u64) };
        Err(err)
    }
}

pub fn sys_exit(status: usize) -> ! {
    syscall1(prelude::SYS_EXIT, status);
    unreachable!()
}

pub fn sys_open(path: &str, mode: OpenFlags) -> Result<usize, AeroSyscallError> {
    let value = syscall4(
        prelude::SYS_OPEN,
        0x00,
        path.as_ptr() as usize,
        path.len(),
        mode.bits(),
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_write(fd: usize, buf: &[u8]) -> Result<usize, AeroSyscallError> {
    let value = syscall3(
        prelude::SYS_WRITE,
        fd as usize,
        buf.as_ptr() as usize,
        buf.len(),
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> Result<usize, AeroSyscallError> {
    let value = syscall3(
        prelude::SYS_READ,
        fd as usize,
        buf.as_mut_ptr() as usize,
        buf.len(),
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_chdir(path: &str) -> Result<usize, AeroSyscallError> {
    let value = syscall2(prelude::SYS_CHDIR, path.as_ptr() as usize, path.len());
    isize_as_syscall_result(value as _)
}

pub fn sys_close(fd: usize) -> Result<usize, AeroSyscallError> {
    let value = syscall1(prelude::SYS_CLOSE, fd);
    isize_as_syscall_result(value as _)
}

pub fn sys_getcwd(buf: &mut [u8]) -> Result<usize, AeroSyscallError> {
    let value = syscall2(prelude::SYS_GETCWD, buf.as_mut_ptr() as usize, buf.len());
    isize_as_syscall_result(value as _)
}

pub fn sys_getdents(fd: usize, buf: &mut [u8]) -> Result<usize, AeroSyscallError> {
    let value = syscall3(
        prelude::SYS_GETDENTS,
        fd as usize,
        buf.as_mut_ptr() as usize,
        buf.len(),
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_fork() -> Result<usize, AeroSyscallError> {
    let value = syscall0(prelude::SYS_FORK);
    isize_as_syscall_result(value as _)
}

pub fn sys_munmap(address: usize, size: usize) -> Result<usize, AeroSyscallError> {
    let value = syscall2(prelude::SYS_MUNMAP, address as usize, size as usize);
    isize_as_syscall_result(value as _)
}

pub fn sys_mkdir(path: &str) -> Result<usize, AeroSyscallError> {
    let value = syscall2(prelude::SYS_MKDIR, path.as_ptr() as usize, path.len());
    isize_as_syscall_result(value as _)
}

pub fn sys_log(message: &str) -> Result<usize, AeroSyscallError> {
    let value = syscall2(prelude::SYS_LOG, message.as_ptr() as usize, message.len());
    isize_as_syscall_result(value as _)
}

pub fn sys_mkdirat(dfd: isize, path: &str) -> Result<usize, AeroSyscallError> {
    let value = syscall3(
        prelude::SYS_MKDIR_AT,
        dfd as usize,
        path.as_ptr() as usize,
        path.len(),
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_exec(path: &str) -> Result<usize, AeroSyscallError> {
    let value = syscall6(
        prelude::SYS_EXEC,
        path.as_ptr() as usize,
        path.len(),
        0,
        0,
        0,
        0,
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_rmdir(path: &str) -> Result<usize, AeroSyscallError> {
    let value = syscall2(prelude::SYS_RMDIR, path.as_ptr() as usize, path.len());
    isize_as_syscall_result(value as _)
}

pub fn sys_uname(struc: &mut Utsname) -> Result<usize, AeroSyscallError> {
    let value = syscall1(prelude::SYS_UNAME, struc as *mut Utsname as usize);
    isize_as_syscall_result(value as _)
}

pub fn sys_shutdown() -> ! {
    syscall0(prelude::SYS_SHUTDOWN);
    unreachable!()
}

pub fn sys_waitpid(pid: usize, status: &mut u32, flags: usize) -> Result<usize, AeroSyscallError> {
    let value = syscall3(
        prelude::SYS_WAITPID,
        pid as usize,
        status as *mut u32 as usize,
        flags,
    );

    isize_as_syscall_result(value as _)
}

pub fn sys_ioctl(fd: usize, command: usize, arg: usize) -> Result<usize, AeroSyscallError> {
    let value = syscall3(prelude::SYS_IOCTL, fd as usize, command, arg);
    isize_as_syscall_result(value as _)
}

pub fn sys_mmap(
    address: usize,
    size: usize,
    protocol: MMapProt,
    flags: MMapFlags,
    fd: usize,
    offset: usize,
) -> Result<usize, AeroSyscallError> {
    let value = syscall6(
        prelude::SYS_MMAP,
        address,
        size,
        protocol.bits(),
        flags.bits(),
        fd,
        offset,
    );

    isize_as_syscall_result(value as _)
}
