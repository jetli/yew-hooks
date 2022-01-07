# Yew Hooks

Hooks for [Yew](https://github.com/yewstack/yew), inspired by [streamich/react-use](https://github.com/streamich/react-use) and [alibaba/hooks](https://github.com/alibaba/hooks).

## Hooks

### State

- `use_toggle` - tracks state of counterparts.
- `use_bool_toggle` - tracks state of a boolean.
- `use_counter` -  tracks state of a number.

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

## Demo

- [Check out a live demo](https://jetli.github.io/yew-hooks/)
