//! Error numbers for arch `sparc`.

#![allow(dead_code)]

// This file automatically generate. Do not edit.

crate::macros::def_errno!();

impl Errno {
    /// Operation not permitted
    pub const EPERM: Self = Self(1);
    /// No such file or directory
    pub const ENOENT: Self = Self(2);
    /// No such process
    pub const ESRCH: Self = Self(3);
    /// Interrupted system call
    pub const EINTR: Self = Self(4);
    /// I/O error
    pub const EIO: Self = Self(5);
    /// No such device or address
    pub const ENXIO: Self = Self(6);
    /// Argument list too long
    pub const E2BIG: Self = Self(7);
    /// Exec format error
    pub const ENOEXEC: Self = Self(8);
    /// Bad file number
    pub const EBADF: Self = Self(9);
    /// No child processes
    pub const ECHILD: Self = Self(10);
    /// Try again
    pub const EAGAIN: Self = Self(11);
    /// Out of memory
    pub const ENOMEM: Self = Self(12);
    /// Permission denied
    pub const EACCES: Self = Self(13);
    /// Bad address
    pub const EFAULT: Self = Self(14);
    /// Block device required
    pub const ENOTBLK: Self = Self(15);
    /// Device or resource busy
    pub const EBUSY: Self = Self(16);
    /// File exists
    pub const EEXIST: Self = Self(17);
    /// Cross-device link
    pub const EXDEV: Self = Self(18);
    /// No such device
    pub const ENODEV: Self = Self(19);
    /// Not a directory
    pub const ENOTDIR: Self = Self(20);
    /// Is a directory
    pub const EISDIR: Self = Self(21);
    /// Invalid argument
    pub const EINVAL: Self = Self(22);
    /// File table overflow
    pub const ENFILE: Self = Self(23);
    /// Too many open files
    pub const EMFILE: Self = Self(24);
    /// Not a typewriter
    pub const ENOTTY: Self = Self(25);
    /// Text file busy
    pub const ETXTBSY: Self = Self(26);
    /// File too large
    pub const EFBIG: Self = Self(27);
    /// No space left on device
    pub const ENOSPC: Self = Self(28);
    /// Illegal seek
    pub const ESPIPE: Self = Self(29);
    /// Read-only file system
    pub const EROFS: Self = Self(30);
    /// Too many links
    pub const EMLINK: Self = Self(31);
    /// Broken pipe
    pub const EPIPE: Self = Self(32);
    /// Math argument out of domain of func
    pub const EDOM: Self = Self(33);
    /// Math result not representable
    pub const ERANGE: Self = Self(34);
    /// Operation now in progress
    pub const EINPROGRESS: Self = Self(36);
    /// Operation already in progress
    pub const EALREADY: Self = Self(37);
    /// Socket operation on non-socket
    pub const ENOTSOCK: Self = Self(38);
    /// Destination address required
    pub const EDESTADDRREQ: Self = Self(39);
    /// Message too long
    pub const EMSGSIZE: Self = Self(40);
    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Self = Self(41);
    /// Protocol not available
    pub const ENOPROTOOPT: Self = Self(42);
    /// Protocol not supported
    pub const EPROTONOSUPPORT: Self = Self(43);
    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Self = Self(44);
    /// Op not supported on transport endpoint
    pub const EOPNOTSUPP: Self = Self(45);
    /// Protocol family not supported
    pub const EPFNOSUPPORT: Self = Self(46);
    /// Address family not supported by protocol
    pub const EAFNOSUPPORT: Self = Self(47);
    /// Address already in use
    pub const EADDRINUSE: Self = Self(48);
    /// Cannot assign requested address
    pub const EADDRNOTAVAIL: Self = Self(49);
    /// Network is down
    pub const ENETDOWN: Self = Self(50);
    /// Network is unreachable
    pub const ENETUNREACH: Self = Self(51);
    /// Net dropped connection because of reset
    pub const ENETRESET: Self = Self(52);
    /// Software caused connection abort
    pub const ECONNABORTED: Self = Self(53);
    /// Connection reset by peer
    pub const ECONNRESET: Self = Self(54);
    /// No buffer space available
    pub const ENOBUFS: Self = Self(55);
    /// Transport endpoint is already connected
    pub const EISCONN: Self = Self(56);
    /// Transport endpoint is not connected
    pub const ENOTCONN: Self = Self(57);
    /// No send after transport endpoint shutdown
    pub const ESHUTDOWN: Self = Self(58);
    /// Too many references: cannot splice
    pub const ETOOMANYREFS: Self = Self(59);
    /// Connection timed out
    pub const ETIMEDOUT: Self = Self(60);
    /// Connection refused
    pub const ECONNREFUSED: Self = Self(61);
    /// Too many symbolic links encountered
    pub const ELOOP: Self = Self(62);
    /// File name too long
    pub const ENAMETOOLONG: Self = Self(63);
    /// Host is down
    pub const EHOSTDOWN: Self = Self(64);
    /// No route to host
    pub const EHOSTUNREACH: Self = Self(65);
    /// Directory not empty
    pub const ENOTEMPTY: Self = Self(66);
    /// SUNOS: Too many processes
    pub const EPROCLIM: Self = Self(67);
    /// Too many users
    pub const EUSERS: Self = Self(68);
    /// Quota exceeded
    pub const EDQUOT: Self = Self(69);
    /// Stale file handle
    pub const ESTALE: Self = Self(70);
    /// Object is remote
    pub const EREMOTE: Self = Self(71);
    /// Device not a stream
    pub const ENOSTR: Self = Self(72);
    /// Timer expired
    pub const ETIME: Self = Self(73);
    /// Out of streams resources
    pub const ENOSR: Self = Self(74);
    /// No message of desired type
    pub const ENOMSG: Self = Self(75);
    /// Not a data message
    pub const EBADMSG: Self = Self(76);
    /// Identifier removed
    pub const EIDRM: Self = Self(77);
    /// Resource deadlock would occur
    pub const EDEADLK: Self = Self(78);
    /// No record locks available
    pub const ENOLCK: Self = Self(79);
    /// Machine is not on the network
    pub const ENONET: Self = Self(80);
    /// SunOS: Too many lvls of remote in path
    pub const ERREMOTE: Self = Self(81);
    /// Link has been severed
    pub const ENOLINK: Self = Self(82);
    /// Advertise error
    pub const EADV: Self = Self(83);
    /// Srmount error
    pub const ESRMNT: Self = Self(84);
    /// Communication error on send
    pub const ECOMM: Self = Self(85);
    /// Protocol error
    pub const EPROTO: Self = Self(86);
    /// Multihop attempted
    pub const EMULTIHOP: Self = Self(87);
    /// RFS specific error
    pub const EDOTDOT: Self = Self(88);
    /// Remote address changed
    pub const EREMCHG: Self = Self(89);
    /// Function not implemented
    pub const ENOSYS: Self = Self(90);
    /// Streams pipe error
    pub const ESTRPIPE: Self = Self(91);
    /// Value too large for defined data type
    pub const EOVERFLOW: Self = Self(92);
    /// File descriptor in bad state
    pub const EBADFD: Self = Self(93);
    /// Channel number out of range
    pub const ECHRNG: Self = Self(94);
    /// Level 2 not synchronized
    pub const EL2NSYNC: Self = Self(95);
    /// Level 3 halted
    pub const EL3HLT: Self = Self(96);
    /// Level 3 reset
    pub const EL3RST: Self = Self(97);
    /// Link number out of range
    pub const ELNRNG: Self = Self(98);
    /// Protocol driver not attached
    pub const EUNATCH: Self = Self(99);
    /// No CSI structure available
    pub const ENOCSI: Self = Self(100);
    /// Level 2 halted
    pub const EL2HLT: Self = Self(101);
    /// Invalid exchange
    pub const EBADE: Self = Self(102);
    /// Invalid request descriptor
    pub const EBADR: Self = Self(103);
    /// Exchange full
    pub const EXFULL: Self = Self(104);
    /// No anode
    pub const ENOANO: Self = Self(105);
    /// Invalid request code
    pub const EBADRQC: Self = Self(106);
    /// Invalid slot
    pub const EBADSLT: Self = Self(107);
    /// File locking deadlock error
    pub const EDEADLOCK: Self = Self(108);
    /// Bad font file format
    pub const EBFONT: Self = Self(109);
    /// Cannot exec a shared library directly
    pub const ELIBEXEC: Self = Self(110);
    /// No data available
    pub const ENODATA: Self = Self(111);
    /// Accessing a corrupted shared library
    pub const ELIBBAD: Self = Self(112);
    /// Package not installed
    pub const ENOPKG: Self = Self(113);
    /// Can not access a needed shared library
    pub const ELIBACC: Self = Self(114);
    /// Name not unique on network
    pub const ENOTUNIQ: Self = Self(115);
    /// Interrupted syscall should be restarted
    pub const ERESTART: Self = Self(116);
    /// Structure needs cleaning
    pub const EUCLEAN: Self = Self(117);
    /// Not a XENIX named type file
    pub const ENOTNAM: Self = Self(118);
    /// No XENIX semaphores available
    pub const ENAVAIL: Self = Self(119);
    /// Is a named type file
    pub const EISNAM: Self = Self(120);
    /// Remote I/O error
    pub const EREMOTEIO: Self = Self(121);
    /// Illegal byte sequence
    pub const EILSEQ: Self = Self(122);
    /// Atmpt to link in too many shared libs
    pub const ELIBMAX: Self = Self(123);
    /// .lib section in a.out corrupted
    pub const ELIBSCN: Self = Self(124);
    /// No medium found
    pub const ENOMEDIUM: Self = Self(125);
    /// Wrong medium type
    pub const EMEDIUMTYPE: Self = Self(126);
    /// Operation Cancelled
    pub const ECANCELED: Self = Self(127);
    /// Required key not available
    pub const ENOKEY: Self = Self(128);
    /// Key has expired
    pub const EKEYEXPIRED: Self = Self(129);
    /// Key has been revoked
    pub const EKEYREVOKED: Self = Self(130);
    /// Key was rejected by service
    pub const EKEYREJECTED: Self = Self(131);
    /// Owner died
    pub const EOWNERDEAD: Self = Self(132);
    /// State not recoverable
    pub const ENOTRECOVERABLE: Self = Self(133);
    /// Operation not possible due to RF-kill
    pub const ERFKILL: Self = Self(134);
    /// Memory page has hardware error
    pub const EHWPOISON: Self = Self(135);
    /// Restart syscall
    pub const ERESTARTSYS: Self = Self(512);
    /// Restart if no interrupt
    pub const ERESTARTNOINTR: Self = Self(513);
    /// restart if no handler..
    pub const ERESTARTNOHAND: Self = Self(514);
    /// No ioctl command
    pub const ENOIOCTLCMD: Self = Self(515);
    /// restart by calling sys_restart_syscall
    pub const ERESTART_RESTARTBLOCK: Self = Self(516);
    /// Driver requests probe retry
    pub const EPROBE_DEFER: Self = Self(517);
    /// open found a stale dentry
    pub const EOPENSTALE: Self = Self(518);
    /// Parameter not supported
    pub const ENOPARAM: Self = Self(519);
    /// Illegal NFS file handle
    pub const EBADHANDLE: Self = Self(521);
    /// Update synchronization mismatch
    pub const ENOTSYNC: Self = Self(522);
    /// Cookie is stale
    pub const EBADCOOKIE: Self = Self(523);
    /// Operation is not supported
    pub const ENOTSUPP: Self = Self(524);
    /// Buffer or request is too small
    pub const ETOOSMALL: Self = Self(525);
    /// An untranslatable error occurred
    pub const ESERVERFAULT: Self = Self(526);
    /// Type not supported by server
    pub const EBADTYPE: Self = Self(527);
    /// Request initiated, but will not complete before timeout
    pub const EJUKEBOX: Self = Self(528);
    /// iocb queued, will get completion event
    pub const EIOCBQUEUED: Self = Self(529);
    /// conflict with recalled state
    pub const ERECALLCONFLICT: Self = Self(530);
    /// NFS file lock reclaim refused
    pub const ENOGRACE: Self = Self(531);
    /// Alias for EAGAIN
    pub const EWOULDBLOCK: Self = Self::EAGAIN;

