//! TODO
//!
//! - If we're using the other curve, then this won't be an instantiation of RedDSA
//! - let's use "Verification" and "Signing" key terminology

use std::marker::PhantomData;

pub trait SigType: private::Sealed {}

pub enum SpendAuth {}
impl SigType for SpendAuth {}

pub enum Binding {}
impl SigType for Binding {}

pub struct SigningKey<T: SigType> {
    _t: PhantomData<T>,
}

pub struct VerificationKey<T: SigType> {
    _t: PhantomData<T>,
}

pub struct Signature<T: SigType> {
    _t: PhantomData<T>,
}

pub(crate) mod private {
    use super::{Binding, SpendAuth};

    pub trait Sealed {}

    impl Sealed for SpendAuth {}

    impl Sealed for Binding {}
}
