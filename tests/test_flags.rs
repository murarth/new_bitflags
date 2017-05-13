#[macro_use] extern crate new_bitflags;

new_bitflags!{
    pub flags Foo: u32 {
        const alpha     = 1 << 0;
        const beta      = 1 << 1;
        const nothing   = 0;
    }
}

new_bitflags!{
    #[allow(non_camel_case_types)]
    flags test_attrs: u32 {
        const alpha = 1 << 0;
        const beta  = 1 << 1;
        #[allow(non_snake_case)]
        const I_DONT_KNOW_WHAT_WERE_YELLING_ABOUT = 1 << 2;
    }
}

#[test]
fn test_debug() {
    fn debug<T: ::std::fmt::Debug>(t: T) -> String {
        format!("{:?}", t)
    }

    assert_eq!(debug(Foo::empty()),   "Foo()");
    assert_eq!(debug(Foo::nothing()), "Foo()");
    assert_eq!(debug(Foo::alpha()),   "Foo(alpha)");
    assert_eq!(debug(Foo::beta()),    "Foo(beta)");
    assert_eq!(debug(Foo::all()),     "Foo(alpha | beta)");
}

#[test]
fn test_flags() {
    let mut flags = Foo::empty();

    assert_eq!(flags.contains(Foo::alpha()), false);
    assert_eq!(flags.is_empty(), true);

    flags.insert(Foo::alpha());

    assert_eq!(flags.contains(Foo::alpha()), true);
    assert_eq!(flags.is_empty(), false);

    assert_eq!(Foo::from_bits(0), Some(Foo::empty()));
    assert_eq!(Foo::from_bits(0b01), Some(Foo::alpha()));
    assert_eq!(Foo::from_bits(0b10), Some(Foo::beta()));
    assert_eq!(Foo::from_bits(0b11), Some(Foo::all()));
    assert_eq!(Foo::from_bits(0b100), None);

    assert_eq!(Foo::from_bits_truncate(0), Foo::empty());
    assert_eq!(Foo::from_bits_truncate(0b01), Foo::alpha());
    assert_eq!(Foo::from_bits_truncate(0b10), Foo::beta());
    assert_eq!(Foo::from_bits_truncate(0b11), Foo::all());
    assert_eq!(Foo::from_bits_truncate(0b100), Foo::empty());
}

#[test]
fn test_zero_bit() {
    assert_eq!(Foo::nothing().is_empty(), true);
    assert_eq!(Foo::empty().contains(Foo::nothing()), true);
}

#[test]
fn test_bits() {
    assert_eq!(Foo::empty().bits(), 0);
    assert_eq!(Foo::alpha().bits(), 0b01);
    assert_eq!(Foo::beta().bits(),  0b10);
    assert_eq!(Foo::all().bits(),   0b11);
}

#[test]
fn test_contains() {
    assert_eq!(Foo::empty().contains(Foo::alpha()), false);
    assert_eq!(Foo::alpha().contains(Foo::alpha()), true);
    assert_eq!(Foo::beta().contains(Foo::alpha()), false);
}

#[test]
fn test_clear() {
    let mut f = Foo::alpha();

    f.clear();
    assert_eq!(f.is_empty(), true);
}

#[test]
fn test_all() {
    let f = Foo::all();

    assert_eq!(f.contains(Foo::alpha()), true);
    assert_eq!(f.contains(Foo::beta()), true);
    assert_eq!(f.contains(Foo::all()), true);
}

#[test]
fn test_empty() {
    let f = Foo::empty();

    assert_eq!(f.contains(Foo::alpha()), false);
    assert_eq!(f.contains(Foo::beta()), false);
}

#[test]
fn test_is_all() {
    assert_eq!(Foo::all().is_all(), true);
    assert_eq!(Foo::empty().is_all(), false);
    assert_eq!(Foo::alpha().is_all(), false);
    assert_eq!(Foo::beta().is_all(), false);
    assert_eq!((Foo::alpha() | Foo::beta()).is_all(), true);
}

