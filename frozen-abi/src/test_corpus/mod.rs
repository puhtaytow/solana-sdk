mod impls;

pub trait TestCorpus<Ctx = ()>: Sized {
    type Corpus: IntoIterator<Item = Self>;

    fn corpus_with_context(ctx: Ctx) -> Self::Corpus;

    fn corpus() -> Self::Corpus
    where
        Ctx: Default,
    {
        Self::corpus_with_context(Ctx::default())
    }
}
