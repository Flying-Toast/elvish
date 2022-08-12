pub use web_sys as web;
pub use wasm_bindgen;

use core::ops::{Deref, DerefMut};

pub struct Reactive<T> {
    inner: T,
    dirty: bool,
}

impl<T> Reactive<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            dirty: false,
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

impl<T> Deref for Reactive<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Reactive<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;

        &mut self.inner
    }
}

pub trait Component {
    fn create() -> Self;
    fn make_fragment(&mut self) -> web::DocumentFragment;
    fn update(&mut self);
}

#[macro_export]
macro_rules! ctx {
    ($($propname:ident: $proptype:ty = $startval:expr,)*$(,)?) => {
        struct Ctx {
            $( $propname: Reactive<$proptype>, )*
        }

        impl Ctx {
            fn new() -> Self {
                Self {
                    $( $propname: Reactive::new($startval), )*
                }
            }
        }
    }
}
