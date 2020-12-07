pub trait Hkt {
    type Member<T>: Mirror<T = T, Family = Self>;
}

pub trait Mirror {
    type T;
    type Family: Hkt;
    fn as_member(self) -> <Self::Family as Hkt>::Member<Self::T>;
}
macro_rules! Plug {
    { $mirror:ty, $target:ty } => { <<$mirror as Mirror>::Family as Hkt>::Member<$target> };
}


pub trait Hkt2 {
    type Member<A, B>: Mirror2<A = A, B = B, Family = Self>;
}

pub trait Mirror2: Sized {
    type A;
    type B;
    type Family: Hkt2;
    fn as_member(self) -> <Self::Family as Hkt2>::Member<Self::A, Self::B>;
}

pub trait Functor: Mirror {
    fn map<B, F: Fn(Self::T) -> B>(self, f: F) -> Plug![Self, B];
    fn flat_map<B, F: Fn(Self::T) -> Plug![Self, B]>(self, f: F) -> Plug![Self, B];
}
