//! More ergonomic bitflags

#![deny(missing_docs)]

/// Generates a bitflags type, wrapping a given primitive integer type.
///
/// # Example
///
/// The following macro invocation will create a `struct Foo`:
///
/// ```ignore
/// #[macro_use] extern crate new_bitflags;
///
/// new_bitflags!{
///     pub flags Foo: u32 {
///         const flag_a = 1 << 0;
///         const flag_b = 1 << 1;
///         const flag_c = 1 << 2;
///     }
/// }
///
/// impl Foo {
///     pub fn flag_abc() -> Foo {
///         Foo::flag_a() |
///         Foo::flag_b() |
///         Foo::flag_c()
///     }
/// }
///
/// fn main() {
///     let f1 = Foo::flag_a() | Foo::flag_c();
///     let f2 = Foo::flag_b() | Foo::flag_c();
///
///     assert_eq!((f1 | f2), Foo::flag_abc()); // union
///     assert_eq!((f1 & f2), Foo::flag_c());   // intersection
///     assert_eq!((f1 - f2), Foo::flag_a());   // difference
///     assert_eq!(!f2,       Foo::flag_a());   // complement
/// }
/// ```
///
/// The generated `struct` can be extended with type and trait `impl`s.
///
/// ```ignore
/// impl Foo {
///     pub fn is_flag_a(&self) -> bool {
///         self.contains(Foo::flag_a())
///     }
/// }
/// ```
///
/// # Visibility
///
/// The visibility of the generated `struct` can be controlled within the
/// invocation of `new_bitflags!`
///
/// ```ignore
/// #[macro_use] extern crate new_bitflags;
///
/// mod example {
///     // `struct Public` will be visible outside this module.
///     new_bitflags!{
///         pub flags Public: u32 {
///             // ...
///         }
///     }
///
///     // `struct Private` will not be visible outside this module.
///     new_bitflags!{
///         flags Private: u32 {
///             // ...
///         }
///     }
/// }
/// ```
///
/// # Trait implementations
///
/// Generated `struct` types will have derived implementations of the following
/// traits: `Copy`, `Clone`, `Hash`, `PartialEq`, `Eq`, `PartialOrd`, and `Ord`.
///
/// The traits `Extend` and `FromIterator` are implemented for sequences of
/// `Self` and `&Self`.
///
/// The `Debug` trait implementation will display the set of named flags contained
/// in a set.
///
/// # Operators
///
/// The following operators are implemented for generated `struct` types:
///
/// * `BitOr` and `BitOrAssign` perform union
/// * `BitAnd` and `BitAndAssign` perform intersection
/// * `BitXor` and `BitXorAssign` perform toggle
/// * `Sub` and `SubAssign` perform set difference
/// * `Not` performs set complement
///
/// # Methods
///
/// The following methods are implemented for generated `struct` types:
///
/// * `fn from_bits(bits) -> Option<Self>` converts from underlying bits,
///    checking that all bits correspond to defined flags.
/// * `fn from_bits_truncate(bits) -> Self` converts from underlying bits,
///   truncating any bits that do not correspond to defined flags.
/// * `fn bits(&self) -> bits` returns the underlying bits
/// * `fn contains(&self, other: Self) -> bool` returns whether the set
///   contains all flags present in `other`
/// * `fn clear(&mut self)` clears all flags on the set
/// * `fn all() -> Self` returns all defined flags
/// * `fn empty() -> Self` returns an empty set
/// * `fn is_all(&self) -> bool` returns whether the set contains all flags
/// * `fn is_empty(&self) -> bool` returns whether the set is empty
/// * `fn intersects(&self, other: Self) -> bool` returns whether any flags
///   are common between `self` and `other`.
/// * `fn insert(&mut self, other: Self)` inserts all flags in `other`
/// * `fn remove(&mut self, other: Self)` removes all flags in `other`
/// * `fn toggle(&mut self, other: Self)` toggles all flags in `other`
/// * `fn set(&mut self, other: Self, value: bool)` sets or removes all flags
///   in `other`, depending on boolean `value`
///
/// Additionally, for each defined flag, a static method of signature
/// `fn() -> Self` is defined, returning a set containing only the named flag.
#[macro_export]
macro_rules! new_bitflags {
    ( $(#[$attr:meta])* pub flags $name:ident : $inner:ty
            { $( $(#[$flag_attr:meta])* const $flag:ident = $value:expr ; )* } ) => {
        #[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
        $(#[$attr])*
        pub struct $name($inner);

        new_bitflags!{ @_impl $name : $inner
            { $( $(#[$flag_attr])* const $flag = $value ; )* } }
    };
    ( $(#[$attr:meta])* flags $name:ident : $inner:ty
            { $( $(#[$flag_attr:meta])* const $flag:ident = $value:expr ; )* } ) => {
        #[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
        $(#[$attr])*
        struct $name($inner);

        new_bitflags!{ @_impl $name : $inner
            { $( $(#[$flag_attr])* const $flag = $value ; )* } }
    };
    ( @_impl $name:ident : $inner:ty
            { $( $(#[$flag_attr:meta])* const $flag:ident = $value:expr ; )* } ) => {
        #[allow(dead_code)]
        impl $name {
            /// Converts from a set of bits, only if all set bits correspond
            /// to defined flags.
            #[inline]
            pub fn from_bits(bits: $inner) -> ::std::option::Option<$name> {
                if (bits & !$name::all().bits()) == 0 {
                    Some($name(bits))
                } else {
                    None
                }
            }

            /// Converts from a set of bits, truncating any invalid bits.
            #[inline]
            pub fn from_bits_truncate(bits: $inner) -> $name {
                $name(bits) & $name::all()
            }

            /// Returns the underlying bits.
            #[inline]
            pub fn bits(&self) -> $inner {
                self.0
            }

            /// Returns whether the given flags are set in `self`.
            #[inline]
            pub fn contains(&self, flag: $name) -> bool {
                *self & flag == flag
            }

            /// Zeroes all bits.
            #[inline]
            pub fn clear(&mut self) {
                self.0 = 0;
            }

            /// Returns the set of all defined flags.
            #[inline]
            pub fn all() -> $name {
                $name(0 $( | $value )*)
            }

            /// Returns an empty set.
            #[inline]
            pub fn empty() -> $name {
                $name(0)
            }

            /// Returns whether all defined flags are set in `self`.
            #[inline]
            pub fn is_all(&self) -> bool {
                self == $name::all()
            }

            /// Returns whether no defined flags are set in `self`.
            #[inline]
            pub fn is_empty(&self) -> bool {
                self.bits() == 0
            }

            /// Returns whether any flags contained in `other` are also
            /// contained in `self`.
            #[inline]
            pub fn intersects(&self, other: $name) -> bool {
                !(*self & other).is_empty()
            }

            /// Inserts a set of flags in-place.
            #[inline]
            pub fn insert(&mut self, other: $name) {
                self.0 |= other.0;
            }

            /// Removes a set of flags in-place.
            #[inline]
            pub fn remove(&mut self, other: $name) {
                self.0 &= !other.0;
            }

            /// Toggles a set of flags in-place.
            #[inline]
            pub fn toggle(&mut self, other: $name) {
                self.0 ^= other.0;
            }

            /// Inserts or removes the given set of flags.
            #[inline]
            pub fn set(&mut self, other: $name, value: bool) {
                if value {
                    self.insert(other);
                } else {
                    self.remove(other);
                }
            }

            $( $(#[$flag_attr])*
            #[inline]
            pub fn $flag() -> $name {
                $name($value)
            } )*
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut flags = *self;
                let mut _first = true;

                f.write_str(concat!(stringify!($name), "("))?;

                $( if !$name::$flag().is_empty() && flags.contains($name::$flag()) {
                    if !_first {
                        f.write_str(" | ")?;
                    }
                    _first = false;

                    flags.remove($name::$flag());
                    f.write_str(stringify!($flag))?;
                } )*

                f.write_str(")")
            }
        }

        impl ::std::iter::Extend<$name> for $name {
            fn extend<I: ::std::iter::IntoIterator<Item=$name>>(&mut self, iter: I) {
                for flag in iter {
                    self.insert(flag);
                }
            }
        }

        impl<'a> ::std::iter::Extend<&'a $name> for $name {
            fn extend<I: ::std::iter::IntoIterator<Item=&'a $name>>(&mut self, iter: I) {
                for flag in iter {
                    self.insert(*flag);
                }
            }
        }

        impl ::std::iter::FromIterator<$name> for $name {
            fn from_iter<I: IntoIterator<Item=$name>>(iter: I) -> $name {
                let mut flags = $name::empty();
                flags.extend(iter);
                flags
            }
        }

        impl<'a> ::std::iter::FromIterator<&'a $name> for $name {
            fn from_iter<I: IntoIterator<Item=&'a $name>>(iter: I) -> $name {
                let mut flags = $name::empty();
                flags.extend(iter);
                flags
            }
        }

        impl ::std::ops::BitOr for $name {
            type Output = $name;

            #[inline]
            fn bitor(self, rhs: $name) -> $name {
                $name(self.0 | rhs.0)
            }
        }

        impl ::std::ops::BitOrAssign for $name {
            #[inline]
            fn bitor_assign(&mut self, rhs: $name) {
                self.0 |= rhs.0;
            }
        }

        impl ::std::ops::BitAnd for $name {
            type Output = $name;

            #[inline]
            fn bitand(self, rhs: $name) -> $name {
                $name(self.0 & rhs.0)
            }
        }

        impl ::std::ops::BitAndAssign for $name {
            #[inline]
            fn bitand_assign(&mut self, rhs: $name) {
                self.0 &= rhs.0;
            }
        }

        impl ::std::ops::BitXor for $name {
            type Output = $name;

            #[inline]
            fn bitxor(self, rhs: $name) -> $name {
                $name(self.0 ^ rhs.0)
            }
        }

        impl ::std::ops::BitXorAssign for $name {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $name) {
                self.0 ^= rhs.0;
            }
        }

        impl ::std::ops::Not for $name {
            type Output = $name;

            #[inline]
            fn not(self) -> $name {
                self ^ $name::all()
            }
        }

        impl ::std::ops::Sub for $name {
            type Output = $name;

            #[inline]
            fn sub(mut self, rhs: $name) -> $name {
                self.remove(rhs);
                self
            }
        }

        impl ::std::ops::SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: $name) {
                self.remove(rhs);
            }
        }

        impl<'a> PartialEq<&'a $name> for $name {
            #[inline]
            fn eq(&self, rhs: &&$name) -> bool { *self == **rhs }
            #[inline]
            fn ne(&self, rhs: &&$name) -> bool { *self != **rhs }
        }

        impl<'a> PartialEq<$name> for &'a $name {
            #[inline]
            fn eq(&self, rhs: &$name) -> bool { **self == *rhs }
            #[inline]
            fn ne(&self, rhs: &$name) -> bool { **self != *rhs }
        }
    }
}
