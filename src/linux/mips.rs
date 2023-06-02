//! Error numbers for arch `mips`.

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
    /// No message of desired type
    pub const ENOMSG: Self = Self(35);
    /// Identifier removed
    pub const EIDRM: Self = Self(36);
    /// Channel number out of range
    pub const ECHRNG: Self = Self(37);
    /// Level 2 not synchronized
    pub const EL2NSYNC: Self = Self(38);
    /// Level 3 halted
    pub const EL3HLT: Self = Self(39);
    /// Level 3 reset
    pub const EL3RST: Self = Self(40);
    /// Link number out of range
    pub const ELNRNG: Self = Self(41);
    /// Protocol driver not attached
    pub const EUNATCH: Self = Self(42);
    /// No CSI structure available
    pub const ENOCSI: Self = Self(43);
    /// Level 2 halted
    pub const EL2HLT: Self = Self(44);
    /// Resource deadlock would occur
    pub const EDEADLK: Self = Self(45);
    /// No record locks available
    pub const ENOLCK: Self = Self(46);
    /// Invalid exchange
    pub const EBADE: Self = Self(50);
    /// Invalid request descriptor
    pub const EBADR: Self = Self(51);
    /// Exchange full
    pub const EXFULL: Self = Self(52);
    /// No anode
    pub const ENOANO: Self = Self(53);
    /// Invalid request code
    pub const EBADRQC: Self = Self(54);
    /// Invalid slot
    pub const EBADSLT: Self = Self(55);
    /// File locking deadlock error
    pub const EDEADLOCK: Self = Self(56);
    /// Bad font file format
    pub const EBFONT: Self = Self(59);
    /// Device not a stream
    pub const ENOSTR: Self = Self(60);
    /// No data available
    pub const ENODATA: Self = Self(61);
    /// Timer expired
    pub const ETIME: Self = Self(62);
    /// Out of streams resources
    pub const ENOSR: Self = Self(63);
    /// Machine is not on the network
    pub const ENONET: Self = Self(64);
    /// Package not installed
    pub const ENOPKG: Self = Self(65);
    /// Object is remote
    pub const EREMOTE: Self = Self(66);
    /// Link has been severed
    pub const ENOLINK: Self = Self(67);
    /// Advertise error
    pub const EADV: Self = Self(68);
    /// Srmount error
    pub const ESRMNT: Self = Self(69);
    /// Communication error on send
    pub const ECOMM: Self = Self(70);
    /// Protocol error
    pub const EPROTO: Self = Self(71);
    /// RFS specific error
    pub const EDOTDOT: Self = Self(73);
    /// Multihop attempted
    pub const EMULTIHOP: Self = Self(74);
    /// Not a data message
    pub const EBADMSG: Self = Self(77);
    /// File name too long
    pub const ENAMETOOLONG: Self = Self(78);
    /// Value too large for defined data type
    pub const EOVERFLOW: Self = Self(79);
    /// Name not unique on network
    pub const ENOTUNIQ: Self = Self(80);
    /// File descriptor in bad state
    pub const EBADFD: Self = Self(81);
    /// Remote address changed
    pub const EREMCHG: Self = Self(82);
    /// Can not access a needed shared library
    pub const ELIBACC: Self = Self(83);
    /// Accessing a corrupted shared library
    pub const ELIBBAD: Self = Self(84);
    /// .lib section in a.out corrupted
    pub const ELIBSCN: Self = Self(85);
    /// Attempting to link in too many shared libraries
    pub const ELIBMAX: Self = Self(86);
    /// Cannot exec a shared library directly
    pub const ELIBEXEC: Self = Self(87);
    /// Illegal byte sequence
    pub const EILSEQ: Self = Self(88);
    /// Function not implemented
    pub const ENOSYS: Self = Self(89);
    /// Too many symbolic links encountered
    pub const ELOOP: Self = Self(90);
    /// Interrupted system call should be restarted
    pub const ERESTART: Self = Self(91);
    /// Streams pipe error
    pub const ESTRPIPE: Self = Self(92);
    /// Directory not empty
    pub const ENOTEMPTY: Self = Self(93);
    /// Too many users
    pub const EUSERS: Self = Self(94);
    /// Socket operation on non-socket
    pub const ENOTSOCK: Self = Self(95);
    /// Destination address required
    pub const EDESTADDRREQ: Self = Self(96);
    /// Message too long
    pub const EMSGSIZE: Self = Self(97);
    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Self = Self(98);
    /// Protocol not available
    pub const ENOPROTOOPT: Self = Self(99);
    /// Protocol not supported
    pub const EPROTONOSUPPORT: Self = Self(120);
    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Self = Self(121);
    /// Operation not supported on transport endpoint
    pub const EOPNOTSUPP: Self = Self(122);
    /// Protocol family not supported
    pub const EPFNOSUPPORT: Self = Self(123);
    /// Address family not supported by protocol
    pub const EAFNOSUPPORT: Self = Self(124);
    /// Address already in use
    pub const EADDRINUSE: Self = Self(125);
    /// Cannot assign requested address
    pub const EADDRNOTAVAIL: Self = Self(126);
    /// Network is down
    pub const ENETDOWN: Self = Self(127);
    /// Network is unreachable
    pub const ENETUNREACH: Self = Self(128);
    /// Network dropped connection because of reset
    pub const ENETRESET: Self = Self(129);
    /// Software caused connection abort
    pub const ECONNABORTED: Self = Self(130);
    /// Connection reset by peer
    pub const ECONNRESET: Self = Self(131);
    /// No buffer space available
    pub const ENOBUFS: Self = Self(132);
    /// Transport endpoint is already connected
    pub const EISCONN: Self = Self(133);
    /// Transport endpoint is not connected
    pub const ENOTCONN: Self = Self(134);
    /// Structure needs cleaning
    pub const EUCLEAN: Self = Self(135);
    /// Not a XENIX named type file
    pub const ENOTNAM: Self = Self(137);
    /// No XENIX semaphores available
    pub const ENAVAIL: Self = Self(138);
    /// Is a named type file
    pub const EISNAM: Self = Self(139);
    /// Remote I/O error
    pub const EREMOTEIO: Self = Self(140);
    /// Reserved
    pub const EINIT: Self = Self(141);
    /// Error 142
    pub const EREMDEV: Self = Self(142);
    /// Cannot send after transport endpoint shutdown
    pub const ESHUTDOWN: Self = Self(143);
    /// Too many references: cannot splice
    pub const ETOOMANYREFS: Self = Self(144);
    /// Connection timed out
    pub const ETIMEDOUT: Self = Self(145);
    /// Connection refused
    pub const ECONNREFUSED: Self = Self(146);
    /// Host is down
    pub const EHOSTDOWN: Self = Self(147);
    /// No route to host
    pub const EHOSTUNREACH: Self = Self(148);
    /// Operation already in progress
    pub const EALREADY: Self = Self(149);
    /// Operation now in progress
    pub const EINPROGRESS: Self = Self(150);
    /// Stale file handle
    pub const ESTALE: Self = Self(151);
    /// AIO operation canceled
    pub const ECANCELED: Self = Self(158);
    /// No medium found
    pub const ENOMEDIUM: Self = Self(159);
    /// Wrong medium type
    pub const EMEDIUMTYPE: Self = Self(160);
    /// Required key not available
    pub const ENOKEY: Self = Self(161);
    /// Key has expired
    pub const EKEYEXPIRED: Self = Self(162);
    /// Key has been revoked
    pub const EKEYREVOKED: Self = Self(163);
    /// Key was rejected by service
    pub const EKEYREJECTED: Self = Self(164);
    /// Owner died
    pub const EOWNERDEAD: Self = Self(165);
    /// State not recoverable
    pub const ENOTRECOVERABLE: Self = Self(166);
    /// Operation not possible due to RF-kill
    pub const ERFKILL: Self = Self(167);
    /// Memory page has hardware error
    pub const EHWPOISON: Self = Self(168);
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
    /// Quota exceeded
    pub const EDQUOT: Self = Self(1133);
    /// Alias for [Self::EAGAIN]
    pub const EWOULDBLOCK: Self = Self::EAGAIN;

    pub const MIN: i32 = 1;
    pub const MAX: i32 = 1133;

    #[cfg(feature = "iter")]
    const ALL: [i32; 153] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 50, 51,
        52, 53, 54, 55, 56, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 73, 74, 77, 78, 79,
        80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 120, 121,
        122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 137, 138, 139, 140,
        141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 158, 159, 160, 161, 162, 163, 164,
        165, 166, 167, 168, 512, 513, 514, 515, 516, 517, 518, 519, 521, 522, 523, 524, 525, 526,
        527, 528, 529, 530, 531, 1133,
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
            35 => Some(("ENOMSG", "No message of desired type")),
            36 => Some(("EIDRM", "Identifier removed")),
            37 => Some(("ECHRNG", "Channel number out of range")),
            38 => Some(("EL2NSYNC", "Level 2 not synchronized")),
            39 => Some(("EL3HLT", "Level 3 halted")),
            40 => Some(("EL3RST", "Level 3 reset")),
            41 => Some(("ELNRNG", "Link number out of range")),
            42 => Some(("EUNATCH", "Protocol driver not attached")),
            43 => Some(("ENOCSI", "No CSI structure available")),
            44 => Some(("EL2HLT", "Level 2 halted")),
            45 => Some(("EDEADLK", "Resource deadlock would occur")),
            46 => Some(("ENOLCK", "No record locks available")),
            50 => Some(("EBADE", "Invalid exchange")),
            51 => Some(("EBADR", "Invalid request descriptor")),
            52 => Some(("EXFULL", "Exchange full")),
            53 => Some(("ENOANO", "No anode")),
            54 => Some(("EBADRQC", "Invalid request code")),
            55 => Some(("EBADSLT", "Invalid slot")),
            56 => Some(("EDEADLOCK", "File locking deadlock error")),
            59 => Some(("EBFONT", "Bad font file format")),
            60 => Some(("ENOSTR", "Device not a stream")),
            61 => Some(("ENODATA", "No data available")),
            62 => Some(("ETIME", "Timer expired")),
            63 => Some(("ENOSR", "Out of streams resources")),
            64 => Some(("ENONET", "Machine is not on the network")),
            65 => Some(("ENOPKG", "Package not installed")),
            66 => Some(("EREMOTE", "Object is remote")),
            67 => Some(("ENOLINK", "Link has been severed")),
            68 => Some(("EADV", "Advertise error")),
            69 => Some(("ESRMNT", "Srmount error")),
            70 => Some(("ECOMM", "Communication error on send")),
            71 => Some(("EPROTO", "Protocol error")),
            73 => Some(("EDOTDOT", "RFS specific error")),
            74 => Some(("EMULTIHOP", "Multihop attempted")),
            77 => Some(("EBADMSG", "Not a data message")),
            78 => Some(("ENAMETOOLONG", "File name too long")),
            79 => Some(("EOVERFLOW", "Value too large for defined data type")),
            80 => Some(("ENOTUNIQ", "Name not unique on network")),
            81 => Some(("EBADFD", "File descriptor in bad state")),
            82 => Some(("EREMCHG", "Remote address changed")),
            83 => Some(("ELIBACC", "Can not access a needed shared library")),
            84 => Some(("ELIBBAD", "Accessing a corrupted shared library")),
            85 => Some(("ELIBSCN", ".lib section in a.out corrupted")),
            86 => Some(("ELIBMAX", "Attempting to link in too many shared libraries")),
            87 => Some(("ELIBEXEC", "Cannot exec a shared library directly")),
            88 => Some(("EILSEQ", "Illegal byte sequence")),
            89 => Some(("ENOSYS", "Function not implemented")),
            90 => Some(("ELOOP", "Too many symbolic links encountered")),
            91 => Some(("ERESTART", "Interrupted system call should be restarted")),
            92 => Some(("ESTRPIPE", "Streams pipe error")),
            93 => Some(("ENOTEMPTY", "Directory not empty")),
            94 => Some(("EUSERS", "Too many users")),
            95 => Some(("ENOTSOCK", "Socket operation on non-socket")),
            96 => Some(("EDESTADDRREQ", "Destination address required")),
            97 => Some(("EMSGSIZE", "Message too long")),
            98 => Some(("EPROTOTYPE", "Protocol wrong type for socket")),
            99 => Some(("ENOPROTOOPT", "Protocol not available")),
            120 => Some(("EPROTONOSUPPORT", "Protocol not supported")),
            121 => Some(("ESOCKTNOSUPPORT", "Socket type not supported")),
            122 => Some((
                "EOPNOTSUPP",
                "Operation not supported on transport endpoint",
            )),
            123 => Some(("EPFNOSUPPORT", "Protocol family not supported")),
            124 => Some(("EAFNOSUPPORT", "Address family not supported by protocol")),
            125 => Some(("EADDRINUSE", "Address already in use")),
            126 => Some(("EADDRNOTAVAIL", "Cannot assign requested address")),
            127 => Some(("ENETDOWN", "Network is down")),
            128 => Some(("ENETUNREACH", "Network is unreachable")),
            129 => Some(("ENETRESET", "Network dropped connection because of reset")),
            130 => Some(("ECONNABORTED", "Software caused connection abort")),
            131 => Some(("ECONNRESET", "Connection reset by peer")),
            132 => Some(("ENOBUFS", "No buffer space available")),
            133 => Some(("EISCONN", "Transport endpoint is already connected")),
            134 => Some(("ENOTCONN", "Transport endpoint is not connected")),
            135 => Some(("EUCLEAN", "Structure needs cleaning")),
            137 => Some(("ENOTNAM", "Not a XENIX named type file")),
            138 => Some(("ENAVAIL", "No XENIX semaphores available")),
            139 => Some(("EISNAM", "Is a named type file")),
            140 => Some(("EREMOTEIO", "Remote I/O error")),
            141 => Some(("EINIT", "Reserved")),
            142 => Some(("EREMDEV", "Error 142")),
            143 => Some(("ESHUTDOWN", "Cannot send after transport endpoint shutdown")),
            144 => Some(("ETOOMANYREFS", "Too many references: cannot splice")),
            145 => Some(("ETIMEDOUT", "Connection timed out")),
            146 => Some(("ECONNREFUSED", "Connection refused")),
            147 => Some(("EHOSTDOWN", "Host is down")),
            148 => Some(("EHOSTUNREACH", "No route to host")),
            149 => Some(("EALREADY", "Operation already in progress")),
            150 => Some(("EINPROGRESS", "Operation now in progress")),
            151 => Some(("ESTALE", "Stale file handle")),
            158 => Some(("ECANCELED", "AIO operation canceled")),
            159 => Some(("ENOMEDIUM", "No medium found")),
            160 => Some(("EMEDIUMTYPE", "Wrong medium type")),
            161 => Some(("ENOKEY", "Required key not available")),
            162 => Some(("EKEYEXPIRED", "Key has expired")),
            163 => Some(("EKEYREVOKED", "Key has been revoked")),
            164 => Some(("EKEYREJECTED", "Key was rejected by service")),
            165 => Some(("EOWNERDEAD", "Owner died")),
            166 => Some(("ENOTRECOVERABLE", "State not recoverable")),
            167 => Some(("ERFKILL", "Operation not possible due to RF-kill")),
            168 => Some(("EHWPOISON", "Memory page has hardware error")),
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
            1133 => Some(("EDQUOT", "Quota exceeded")),
            _ => None,
        }
    }
}
