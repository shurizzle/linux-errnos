macro_rules! def_errno {
    () => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
        pub struct Errno(pub(crate) i32);

        impl Errno {
            #[inline]
            pub fn new(num: i32) -> Self {
                Self(num)
            }

            #[inline]
            pub fn into_raw(self) -> i32 {
                self.0
            }

            #[inline]
            pub fn is_valid(&self) -> bool {
                self.0 < 4096
            }

            #[inline(always)]
            pub fn from_ret(value: usize) -> ::core::result::Result<usize, Errno> {
                if value > -4096isize as usize {
                    ::core::result::Result::Err(Self(-(value as i32)))
                } else {
                    ::core::result::Result::Ok(value)
                }
            }

            pub fn name(&self) -> ::core::option::Option<&'static str> {
                self.name_and_description().map(|x| x.0)
            }

            pub fn description(&self) -> ::core::option::Option<&'static str> {
                self.name_and_description().map(|x| x.1)
            }

            #[cfg(feature = "std")]
            #[inline]
            pub fn from_io_error(err: ::std::io::Error) -> ::core::option::Option<Self> {
                err.raw_os_error().map(Self)
            }
        }

        impl ::core::fmt::Display for Errno {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self.name_and_description() {
                    Some((name, desc)) => {
                        write!(f, "{} {name} ({desc})", -self.0)
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

        #[cfg(feature = "std")]
        impl ::core::convert::From<Errno> for ::std::io::Error {
            #[inline]
            fn from(value: Errno) -> Self {
                ::std::io::Error::from_raw_os_error(value.into_raw())
            }
        }

        #[cfg(feature = "std")]
        impl ::std::error::Error for Errno {}
    };
}

pub(crate) use def_errno;