    pub const MIN: i32 = 1;
    pub const MAX: i32 = 531;

    #[cfg(feature = "iter")]
    const ALL: [i32; 153] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72,
        73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95,
        96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114,
        115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132,
        133, 134, 135, 512, 513, 514, 515, 516, 517, 518, 519, 521, 522, 523, 524, 525, 526, 527,
        528, 529, 530, 531,
    ];

    pub(crate) fn name_and_description(&self) -> Option<(&'static str, &'static str)> {
        match self.0 {
            1 => Some(("EPERM", "Operation not permitted")),
            2 => Some(("ENOENT", "No such file or directory")),
            3 => Some(("ESRCH", "No such process")),
            4 => Some(("EINTR", "Interrupted system call")),
            5 => Some(("EIO", "I/O error")),
            6 => Some(("ENXIO", "No such device or address")),
            7 => Some(("E2BIG", "Argument list too long")),
            8 => Some(("ENOEXEC", "Exec format error")),
            9 => Some(("EBADF", "Bad file number")),
            10 => Some(("ECHILD", "No child processes")),
            11 => Some(("EAGAIN", "Try again")),
            12 => Some(("ENOMEM", "Out of memory")),
            13 => Some(("EACCES", "Permission denied")),
            14 => Some(("EFAULT", "Bad address")),
            15 => Some(("ENOTBLK", "Block device required")),
            16 => Some(("EBUSY", "Device or resource busy")),
            17 => Some(("EEXIST", "File exists")),
            18 => Some(("EXDEV", "Cross-device link")),
            19 => Some(("ENODEV", "No such device")),
            20 => Some(("ENOTDIR", "Not a directory")),
            21 => Some(("EISDIR", "Is a directory")),
            22 => Some(("EINVAL", "Invalid argument")),
            23 => Some(("ENFILE", "File table overflow")),
            24 => Some(("EMFILE", "Too many open files")),
            25 => Some(("ENOTTY", "Not a typewriter")),
            26 => Some(("ETXTBSY", "Text file busy")),
            27 => Some(("EFBIG", "File too large")),
            28 => Some(("ENOSPC", "No space left on device")),
            29 => Some(("ESPIPE", "Illegal seek")),
            30 => Some(("EROFS", "Read-only file system")),
            31 => Some(("EMLINK", "Too many links")),
            32 => Some(("EPIPE", "Broken pipe")),
            33 => Some(("EDOM", "Math argument out of domain of func")),
            34 => Some(("ERANGE", "Math result not representable")),
            36 => Some(("EINPROGRESS", "Operation now in progress")),
            37 => Some(("EALREADY", "Operation already in progress")),
            38 => Some(("ENOTSOCK", "Socket operation on non-socket")),
            39 => Some(("EDESTADDRREQ", "Destination address required")),
            40 => Some(("EMSGSIZE", "Message too long")),
            41 => Some(("EPROTOTYPE", "Protocol wrong type for socket")),
            42 => Some(("ENOPROTOOPT", "Protocol not available")),
            43 => Some(("EPROTONOSUPPORT", "Protocol not supported")),
            44 => Some(("ESOCKTNOSUPPORT", "Socket type not supported")),
            45 => Some(("EOPNOTSUPP", "Op not supported on transport endpoint")),
            46 => Some(("EPFNOSUPPORT", "Protocol family not supported")),
            47 => Some(("EAFNOSUPPORT", "Address family not supported by protocol")),
            48 => Some(("EADDRINUSE", "Address already in use")),
            49 => Some(("EADDRNOTAVAIL", "Cannot assign requested address")),
            50 => Some(("ENETDOWN", "Network is down")),
            51 => Some(("ENETUNREACH", "Network is unreachable")),
            52 => Some(("ENETRESET", "Net dropped connection because of reset")),
            53 => Some(("ECONNABORTED", "Software caused connection abort")),
            54 => Some(("ECONNRESET", "Connection reset by peer")),
            55 => Some(("ENOBUFS", "No buffer space available")),
            56 => Some(("EISCONN", "Transport endpoint is already connected")),
            57 => Some(("ENOTCONN", "Transport endpoint is not connected")),
            58 => Some(("ESHUTDOWN", "No send after transport endpoint shutdown")),
            59 => Some(("ETOOMANYREFS", "Too many references: cannot splice")),
            60 => Some(("ETIMEDOUT", "Connection timed out")),
            61 => Some(("ECONNREFUSED", "Connection refused")),
            62 => Some(("ELOOP", "Too many symbolic links encountered")),
            63 => Some(("ENAMETOOLONG", "File name too long")),
            64 => Some(("EHOSTDOWN", "Host is down")),
            65 => Some(("EHOSTUNREACH", "No route to host")),
            66 => Some(("ENOTEMPTY", "Directory not empty")),
            67 => Some(("EPROCLIM", "SUNOS: Too many processes")),
            68 => Some(("EUSERS", "Too many users")),
            69 => Some(("EDQUOT", "Quota exceeded")),
            70 => Some(("ESTALE", "Stale file handle")),
            71 => Some(("EREMOTE", "Object is remote")),
            72 => Some(("ENOSTR", "Device not a stream")),
            73 => Some(("ETIME", "Timer expired")),
            74 => Some(("ENOSR", "Out of streams resources")),
            75 => Some(("ENOMSG", "No message of desired type")),
            76 => Some(("EBADMSG", "Not a data message")),
            77 => Some(("EIDRM", "Identifier removed")),
            78 => Some(("EDEADLK", "Resource deadlock would occur")),
            79 => Some(("ENOLCK", "No record locks available")),
            80 => Some(("ENONET", "Machine is not on the network")),
            81 => Some(("ERREMOTE", "SunOS: Too many lvls of remote in path")),
            82 => Some(("ENOLINK", "Link has been severed")),
            83 => Some(("EADV", "Advertise error")),
            84 => Some(("ESRMNT", "Srmount error")),
            85 => Some(("ECOMM", "Communication error on send")),
            86 => Some(("EPROTO", "Protocol error")),
            87 => Some(("EMULTIHOP", "Multihop attempted")),
            88 => Some(("EDOTDOT", "RFS specific error")),
            89 => Some(("EREMCHG", "Remote address changed")),
            90 => Some(("ENOSYS", "Function not implemented")),
            91 => Some(("ESTRPIPE", "Streams pipe error")),
            92 => Some(("EOVERFLOW", "Value too large for defined data type")),
            93 => Some(("EBADFD", "File descriptor in bad state")),
            94 => Some(("ECHRNG", "Channel number out of range")),
            95 => Some(("EL2NSYNC", "Level 2 not synchronized")),
            96 => Some(("EL3HLT", "Level 3 halted")),
            97 => Some(("EL3RST", "Level 3 reset")),
            98 => Some(("ELNRNG", "Link number out of range")),
            99 => Some(("EUNATCH", "Protocol driver not attached")),
            100 => Some(("ENOCSI", "No CSI structure available")),
            101 => Some(("EL2HLT", "Level 2 halted")),
            102 => Some(("EBADE", "Invalid exchange")),
            103 => Some(("EBADR", "Invalid request descriptor")),
            104 => Some(("EXFULL", "Exchange full")),
            105 => Some(("ENOANO", "No anode")),
            106 => Some(("EBADRQC", "Invalid request code")),
            107 => Some(("EBADSLT", "Invalid slot")),
            108 => Some(("EDEADLOCK", "File locking deadlock error")),
            109 => Some(("EBFONT", "Bad font file format")),
            110 => Some(("ELIBEXEC", "Cannot exec a shared library directly")),
            111 => Some(("ENODATA", "No data available")),
            112 => Some(("ELIBBAD", "Accessing a corrupted shared library")),
            113 => Some(("ENOPKG", "Package not installed")),
            114 => Some(("ELIBACC", "Can not access a needed shared library")),
            115 => Some(("ENOTUNIQ", "Name not unique on network")),
            116 => Some(("ERESTART", "Interrupted syscall should be restarted")),
            117 => Some(("EUCLEAN", "Structure needs cleaning")),
            118 => Some(("ENOTNAM", "Not a XENIX named type file")),
            119 => Some(("ENAVAIL", "No XENIX semaphores available")),
            120 => Some(("EISNAM", "Is a named type file")),
            121 => Some(("EREMOTEIO", "Remote I/O error")),
            122 => Some(("EILSEQ", "Illegal byte sequence")),
            123 => Some(("ELIBMAX", "Atmpt to link in too many shared libs")),
            124 => Some(("ELIBSCN", ".lib section in a.out corrupted")),
            125 => Some(("ENOMEDIUM", "No medium found")),
            126 => Some(("EMEDIUMTYPE", "Wrong medium type")),
            127 => Some(("ECANCELED", "Operation Cancelled")),
            128 => Some(("ENOKEY", "Required key not available")),
            129 => Some(("EKEYEXPIRED", "Key has expired")),
            130 => Some(("EKEYREVOKED", "Key has been revoked")),
            131 => Some(("EKEYREJECTED", "Key was rejected by service")),
            132 => Some(("EOWNERDEAD", "Owner died")),
            133 => Some(("ENOTRECOVERABLE", "State not recoverable")),
            134 => Some(("ERFKILL", "Operation not possible due to RF-kill")),
            135 => Some(("EHWPOISON", "Memory page has hardware error")),
            512 => Some(("ERESTARTSYS", "Restart syscall")),
            513 => Some(("ERESTARTNOINTR", "Restart if no interrupt")),
            514 => Some(("ERESTARTNOHAND", "restart if no handler..")),
            515 => Some(("ENOIOCTLCMD", "No ioctl command")),
            516 => Some((
                "ERESTART_RESTARTBLOCK",
                "restart by calling sys_restart_syscall",
            )),
            517 => Some(("EPROBE_DEFER", "Driver requests probe retry")),
            518 => Some(("EOPENSTALE", "open found a stale dentry")),
            519 => Some(("ENOPARAM", "Parameter not supported")),
            521 => Some(("EBADHANDLE", "Illegal NFS file handle")),
            522 => Some(("ENOTSYNC", "Update synchronization mismatch")),
            523 => Some(("EBADCOOKIE", "Cookie is stale")),
            524 => Some(("ENOTSUPP", "Operation is not supported")),
            525 => Some(("ETOOSMALL", "Buffer or request is too small")),
            526 => Some(("ESERVERFAULT", "An untranslatable error occurred")),
            527 => Some(("EBADTYPE", "Type not supported by server")),
            528 => Some((
                "EJUKEBOX",
                "Request initiated, but will not complete before timeout",
            )),
            529 => Some(("EIOCBQUEUED", "iocb queued, will get completion event")),
            530 => Some(("ERECALLCONFLICT", "conflict with recalled state")),
            531 => Some(("ENOGRACE", "NFS file lock reclaim refused")),
            _ => None,
        }
    }
}
