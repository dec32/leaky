use crate::Leak;

macro_rules! delegate_iter {
    ($name:ident ($($arg:ident: $ty:ty),*)) => {
        #[inline(always)]
        pub fn $name(self, $($arg: $ty),*) -> impl Iterator<Item = Self> {
            self.0.$name($($arg),*).map(Self)
        }
    };
}

macro_rules! delegate_opt_pair {
    ($name:ident ($($arg:ident: $ty:ty),*)) => {
        #[inline(always)]
        pub fn $name(self, $($arg: $ty),*) -> Option<(Self, Self)> {
            self.0.$name($($arg),*).map(|(a, b)|(Self(a), Self(b)))
        }
    };
}

macro_rules! delegate_pair {
    ($name:ident ($($arg:ident: $ty:ty),*)) => {
        #[inline(always)]
        pub fn $name(self, $($arg: $ty),*) -> (Self, Self) {
            let (a, b) = self.0.$name($($arg),*);
            (Self(a), Self(b))
        }
    };
}

impl Leak<str> {
    /// Returns the inner `&'static str`
    pub fn as_str(self) -> &'static str {
        self.0
    }

    delegate_iter! { split(pat: &str) }
    delegate_iter! { rsplit(pat: &str) }
    delegate_iter! { splitn(n: usize, pat: &str) }
    delegate_iter! { rsplitn(n: usize, pat: &str) }
    delegate_iter! { split_inclusive(pat: &str) }
    delegate_iter! { split_terminator(pat: &str) }
    delegate_iter! { split_whitespace() }
    delegate_iter! { split_ascii_whitespace() }

    delegate_pair! { split_at(mid: usize) }

    delegate_opt_pair! { split_once(pat: &str) }
    delegate_opt_pair! { rsplit_once(pat: &str) }
    delegate_opt_pair! { split_at_checked(mid: usize) }
}
