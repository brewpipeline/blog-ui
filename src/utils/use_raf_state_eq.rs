use std::ops::Deref;
use std::{cell::RefCell, rc::Rc};

use gloo::render::{request_animation_frame, AnimationFrame};
use yew::prelude::*;

use yew_hooks::use_unmount;

pub struct UseRafStateEqHandle<T> {
    inner: UseStateHandle<T>,
    raf: Rc<RefCell<Option<AnimationFrame>>>,
}

impl<T> UseRafStateEqHandle<T>
where
    T: 'static,
{
    pub fn set(&self, value: T) {
        let inner = self.inner.clone();
        *self.raf.borrow_mut() = Some(request_animation_frame(move |_| {
            inner.set(value);
        }));
    }
}

impl<T> Deref for UseRafStateEqHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Clone for UseRafStateEqHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            raf: self.raf.clone(),
        }
    }
}

impl<T> PartialEq for UseRafStateEqHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

#[hook]
pub fn use_raf_state_eq<T, F>(init_fn: F) -> UseRafStateEqHandle<T>
where
    T: PartialEq + 'static,
    F: FnOnce() -> T,
{
    let inner = use_state_eq(init_fn);
    let raf = use_mut_ref(|| None);

    {
        let raf = raf.clone();
        use_unmount(move || {
            *raf.borrow_mut() = None;
        });
    }

    UseRafStateEqHandle { inner, raf }
}
