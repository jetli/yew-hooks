use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use yew::functional::{use_reducer, Reducible, UseReducerHandle};

enum ToggleAction<T> {
    Toggle,
    Reset,
    Set(T),
    SetLeft,
    SetRight,
}

struct UseToggleReducer<T>
where
    T: PartialEq,
{
    value: Rc<T>,
    left: Rc<T>,
    right: Rc<T>,
}

impl<T> Reducible for UseToggleReducer<T>
where
    T: PartialEq,
{
    type Action = ToggleAction<T>;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_value = match action {
            ToggleAction::Toggle => {
                if self.value == self.left {
                    self.right.clone()
                } else {
                    self.left.clone()
                }
            }
            ToggleAction::Reset => self.left.clone(),
            ToggleAction::Set(value) => Rc::new(value),
            ToggleAction::SetLeft => self.left.clone(),
            ToggleAction::SetRight => self.right.clone(),
        };

        Self {
            value: next_value,
            left: self.left.clone(),
            right: self.right.clone(),
        }
        .into()
    }
}

impl<T> PartialEq for UseToggleReducer<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// State handle for the [`use_toggle`] hook.
pub struct UseToggleHandle<T>
where
    T: PartialEq,
{
    inner: UseReducerHandle<UseToggleReducer<T>>,
}

impl<T> UseToggleHandle<T>
where
    T: PartialEq,
{
    pub fn toggle(&self) {
        self.inner.dispatch(ToggleAction::Toggle)
    }

    pub fn set(&self, value: T) {
        self.inner.dispatch(ToggleAction::Set(value))
    }

    pub fn set_left(&self) {
        self.inner.dispatch(ToggleAction::SetLeft)
    }

    pub fn set_right(&self) {
        self.inner.dispatch(ToggleAction::SetRight)
    }

    pub fn reset(&self) {
        self.inner.dispatch(ToggleAction::Reset)
    }
}

impl<T> Deref for UseToggleHandle<T>
where
    T: PartialEq,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}

impl<T> Clone for UseToggleHandle<T>
where
    T: PartialEq,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> PartialEq for UseToggleHandle<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl<T: fmt::Debug> fmt::Debug for UseToggleHandle<T>
where
    T: PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseToggleHandle")
            .field("value", &format!("{:?}", self.inner.value))
            .field("left", &format!("{:?}", self.inner.left))
            .field("right", &format!("{:?}", self.inner.right))
            .finish()
    }
}

/// This hook is a simplified [`use_toggle`] to manage boolean toggle state in a function component.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// # use yew_hooks::use_bool_toggle;
/// #
/// #[function_component(UseToggle)]
/// fn toggle() -> Html {
///     let toggle = use_bool_toggle(true);
///     let onclick = {
///         let toggle = toggle.clone();
///         Callback::from(move |_| toggle.toggle())
///     };
///     
///     html! {
///         <div>
///             <button {onclick}>{ "Toggle" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *toggle }
///             </p>
///         </div>
///     }
///
/// }
/// ```
pub fn use_bool_toggle(default: bool) -> UseToggleHandle<bool> {
    use_toggle(default, !default)
}

/// This hook is used to manage toggle state in a function component.
///
/// # Example
/// ```rust
/// # use yew::prelude::*;
/// # use std::rc::Rc;
/// #
/// # use yew_hooks::use_toggle;
/// #
/// #[function_component(UseToggle)]
/// fn toggle() -> Html {
///     let toggle = use_toggle("Hello", "World");
///     let onclick = {
///         let toggle = toggle.clone();
///         Callback::from(move |_| toggle.toggle())
///     };
///     
///     html! {
///         <div>
///             <button {onclick}>{ "Toggle" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *toggle }
///             </p>
///         </div>
///     }
///
/// }
/// ```
pub fn use_toggle<T>(default: T, other: T) -> UseToggleHandle<T>
where
    T: 'static + PartialEq,
{
    let value = Rc::new(default);
    let left = value.clone();
    let right = Rc::new(other);

    let handle = use_reducer(move || UseToggleReducer { value, left, right });

    UseToggleHandle { inner: handle }
}
