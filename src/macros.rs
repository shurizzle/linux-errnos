macro_rules! def_errno {
    () => {
        /// Error number representation.
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
        pub struct Errno(pub(crate) i32);

        impl Errno {
            /// Returns a new `Errno` from the given integer.
            #[inline]
            pub fn new(num: i32) -> Self {
                Self(num)
            }

            /// Converts `Errno` into the under underlining integer.
            #[inline]
            pub fn into_raw(self) -> i32 {
                self.0
            }

            /// Returns `true` if the error code is in valid range (lower than 4096)
            #[inline]
            pub fn is_valid(&self) -> bool {
                self.0 < 4096
            }

            /// Returns a new `Errno` from a syscall's result.
            #[inline(always)]
            pub fn from_ret(value: usize) -> ::core::result::Result<usize, Errno> {
                if value > -4096isize as usize {
                    ::core::result::Result::Err(Self(-(value as i32)))
                } else {
                    ::core::result::Result::Ok(value)
                }
            }

            /// Returns the name of the error if it's known. Generally the name of the constant.
            pub fn name(&self) -> ::core::option::Option<&'static str> {
                self.name_and_description().map(|x| x.0)
            }

            /// Returns a description of the error if it's known.
            pub fn description(&self) -> ::core::option::Option<&'static str> {
                self.name_and_description().map(|x| x.1)
            }

            /// Returns a new `Errno` if the given error is generated from a system error.
            /// None otherwise.
            #[cfg(any(doc, feature = "std"))]
            #[inline]
            pub fn from_io_error(err: ::std::io::Error) -> ::core::option::Option<Self> {
                err.raw_os_error().map(Self)
            }

            /// Returns a new `Errno` from last OS error.
            #[cfg(any(doc, all(feature = "std", not(feature = "libc"))))]
            #[inline]
            pub fn last_os_error() -> Self {
                Self::from_io_error(::std::io::Error::last_os_error()).unwrap()
            }

            /// Returns a new `Errno` from last OS error.
            #[cfg(any(doc, feature = "libc"))]
            pub fn last_os_error() -> Self {
                Self(unsafe { *libc::__errno_location() })
            }

            /// Returns an iterator `ErrnoIter` over all the known error numbers.
            #[cfg(any(doc, feature = "iter"))]
            #[inline]
            pub fn iter() -> ErrnoIter {
                ErrnoIter(Self::ALL.iter())
            }
        }

        impl ::core::fmt::Display for Errno {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self.name_and_description() {
                    Some((name, desc)) => {
                        write!(f, "{} {} ({})", -self.0, name, desc)
                    }
                    None => {
                        if self.is_valid() {
                            write!(f, "{}", -self.0)
                        } else {
                            write!(f, "Unknown errno {:#x}", self.0)
                        }
                    }
                }
            }
        }

        impl ::core::fmt::Debug for Errno {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self.name() {
                    ::core::option::Option::Some(name) => f.write_str(name),
                    ::core::option::Option::None => write!(f, "Errno({})", self.0),
                }
            }
        }

        #[cfg(any(doc, feature = "std"))]
        impl ::core::convert::From<Errno> for ::std::io::Error {
            #[inline]
            fn from(value: Errno) -> Self {
                ::std::io::Error::from_raw_os_error(value.into_raw())
            }
        }

        #[cfg(any(doc, feature = "std"))]
        impl ::std::error::Error for Errno {}

        #[cfg(any(doc, feature = "no_std_io-compat"))]
        impl ::core::convert::From<Errno> for ::no_std_io::io::Error {
            #[inline]
            fn from(value: Errno) -> Self {
                use ::no_std_io::io::ErrorKind::*;

                match value {
                    // Errno::E2BIG => ArgumentListTooLong.into(),
                    Errno::EADDRINUSE => AddrInUse.into(),
                    Errno::EADDRNOTAVAIL => AddrNotAvailable.into(),
                    // Errno::EBUSY => ResourceBusy.into(),
                    Errno::ECONNABORTED => ConnectionAborted.into(),
                    Errno::ECONNREFUSED => ConnectionRefused.into(),
                    Errno::ECONNRESET => ConnectionReset.into(),
                    // Errno::EDEADLK => Deadlock.into(),
                    // Errno::EDQUOT => FilesystemQuotaExceeded.into(),
                    Errno::EEXIST => AlreadyExists.into(),
                    // Errno::EFBIG => FileTooLarge.into(),
                    // Errno::EHOSTUNREACH => HostUnreachable.into(),
                    Errno::EINTR => Interrupted.into(),
                    Errno::EINVAL => InvalidInput.into(),
                    // Errno::EISDIR => IsADirectory.into(),
                    // Errno::ELOOP => FilesystemLoop.into(),
                    Errno::ENOENT => NotFound.into(),
                    // Errno::ENOMEM => OutOfMemory.into(),
                    // Errno::ENOSPC => StorageFull.into(),
                    // Errno::ENOSYS => Unsupported.into(),
                    // Errno::EMLINK => TooManyLinks.into(),
                    // Errno::ENAMETOOLONG => InvalidFilename.into(),
                    // Errno::ENETDOWN => NetworkDown.into(),
                    // Errno::ENETUNREACH => NetworkUnreachable.into(),
                    Errno::ENOTCONN => NotConnected.into(),
                    // Errno::ENOTDIR => NotADirectory.into(),
                    // Errno::ENOTEMPTY => DirectoryNotEmpty.into(),
                    Errno::EPIPE => BrokenPipe.into(),
                    // Errno::EROFS => ReadOnlyFilesystem.into(),
                    // Errno::ESPIPE => NotSeekable.into(),
                    // Errno::ESTALE => StaleNetworkFileHandle.into(),
                    Errno::ETIMEDOUT => TimedOut.into(),
                    // Errno::ETXTBSY => ExecutableFileBusy.into(),
                    // Errno::EXDEV => CrossesDevices.into(),
                    Errno::EACCES | Errno::EPERM => PermissionDenied.into(),

                    // These two constants can have the same value on some systems,
                    // but different values on others, so we can't use a match
                    // clause
                    x if x == Errno::EAGAIN || x == Errno::EWOULDBLOCK => WouldBlock.into(),

                    x => ::no_std_io::io::Error::new(
                        Uncategorized,
                        x.description().unwrap_or("Unknown error"),
                    ),
                }
            }
        }

        #[cfg(any(doc, feature = "core2-compat"))]
        impl ::no_std_io::error::Error for Errno {}

        #[cfg(any(doc, feature = "iter"))]
        /// Iterator over all possible error numbers.
        pub struct ErrnoIter(::core::slice::Iter<'static, i32>);

        #[cfg(any(doc, feature = "iter"))]
        impl ::core::iter::Iterator for ErrnoIter {
            type Item = Errno;

            #[inline]
            fn next(&mut self) -> ::core::option::Option<Self::Item> {
                self.0.next().copied().map(Errno)
            }

            #[inline]
            fn size_hint(&self) -> (usize, ::core::option::Option<usize>) {
                self.0.size_hint()
            }

            #[inline]
            fn count(self) -> usize
            where
                Self: Sized,
            {
                self.0.count()
            }

            #[inline]
            fn last(self) -> ::core::option::Option<Self::Item>
            where
                Self: Sized,
            {
                self.0.last().copied().map(Errno)
            }

            fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
                self.0.nth(n).copied().map(Errno)
            }
        }

        #[cfg(any(doc, feature = "iter"))]
        impl ::core::iter::ExactSizeIterator for ErrnoIter {
            #[inline]
            fn len(&self) -> usize {
                self.0.len()
            }
        }

        #[cfg(any(doc, feature = "iter"))]
        impl ::core::iter::DoubleEndedIterator for ErrnoIter {
            #[inline]
            fn next_back(&mut self) -> ::core::option::Option<Self::Item> {
                self.0.next_back().copied().map(Errno)
            }

            #[inline]
            fn nth_back(&mut self, n: usize) -> ::core::option::Option<Self::Item> {
                self.0.nth_back(n).copied().map(Errno)
            }
        }
    };
}

pub(crate) use def_errno;
