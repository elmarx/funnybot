//! Poor Mans Mock Helper
//!
//! When the [existing mock crates](https://asomers.github.io/mock_shootout/) do not work, Funnybot
//! could help out.
//!
//! It helps with manual mocking (of traits). Mocks can call into Funnybot and thus both return
//! pre-programmed values and record arguments.
//!
//! Since pointing to the current value to return, and recording arguments requires mutability,
//! Funnybot encapsulates the mutability into [`RwLock`](std::sync::RwLock).  

use std::iter::repeat_with;
use std::sync::{RwLock, RwLockReadGuard};

pub struct FunnyBot<'a, A, R>
where
    A: Send + Sync + Clone,
    R: Send + Sync + 'a,
{
    arguments: RwLock<Vec<A>>,
    returns: RwLock<Box<dyn Iterator<Item = R> + Send + Sync + 'a>>,
}

impl<'a, A, R> FunnyBot<'a, A, R>
where
    A: Send + Sync + Clone,
    R: Send + Sync + 'a + Clone,
{
    /// initialize Funnybot with a fixed value that will be repeated for each call
    pub fn repeat(elt: R) -> Self {
        FunnyBot::new(Box::new(std::iter::repeat(elt)))
    }
}

impl<'a, A, R> FunnyBot<'a, A, R>
where
    A: Send + Sync + Clone,
    R: Send + Sync + 'a,
{
    #[must_use]
    pub fn new(returns: Box<dyn Iterator<Item = R> + Send + Sync + 'a>) -> Self {
        FunnyBot {
            arguments: RwLock::new(Vec::new()),
            returns: RwLock::new(returns),
        }
    }

    /// initialize Funnybot with a single return value.
    /// consequently calling more than once will panic
    #[must_use]
    pub fn from_single(returns: R) -> Self {
        Self::from_list(vec![returns])
    }

    /// initialize Funnybot with a fixed list of return values.
    /// once the all values have returned, the function will panic
    #[must_use]
    pub fn from_list(returns: Vec<R>) -> Self {
        Self::new(Box::new(returns.into_iter()))
    }

    /// initialize Funnybot with a repeater-function that generates values for each call
    pub fn repeat_with<F: 'a + Sync + Send + FnMut() -> R>(repeater: F) -> Self {
        Self::new(Box::new(repeat_with(repeater)))
    }

    /// simulate a call
    ///
    /// pass the arguments to record as `a`
    ///
    /// returns the next value from the iterator as pre-programmed
    pub fn call(&self, a: A) -> R {
        self.arguments.write().unwrap().push(a);

        self.returns
            .write()
            .unwrap()
            .next()
            .expect("missing pre-programmed value for call")
    }

    /// return the recorded arguments. This consumes the Funnybot
    pub fn into_args(self) -> Vec<A> {
        self.arguments.into_inner().unwrap()
    }

    /// return a current snapshot of the recorded arguments
    pub fn args(&self) -> RwLockReadGuard<Vec<A>> {
        self.arguments.read().unwrap()
    }
}
