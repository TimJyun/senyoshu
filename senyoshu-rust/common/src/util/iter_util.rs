use std::mem::swap;

pub struct WithNextMutMap<I, F, O>
    where
        I: Iterator,
        F: FnMut(I::Item, Option<&mut I::Item>) -> O,
{
    iter: I,
    f: F,
    next: Option<I::Item>,
}

pub trait WithNextMutMapItertool: Iterator {
    fn with_next_mut_map<F, O>(self, f: F) -> WithNextMutMap<Self, F, O>
        where
            Self: Sized,
            F: FnMut(Self::Item, Option<&mut Self::Item>) -> O;
}

impl<I: Iterator> WithNextMutMapItertool for I {
    fn with_next_mut_map<F, O>(mut self, f: F) -> WithNextMutMap<Self, F, O>
        where
            F: FnMut(Self::Item, Option<&mut Self::Item>) -> O,
    {
        let next = self.next();
        WithNextMutMap::<Self, F, O> {
            iter: self,
            f,
            next,
        }
    }
}

impl<I, F, O> Iterator for WithNextMutMap<I, F, O>
    where
        I: Iterator,
        F: FnMut(I::Item, Option<&mut I::Item>) -> O,
{
    type Item = O;

    fn next(&mut self) -> Option<O> {
        if self.next.is_none() {
            return None;
        }
        let mut value = None;
        swap(&mut value, &mut self.next);
        self.next = self.iter.next();

        Some(if let Some(next) = &mut self.next {
            (self.f)(value.unwrap(), Some(next))
        } else {
            (self.f)(value.unwrap(), None)
        })
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::storage::iter_util::WithNextMutMapItertool;
//
//     #[test]
//     fn it_works() {
//         let array = [1, 2, 3, 4];
//
//         let mut a = array.into_iter().with_next_mut_map(|a, b| (a, b.map(|b| *b)));
//
//         assert_eq!((1, Some(2)), a.next().unwrap());
//         assert_eq!((2, Some(3)), a.next().unwrap());
//         assert_eq!((3, Some(4)), a.next().unwrap());
//         assert_eq!((4, None), a.next().unwrap());
//         assert_eq!(None, a.next());
//     }
// }
