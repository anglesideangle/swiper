use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

#[derive(Clone, Copy)]
pub struct Translation<Unit, const Dim: usize, Src, Dst> {
    pub vector: [Unit; Dim],
    _src: PhantomData<Src>,
    _dst: PhantomData<Dst>,
}

impl<Unit, const Dim: usize, Src, Mid, Dst> Add<Translation<Unit, Dim, Mid, Dst>>
    for Translation<Unit, Dim, Src, Mid>
{
    type Output = Translation<Unit, Dim, Src, Dst>;

    fn add(self, rhs: Translation<Unit, Dim, Mid, Dst>) -> Self::Output {
        todo!()
    }
}
