use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use yew::functional::{use_reducer, Reducible, UseReducerHandle};

enum CounterAction {
    Increase,
    IncreaseBy(i32),
    Decrease,
    DecreaseBy(i32),
    Set(i32),
    Reset,
}

struct UseCounterReducer {
    value: i32,
    default: i32,
}

impl Reducible for UseCounterReducer {
    type Action = CounterAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_value = match action {
            CounterAction::Increase => self.value + 1,
            CounterAction::IncreaseBy(delta) => self.value + delta,
            CounterAction::Decrease => self.value - 1,
            CounterAction::DecreaseBy(delta) => self.value - delta,
            CounterAction::Set(value) => value,
            CounterAction::Reset => self.default,
        };

        Self {
            value: next_value,
            default: self.default,
        }
        .into()
    }
}

impl PartialEq for UseCounterReducer {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// State handle for the [`use_counter`] hook.
pub struct UseCounterHandle {
    inner: UseReducerHandle<UseCounterReducer>,
}

impl UseCounterHandle {
    pub fn increase(&self) {
        self.inner.dispatch(CounterAction::Increase)
    }

    pub fn increase_by(&self, delta: i32) {
        self.inner.dispatch(CounterAction::IncreaseBy(delta))
    }

    pub fn decrease(&self) {
        self.inner.dispatch(CounterAction::Decrease)
    }

    pub fn decrease_by(&self, delta: i32) {
        self.inner.dispatch(CounterAction::DecreaseBy(delta))
    }

    pub fn set(&self, value: i32) {
        self.inner.dispatch(CounterAction::Set(value))
    }

    pub fn reset(&self) {
        self.inner.dispatch(CounterAction::Reset)
    }
}

impl fmt::Debug for UseCounterHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UseCounterHandle")
            .field("value", &format!("{:?}", self.inner.value))
            .field("default", &format!("{:?}", self.inner.default))
            .finish()
    }
}

impl Deref for UseCounterHandle {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &(*self.inner).value
    }
}

impl Clone for UseCounterHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl PartialEq for UseCounterHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

/// This hook is used to manage counter state in a function component.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// # use yew_hooks::use_counter;
/// #
/// #[function_component(Counter)]
/// fn counter() -> Html {
///     let counter = use_counter(0);
///
///     let onincrease = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.increase())
///     };
///     let ondecrease = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.decrease())
///     };
///     let onincreaseby = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.increase_by(10))
///     };
///     let ondecreaseby = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.decrease_by(10))
///     };
///     let onset = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.set(100))
///     };
///     let onreset = {
///         let counter = counter.clone();
///         Callback::from(move |_| counter.reset())
///     };
///     
///     html! {
///         <div>
///             <button onclick={onincrease}>{ "Increase" }</button>
///             <button onclick={ondecrease}>{ "Decrease" }</button>
///             <button onclick={onincreaseby}>{ "Increase by 10" }</button>
///             <button onclick={ondecreaseby}>{ "Decrease by 10" }</button>
///             <button onclick={onset}>{ "Set to 100" }</button>
///             <button onclick={onreset}>{ "Reset" }</button>
///             <p>
///                 <b>{ "Current value: " }</b>
///                 { *counter }
///             </p>
///         </div>
///     }
/// }
/// ```
pub fn use_counter(default: i32) -> UseCounterHandle {
    let handle = use_reducer(move || UseCounterReducer {
        value: default,
        default,
    });

    UseCounterHandle { inner: handle }
}