#[test]
fn test_is_empty() {
    assert_eq!(Foo::all().is_empty(), false);
    assert_eq!(Foo::empty().is_empty(), true);
    assert_eq!(Foo::alpha().is_empty(), false);
    assert_eq!(Foo::beta().is_empty(), false);
}

#[test]
fn test_intersects() {
    assert_eq!(Foo::alpha().intersects(Foo::beta()), false);
    assert_eq!(Foo::all().intersects(Foo::alpha()), true);
    assert_eq!(Foo::alpha().intersects(Foo::all()), true);
}

#[test]
fn test_insert() {
    let mut f = Foo::empty();

    assert_eq!(f.contains(Foo::alpha()), false);
    f.insert(Foo::alpha());
    assert_eq!(f.contains(Foo::alpha()), true);

    assert_eq!(f.contains(Foo::beta()), false);
    f.insert(Foo::beta());
    assert_eq!(f.contains(Foo::beta()), true);
}

#[test]
fn test_remove() {
    let mut f = Foo::all();

    assert_eq!(f.contains(Foo::alpha()), true);
    f.remove(Foo::alpha());
    assert_eq!(f.contains(Foo::alpha()), false);

    assert_eq!(f.contains(Foo::beta()), true);
    f.remove(Foo::beta());
    assert_eq!(f.contains(Foo::beta()), false);
}

#[test]
fn test_toggle() {
    let mut f = Foo::empty();

    f.toggle(Foo::alpha());
    assert_eq!(f, Foo::alpha());

    f.toggle(Foo::all());
    assert_eq!(f, Foo::beta());
}

#[test]
fn test_set() {
    let mut f = Foo::empty();

    f.set(Foo::alpha(), true);
    assert_eq!(f.contains(Foo::alpha()), true);

    let mut f = Foo::all();

    f.set(Foo::beta(), false);
    assert_eq!(f.contains(Foo::beta()), false);
}

#[test]
fn test_extend() {
    let mut f = Foo::empty();

    f.extend(&[Foo::alpha(), Foo::beta()][..]);

    assert_eq!(f, Foo::all());
}

#[test]
fn test_from_iterator() {
    let f = [Foo::alpha(), Foo::beta()].iter().collect::<Foo>();

    assert_eq!(f, Foo::all());
}

// Example from macro docs
mod example {
    new_bitflags!{
        pub flags Foo: u32 {
            const flag_a = 1 << 0;
            const flag_b = 1 << 1;
            const flag_c = 1 << 2;
        }
    }

    impl Foo {
        pub fn flag_abc() -> Foo {
            Foo::flag_a() |
            Foo::flag_b() |
            Foo::flag_c()
        }

        pub fn is_flag_a(&self) -> bool {
            self.contains(Foo::flag_a())
        }
    }

    #[test]
    fn test_example() {
        let f1 = Foo::flag_a() | Foo::flag_c();
        let f2 = Foo::flag_b() | Foo::flag_c();

        assert_eq!((f1 | f2), Foo::flag_abc());
        assert_eq!((f1 & f2), Foo::flag_c());
        assert_eq!((f1 - f2), Foo::flag_a());
        assert_eq!(!f2,       Foo::flag_a());

        assert_eq!(f1.is_flag_a(), true);
    }
}

#[test]
fn test_ops() {
    let mut flags = Foo::empty();

    flags |= Foo::alpha();
    flags &= Foo::beta();
    flags ^= Foo::all();
}

mod private {
    mod private_flags {
        new_bitflags!{
            flags Bar: u32 {
                const abc = 1;
            }
        }
    }
}

mod test_docs {
    new_bitflags!{
        /// Document `Foo`
        ///
        /// Important stuff here.
        flags Foo: u32 {
            /// Document `a`
            const a = 1 << 0;
            /// Document `b`
            ///
            /// These remarks are essential to understanding `b`.
            const b = 1 << 1;
        }
    }
}
