use crate::test_corpus::TestCorpus;

impl<T, I> TestCorpus<I> for T
where
    I: IntoIterator<Item = T>,
{
    type Corpus = I;

    fn corpus_with_context(ctx: I) -> Self::Corpus {
        ctx
    }
}

#[cfg(test)]
mod tests {
    fn test_corpus_vec() -> [TestCorpusVecContext; 2] {
        [
            TestCorpusVecContext {
                a: 0,
                b: false,
                c: None,
            },
            TestCorpusVecContext {
                a: u8::MAX,
                b: true,
                c: Some(vec![0, u16::MAX]),
            },
        ]
    }

    #[derive(wincode::SchemaRead, wincode::SchemaWrite)]
    #[cfg_attr(
        feature = "frozen-abi",
        derive(
            solana_frozen_abi_macro::StableAbi,
            solana_frozen_abi_macro::StableAbiSample
        ),
        solana_frozen_abi_macro::frozen_abi(
            abi_digest = "ErVp1LhW4wAyXr8KudiFF9DpT3ZuGx8mWBeT5mXnCn8m",
            abi_serializer = "wincode",
            test_roundtrip = "wire_only",
            test_corpus = test_corpus_vec(),
        )
    )]
    struct TestCorpusVecContext {
        a: u8,
        b: bool,
        c: Option<Vec<u16>>,
    }
}
