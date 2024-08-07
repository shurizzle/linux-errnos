//! Error numbers for arch `parisc`.

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
    /// Illegal byte sequence
    pub const EILSEQ: Self = Self(47);
    /// Machine is not on the network
    pub const ENONET: Self = Self(50);
    /// No data available
    pub const ENODATA: Self = Self(51);
    /// Timer expired
    pub const ETIME: Self = Self(52);
    /// Out of streams resources
    pub const ENOSR: Self = Self(53);
    /// Device not a stream
    pub const ENOSTR: Self = Self(54);
    /// Package not installed
    pub const ENOPKG: Self = Self(55);
    /// Link has been severed
    pub const ENOLINK: Self = Self(57);
    /// Advertise error
    pub const EADV: Self = Self(58);
    /// Srmount error
    pub const ESRMNT: Self = Self(59);
    /// Communication error on send
    pub const ECOMM: Self = Self(60);
    /// Protocol error
    pub const EPROTO: Self = Self(61);
    /// Multihop attempted
    pub const EMULTIHOP: Self = Self(64);
    /// RFS specific error
    pub const EDOTDOT: Self = Self(66);
    /// Not a data message
    pub const EBADMSG: Self = Self(67);
    /// Too many users
    pub const EUSERS: Self = Self(68);
    /// Quota exceeded
    pub const EDQUOT: Self = Self(69);
    /// Stale file handle
    pub const ESTALE: Self = Self(70);
    /// Object is remote
    pub const EREMOTE: Self = Self(71);
    /// Value too large for defined data type
    pub const EOVERFLOW: Self = Self(72);
    /// Invalid exchange
    pub const EBADE: Self = Self(160);
    /// Invalid request descriptor
    pub const EBADR: Self = Self(161);
    /// Exchange full
    pub const EXFULL: Self = Self(162);
    /// No anode
    pub const ENOANO: Self = Self(163);
    /// Invalid request code
    pub const EBADRQC: Self = Self(164);
    /// Invalid slot
    pub const EBADSLT: Self = Self(165);
    /// Bad font file format
    pub const EBFONT: Self = Self(166);
    /// Name not unique on network
    pub const ENOTUNIQ: Self = Self(167);
    /// File descriptor in bad state
    pub const EBADFD: Self = Self(168);
    /// Remote address changed
    pub const EREMCHG: Self = Self(169);
    /// Can not access a needed shared library
    pub const ELIBACC: Self = Self(170);
    /// Accessing a corrupted shared library
    pub const ELIBBAD: Self = Self(171);
    /// .lib section in a.out corrupted
    pub const ELIBSCN: Self = Self(172);
    /// Attempting to link in too many shared libraries
    pub const ELIBMAX: Self = Self(173);
    /// Cannot exec a shared library directly
    pub const ELIBEXEC: Self = Self(174);
    /// Interrupted system call should be restarted
    pub const ERESTART: Self = Self(175);
    /// Streams pipe error
    pub const ESTRPIPE: Self = Self(176);
    /// Structure needs cleaning
    pub const EUCLEAN: Self = Self(177);
    /// Not a XENIX named type file
    pub const ENOTNAM: Self = Self(178);
    /// No XENIX semaphores available
    pub const ENAVAIL: Self = Self(179);
    /// Is a named type file
    pub const EISNAM: Self = Self(180);
    /// Remote I/O error
    pub const EREMOTEIO: Self = Self(181);
    /// No medium found
    pub const ENOMEDIUM: Self = Self(182);
    /// Wrong medium type
    pub const EMEDIUMTYPE: Self = Self(183);
    /// Required key not available
    pub const ENOKEY: Self = Self(184);
    /// Key has expired
    pub const EKEYEXPIRED: Self = Self(185);
    /// Key has been revoked
    pub const EKEYREVOKED: Self = Self(186);
    /// Key was rejected by service
    pub const EKEYREJECTED: Self = Self(187);
    /// Socket operation on non-socket
    pub const ENOTSOCK: Self = Self(216);
    /// Destination address required
    pub const EDESTADDRREQ: Self = Self(217);
    /// Message too long
    pub const EMSGSIZE: Self = Self(218);
    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Self = Self(219);
    /// Protocol not available
    pub const ENOPROTOOPT: Self = Self(220);
    /// Protocol not supported
    pub const EPROTONOSUPPORT: Self = Self(221);
    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Self = Self(222);
    /// Operation not supported on transport endpoint
    pub const EOPNOTSUPP: Self = Self(223);
    /// Protocol family not supported
    pub const EPFNOSUPPORT: Self = Self(224);
    /// Address family not supported by protocol
    pub const EAFNOSUPPORT: Self = Self(225);
    /// Address already in use
    pub const EADDRINUSE: Self = Self(226);
    /// Cannot assign requested address
    pub const EADDRNOTAVAIL: Self = Self(227);
    /// Network is down
    pub const ENETDOWN: Self = Self(228);
    /// Network is unreachable
    pub const ENETUNREACH: Self = Self(229);
    /// Network dropped connection because of reset
    pub const ENETRESET: Self = Self(230);
    /// Software caused connection abort
    pub const ECONNABORTED: Self = Self(231);
    /// Connection reset by peer
    pub const ECONNRESET: Self = Self(232);
    /// No buffer space available
    pub const ENOBUFS: Self = Self(233);
    /// Transport endpoint is already connected
    pub const EISCONN: Self = Self(234);
    /// Transport endpoint is not connected
    pub const ENOTCONN: Self = Self(235);
    /// Cannot send after transport endpoint shutdown
    pub const ESHUTDOWN: Self = Self(236);
    /// Too many references: cannot splice
    pub const ETOOMANYREFS: Self = Self(237);
    /// Connection timed out
    pub const ETIMEDOUT: Self = Self(238);
    /// Connection refused
    pub const ECONNREFUSED: Self = Self(239);
    /// Host is down
    pub const EHOSTDOWN: Self = Self(241);
    /// No route to host
    pub const EHOSTUNREACH: Self = Self(242);
    /// Operation already in progress
    pub const EALREADY: Self = Self(244);
    /// Operation now in progress
    pub const EINPROGRESS: Self = Self(245);
    /// Directory not empty
    pub const ENOTEMPTY: Self = Self(247);
    /// File name too long
    pub const ENAMETOOLONG: Self = Self(248);
    /// Too many symbolic links encountered
    pub const ELOOP: Self = Self(249);
    /// Function not implemented
    pub const ENOSYS: Self = Self(251);
    /// aio request was canceled before complete (POSIX.4 / HPUX)
    pub const ECANCELLED: Self = Self(253);
    /// Owner died
    pub const EOWNERDEAD: Self = Self(254);
    /// State not recoverable
    pub const ENOTRECOVERABLE: Self = Self(255);
    /// Operation not possible due to RF-kill
    pub const ERFKILL: Self = Self(256);
    /// Memory page has hardware error
    pub const EHWPOISON: Self = Self(257);
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
    /// Alias for [Self::ECANCELLED]
    pub const ECANCELED: Self = Self::ECANCELLED;
    /// Alias for [Self::EDEADLK]
    pub const EDEADLOCK: Self = Self::EDEADLK;
    /// Alias for [Self::ECONNREFUSED]
    pub const EREFUSED: Self = Self::ECONNREFUSED;
    /// Alias for [Self::EAGAIN]
    pub const EWOULDBLOCK: Self = Self::EAGAIN;

    pub const MIN: i32 = 1;
    pub const MAX: i32 = 531;

    #[cfg(feature = "iter")]
    const ALL: [i32; 150] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 50,
        51, 52, 53, 54, 55, 57, 58, 59, 60, 61, 64, 66, 67, 68, 69, 70, 71, 72, 160, 161, 162, 163,
        164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181,
        182, 183, 184, 185, 186, 187, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227,
        228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 241, 242, 244, 245, 247, 248,
        249, 251, 253, 254, 255, 256, 257, 512, 513, 514, 515, 516, 517, 518, 519, 521, 522, 523,
        524, 525, 526, 527, 528, 529, 530, 531,
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
            47 => Some(("EILSEQ", "Illegal byte sequence")),
            50 => Some(("ENONET", "Machine is not on the network")),
            51 => Some(("ENODATA", "No data available")),
            52 => Some(("ETIME", "Timer expired")),
            53 => Some(("ENOSR", "Out of streams resources")),
            54 => Some(("ENOSTR", "Device not a stream")),
            55 => Some(("ENOPKG", "Package not installed")),
            57 => Some(("ENOLINK", "Link has been severed")),
            58 => Some(("EADV", "Advertise error")),
            59 => Some(("ESRMNT", "Srmount error")),
            60 => Some(("ECOMM", "Communication error on send")),
            61 => Some(("EPROTO", "Protocol error")),
            64 => Some(("EMULTIHOP", "Multihop attempted")),
            66 => Some(("EDOTDOT", "RFS specific error")),
            67 => Some(("EBADMSG", "Not a data message")),
            68 => Some(("EUSERS", "Too many users")),
            69 => Some(("EDQUOT", "Quota exceeded")),
            70 => Some(("ESTALE", "Stale file handle")),
            71 => Some(("EREMOTE", "Object is remote")),
            72 => Some(("EOVERFLOW", "Value too large for defined data type")),
            160 => Some(("EBADE", "Invalid exchange")),
            161 => Some(("EBADR", "Invalid request descriptor")),
            162 => Some(("EXFULL", "Exchange full")),
            163 => Some(("ENOANO", "No anode")),
            164 => Some(("EBADRQC", "Invalid request code")),
            165 => Some(("EBADSLT", "Invalid slot")),
            166 => Some(("EBFONT", "Bad font file format")),
            167 => Some(("ENOTUNIQ", "Name not unique on network")),
            168 => Some(("EBADFD", "File descriptor in bad state")),
            169 => Some(("EREMCHG", "Remote address changed")),
            170 => Some(("ELIBACC", "Can not access a needed shared library")),
            171 => Some(("ELIBBAD", "Accessing a corrupted shared library")),
            172 => Some(("ELIBSCN", ".lib section in a.out corrupted")),
            173 => Some(("ELIBMAX", "Attempting to link in too many shared libraries")),
            174 => Some(("ELIBEXEC", "Cannot exec a shared library directly")),
            175 => Some(("ERESTART", "Interrupted system call should be restarted")),
            176 => Some(("ESTRPIPE", "Streams pipe error")),
            177 => Some(("EUCLEAN", "Structure needs cleaning")),
            178 => Some(("ENOTNAM", "Not a XENIX named type file")),
            179 => Some(("ENAVAIL", "No XENIX semaphores available")),
            180 => Some(("EISNAM", "Is a named type file")),
            181 => Some(("EREMOTEIO", "Remote I/O error")),
            182 => Some(("ENOMEDIUM", "No medium found")),
            183 => Some(("EMEDIUMTYPE", "Wrong medium type")),
            184 => Some(("ENOKEY", "Required key not available")),
            185 => Some(("EKEYEXPIRED", "Key has expired")),
            186 => Some(("EKEYREVOKED", "Key has been revoked")),
            187 => Some(("EKEYREJECTED", "Key was rejected by service")),
            216 => Some(("ENOTSOCK", "Socket operation on non-socket")),
            217 => Some(("EDESTADDRREQ", "Destination address required")),
            218 => Some(("EMSGSIZE", "Message too long")),
            219 => Some(("EPROTOTYPE", "Protocol wrong type for socket")),
            220 => Some(("ENOPROTOOPT", "Protocol not available")),
            221 => Some(("EPROTONOSUPPORT", "Protocol not supported")),
            222 => Some(("ESOCKTNOSUPPORT", "Socket type not supported")),
            223 => Some((
                "EOPNOTSUPP",
                "Operation not supported on transport endpoint",
            )),
            224 => Some(("EPFNOSUPPORT", "Protocol family not supported")),
            225 => Some(("EAFNOSUPPORT", "Address family not supported by protocol")),
            226 => Some(("EADDRINUSE", "Address already in use")),
            227 => Some(("EADDRNOTAVAIL", "Cannot assign requested address")),
            228 => Some(("ENETDOWN", "Network is down")),
            229 => Some(("ENETUNREACH", "Network is unreachable")),
            230 => Some(("ENETRESET", "Network dropped connection because of reset")),
            231 => Some(("ECONNABORTED", "Software caused connection abort")),
            232 => Some(("ECONNRESET", "Connection reset by peer")),
            233 => Some(("ENOBUFS", "No buffer space available")),
            234 => Some(("EISCONN", "Transport endpoint is already connected")),
            235 => Some(("ENOTCONN", "Transport endpoint is not connected")),
            236 => Some(("ESHUTDOWN", "Cannot send after transport endpoint shutdown")),
            237 => Some(("ETOOMANYREFS", "Too many references: cannot splice")),
            238 => Some(("ETIMEDOUT", "Connection timed out")),
            239 => Some(("ECONNREFUSED", "Connection refused")),
            241 => Some(("EHOSTDOWN", "Host is down")),
            242 => Some(("EHOSTUNREACH", "No route to host")),
            244 => Some(("EALREADY", "Operation already in progress")),
            245 => Some(("EINPROGRESS", "Operation now in progress")),
            247 => Some(("ENOTEMPTY", "Directory not empty")),
            248 => Some(("ENAMETOOLONG", "File name too long")),
            249 => Some(("ELOOP", "Too many symbolic links encountered")),
            251 => Some(("ENOSYS", "Function not implemented")),
            253 => Some((
                "ECANCELLED",
                "aio request was canceled before complete (POSIX.4 / HPUX)",
            )),
            254 => Some(("EOWNERDEAD", "Owner died")),
            255 => Some(("ENOTRECOVERABLE", "State not recoverable")),
            256 => Some(("ERFKILL", "Operation not possible due to RF-kill")),
            257 => Some(("EHWPOISON", "Memory page has hardware error")),
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
