use super::{covariant::*, derived::applicative::*, hkt::*};

pub trait Traversable: Covariant {
    fn foreach<App: Applicative, A, B: Clone, F: Fn(A) -> App::Member<B>>(
        fa: Self::Member<A>,
        f: F,
    ) -> App::Member<Self::Member<B>>;
}

pub trait TraverseSyntax: Mirror + Sized
where
    Self::Family: Traversable,
{
    fn traverse<AppB: Mirror, F: Fn(Self::T) -> AppB>(
        self,
        f: F,
    ) -> <AppB::Family as Hkt>::Member<<Self::Family as Hkt>::Member<AppB::T>>
    where
        AppB::Family: Applicative,
        AppB::T: Clone,
    {
        <Self::Family as Traversable>::foreach::<AppB::Family, _, _, _>(
            self.as_member(),
            move |t| f(t).as_member(),
        )
    }
}
impl<F: Traversable, T: Mirror<Family = F>> TraverseSyntax for T {}

pub trait SequenceSyntax: Mirror + Sized
where
    Self::Family: Traversable,
    Self::T: Mirror,
    <Self::T as Mirror>::Family: Applicative,
    <Self::T as Mirror>::T: Clone,
{
    fn sequence(
        self,
    ) -> <<Self::T as Mirror>::Family as Hkt>::Member<
        <Self::Family as Hkt>::Member<<Self::T as Mirror>::T>,
    > {
        self.traverse(move |x| x)
    }
}
impl<
        App: Applicative,
        Tr: Traversable,
        A: Clone,
        AppA: Mirror<Family = App, T = A>,
        TrAppA: Mirror<Family = Tr, T = AppA>,
    > SequenceSyntax for TrAppA
{
}
