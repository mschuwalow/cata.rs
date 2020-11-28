// use super::{associative_flatten::*, covariant::*, derived::monad::*, hkt::*};
use super::{
    associative_both::*,
    associative_either::*,
    associative_flatten::*,
    bifunctor::*,
    covariant::*,
    derived::{applicative::*, monad::*},
    hkt::*,
    identity_both::*,
    right_covariant::*,
    traversable::*,
};
use std::marker::PhantomData;

// vec
impl<T> Mirror for Vec<T> {
    type Family = VectorFamily;
    type T = T;

    fn as_member(self) -> <Self::Family as Hkt>::Member<Self::T> {
        self
    }

    fn as_member_(&self) -> &<Self::Family as Hkt>::Member<Self::T> {
        self
    }
}
pub struct VectorFamily;
impl Hkt for VectorFamily {
    type Member<T> = Vec<T>;
}
impl Covariant for VectorFamily {
    fn map<A, B, F: FnMut(A) -> B>(fa: Vec<A>, mut f: F) -> Vec<B> {
        let mut acc = Vec::new();
        for a in fa {
            acc.push(f(a));
        }
        acc
    }
}
impl AssociativeFlatten for VectorFamily {
    fn flatten<A>(ffa: Vec<Vec<A>>) -> Vec<A> {
        let mut acc = Vec::new();
        for fa in ffa {
            for a in fa {
                acc.push(a);
            }
        }
        acc
    }
}
impl AssociativeBoth for VectorFamily {
    fn both<A: Clone, B: Clone>(
        fa: Vec<A>,
        fb: Vec<B>,
    ) -> Self::Member<(A, B)> {
        fa.flat_map(move |a| fb.clone().map(move |b| (a.clone(), b)))
    }
}
impl IdentityBoth for VectorFamily {
    fn any() -> Self::Member<()> {
        vec![()]
    }
}
impl Traversable for VectorFamily {
    fn foreach<App: Applicative, A, B: Clone, F: FnMut(A) -> App::Member<B>>(
        fa: Vec<A>,
        mut f: F,
    ) -> App::Member<Vec<B>> {
        let init = App::pure(Vec::new());
        let result = fa.into_iter().fold(init, move |app_acc, a| {
            let app_b = f(a);
            App::both(app_acc.as_member(), app_b.as_member()).map(
                |(mut acc, b)| {
                    acc.push(b);
                    acc
                },
            )
        });
        result
    }
}

// either
#[derive(Eq, PartialEq, Debug)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
}
impl<A: Clone, B: Clone> Clone for Either<A, B> {
    fn clone(&self) -> Self {
        match self {
            Either::Left(l) => Either::Left(l.clone()),
            Either::Right(r) => Either::Right(r.clone()),
        }
    }
}
impl<L, R> Mirror for Either<L, R> {
    type Family = EitherFamily<L>;
    type T = R;

    fn as_member(self) -> Either<L, R> {
        self
    }

    fn as_member_(&self) -> &Either<L, R> {
        self
    }
}
impl<L, R> Mirror2 for Either<L, R> {
    type A = L;
    type B = R;
    type Family = EitherFamily2;

    fn as_member(self) -> <Self::Family as Hkt2>::Member<Self::A, Self::B> {
        self
    }

    fn as_member_(&self) -> &<Self::Family as Hkt2>::Member<Self::A, Self::B> {
        self
    }
}

pub struct EitherFamily<L>(PhantomData<L>);
impl<L> Hkt for EitherFamily<L> {
    type Member<R> = Either<L, R>;
}
pub struct EitherFamily2;
impl<L> Covariant for EitherFamily<L> {
    fn map<A, B, F: FnMut(A) -> B>(fa: Either<L, A>, mut f: F) -> Either<L, B> {
        match fa {
            Either::Left(a) => Either::Left(a),
            Either::Right(b) => Either::Right(f(b)),
        }
    }
}
impl<L> AssociativeFlatten for EitherFamily<L> {
    fn flatten<A>(ffa: Either<L, Either<L, A>>) -> Either<L, A> {
        match ffa {
            Either::Left(l) => Either::Left(l),
            Either::Right(fa) => fa,
        }
    }
}
impl<L> AssociativeBoth for EitherFamily<L> {
    fn both<A, B>(fa: Either<L, A>, fb: Either<L, B>) -> Either<L, (A, B)> {
        match fa {
            Either::Left(l) => Either::Left(l),
            Either::Right(lr) => match fb {
                Either::Left(l) => Either::Left(l),
                Either::Right(rr) => Either::Right((lr, rr)),
            },
        }
    }
}
impl<L> AssociativeEither for EitherFamily<L> {
    fn either<A, B>(
        fa: Either<L, A>,
        fb: Either<L, B>,
    ) -> Either<L, Either<A, B>> {
        match (fa, fb) {
            (Either::Left(_), Either::Left(lr)) => Either::Left(lr),
            (Either::Right(lr), _) => Either::Right(Either::Left(lr)),
            (_, Either::Right(rr)) => Either::Right(Either::Right(rr)),
        }
    }
}
impl<L> IdentityBoth for EitherFamily<L> {
    fn any() -> Self::Member<()> {
        Either::Right(())
    }
}
impl<L: Clone> Traversable for EitherFamily<L> {
    fn foreach<App: Applicative, A, B, F: FnMut(A) -> App::Member<B>>(
        fa: Self::Member<A>,
        mut f: F,
    ) -> App::Member<Self::Member<B>> {
        match fa {
            Either::Left(l) => App::pure(l).map(move |l| Either::Left(l)),
            Either::Right(a) => f(a).map(move |b| Either::Right(b)),
        }
    }
}
impl Hkt2 for EitherFamily2 {
    type Member<A, B> = Either<A, B>;
}
impl RightCovariant for EitherFamily2 {
    fn right_map<A, B, C, F: FnMut(B) -> C>(
        ab: Self::Member<A, B>,
        f: F,
    ) -> Self::Member<A, C> {
        ab.map(f)
    }
}
impl BiFunctor for EitherFamily2 {
    fn bimap<A, B, C, D, F: FnMut(A) -> C, G: FnMut(B) -> D>(
        ab: Self::Member<A, B>,
        mut f: F,
        mut g: G,
    ) -> Self::Member<C, D> {
        match ab {
            Either::Left(l) => Either::Left(f(l)),
            Either::Right(r) => Either::Right(g(r)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traversable::*;
    #[test]
    fn test_1() {
        let a = vec![vec![1, 2], vec![3, 4], vec![5, 6]].sequence();
        assert_eq!(
            a,
            vec![
                vec![1, 3, 5],
                vec![1, 3, 6],
                vec![1, 4, 5],
                vec![1, 4, 6],
                vec![2, 3, 5],
                vec![2, 3, 6],
                vec![2, 4, 5],
                vec![2, 4, 6]
            ] as Vec<Vec<i32>>
        )
    }
    #[test]
    fn test_2() {
        let v = vec![Either::Right(1), Either::Left(2)].sequence();
        assert_eq!(v, Either::Left(2))
    }
    #[test]
    fn test_3() {
        let v: Either<i32, Vec<i32>> =
            vec![Either::Right(1), Either::Right(2)].sequence();
        assert_eq!(v, Either::Right(vec![1, 2]))
    }
}
