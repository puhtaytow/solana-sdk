use {
    arbitrary::{Arbitrary, Unstructured},
    rand::RngCore,
};

pub trait StableAbi: Sized {
    fn random(rng: &mut impl RngCore) -> Self
    where
        Self: for<'a> Arbitrary<'a>,
    {
        let mut buffer = vec![0u8; 1024];
        rng.fill_bytes(&mut buffer);

        let mut unstructured = Unstructured::new(&buffer);
        Self::arbitrary(&mut unstructured).expect("failed to fill")
    }
}
