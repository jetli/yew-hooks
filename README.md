# Yew Hooks

Hooks for [Yew](https://github.com/yewstack/yew), inspired by [streamich/react-use](https://github.com/streamich/react-use) and [alibaba/hooks](https://github.com/alibaba/hooks).

## Hooks

### State

- `use_toggle` - tracks state of counterparts.
- `use_bool_toggle` - tracks state of a boolean.
- `use_counter` -  tracks state of a number.

### Side-effects

- `use_async` - resolves an async future.
- `use_title` - sets title of the page.

### Lifecycles

- `use_effect_once` - a modified use_effect hook that only runs once.
- `use_mount` - calls mount callbacks.
- `use_unmount` - calls unmount callbacks.
- `use_is_first_mount` - checks if current render is first.
- `use_is_mounted` - tracks if component is mounted.

### Animations

- `use_timeout` - schedules a timeout to invoke callback.
- `use_interval` - schedules an interval to invoke callback.
- `use_update` - returns a callback, which re-renders component when called.

## Examples

```rust
use yew::prelude::*;

use yew_hooks::use_counter;

#[function_component(Counter)]
fn counter() -> Html {
    let counter = use_counter(0);

    let onincrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase())
    };
    let ondecrease = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease())
    };
    let onincreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.increase_by(10))
    };
    let ondecreaseby = {
        let counter = counter.clone();
        Callback::from(move |_| counter.decrease_by(10))
    };
    let onset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(100))
    };
    let onreset = {
        let counter = counter.clone();
        Callback::from(move |_| counter.reset())
    };
    
    html! {
        <div>
            <button onclick={onincrease}>{ "Increase" }</button>
            <button onclick={ondecrease}>{ "Decrease" }</button>
            <button onclick={onincreaseby}>{ "Increase by 10" }</button>
            <button onclick={ondecreaseby}>{ "Decrease by 10" }</button>
            <button onclick={onset}>{ "Set to 100" }</button>
            <button onclick={onreset}>{ "Reset" }</button>
            <p>
                <b>{ "Current value: " }</b>
                { *counter }
            </p>
        </div>
    }
}
```

## Demo

[Check out a live demo](https://jetli.github.io/yew-hooks/)
