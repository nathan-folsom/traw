pub trait Mutate {
    type Mutation;
    fn mutate(&mut self, mx: Self::Mutation) -> Self::Mutation;
}
