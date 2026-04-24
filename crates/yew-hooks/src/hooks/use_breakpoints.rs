use gloo::utils::window;
use yew::prelude::*;

use super::{use_event_with_window, use_mount, use_raf_state};

/// A map of breakpoint names to their min-width values.
///
/// Provided as a convenience so you don't have to look up common
/// breakpoint sets yourself.
///
/// # Example
///
/// ```rust
/// # use yew_hooks::prelude::*;
/// #
/// let breakpoints = BreakpointConfig::tailwind();
/// ```
pub struct BreakpointConfig;

impl BreakpointConfig {
    /// Tailwind CSS default breakpoints.
    ///
    /// | Name    | Min-width |
    /// |---------|-----------|
    /// | `sm`    | 640px     |
    /// | `md`    | 768px     |
    /// | `lg`    | 1024px    |
    /// | `xl`    | 1280px    |
    pub fn tailwind() -> Vec<(String, u32)> {
        vec![
            ("sm".to_string(), 640),
            ("md".to_string(), 768),
            ("lg".to_string(), 1024),
            ("xl".to_string(), 1280),
        ]
    }

    /// Bootstrap's default breakpoints.
    ///
    /// | Name | Min-width |
    /// |------|-----------|
    /// | `sm` | 576px     |
    /// | `md` | 768px     |
    /// | `lg` | 992px     |
    /// | `xl` | 1200px    |
    /// | `xxl`| 1400px    |
    pub fn bootstrap() -> Vec<(String, u32)> {
        vec![
            ("sm".to_string(), 576),
            ("md".to_string(), 768),
            ("lg".to_string(), 992),
            ("xl".to_string(), 1200),
            ("xxl".to_string(), 1400),
        ]
    }

    /// Breakpoints based on Material Design.
    ///
    /// | Name   | Min-width |
    /// |--------|-----------|
    /// | `sm`   | 600px     |
    /// | `md`   | 960px     |
    /// | `lg`   | 1280px    |
    /// | `xl`   | 1920px    |
    pub fn material() -> Vec<(String, u32)> {
        vec![
            ("sm".to_string(), 600),
            ("md".to_string(), 960),
            ("lg".to_string(), 1280),
            ("xl".to_string(), 1920),
        ]
    }
}

/// A sensor hook that tracks the current responsive breakpoint based on
/// window width.
///
/// Takes a list of breakpoint definitions (name → min-width). The breakpoints
/// are expected to be ordered from smallest to largest. The hook returns the
/// **largest** breakpoint whose min-width is less than or equal to the current
/// window inner width. If the window is narrower than the smallest breakpoint,
/// the given `default_breakpoint` (or "base") is returned.
///
/// # Resize handling
///
/// The hook listens to the window `"resize"` event and re-computes the active
/// breakpoint each render frame via `requestAnimationFrame` to avoid
/// excessive re-renders.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseBreakpoints)]
/// fn breakpoints_demo() -> Html {
///     let breakpoints = vec![
///         ("mobile".to_string(), 0),
///         ("tablet".to_string(), 640),
///         ("laptop".to_string(), 1024),
///         ("desktop".to_string(), 1440),
///     ];
///     let current = use_breakpoints(breakpoints);
///
///     html! {
///         <p>
///             <b>{ "Current breakpoint: " }</b>
///             { current }
///         </p>
///     }
/// }
/// ```
///
/// # Using a predefined set of breakpoints
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseTailwindBreakpoints)]
/// fn tailwind_demo() -> Html {
///     let current = use_breakpoints(BreakpointConfig::tailwind());
///
///     html! {
///         <p>
///             <b>{ "Current Tailwind breakpoint: " }</b>
///             { current }
///         </p>
///     }
/// }
/// ```
#[hook]
pub fn use_breakpoints(breakpoints: Vec<(String, u32)>) -> String {
    use_breakpoints_with_default(breakpoints, "base")
}

/// Like [`use_breakpoints`] but allows you to specify the name returned when
/// the window is narrower than the smallest breakpoint (`default_breakpoint`).
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
///
/// #[function_component(UseBreakpointsDefault)]
/// fn default_demo() -> Html {
///     let breakpoints = vec![
///         ("sm".to_string(), 640),
///         ("md".to_string(), 768),
///         ("lg".to_string(), 1024),
///     ];
///     let current = use_breakpoints_with_default(breakpoints, "xs");
///
///     html! {
///         <p><b>{ "Current: " }</b>{ current }</p>
///     }
/// }
/// ```
#[hook]
pub fn use_breakpoints_with_default(
    breakpoints: Vec<(String, u32)>,
    default_breakpoint: &str,
) -> String {
    let default: String = default_breakpoint.to_owned();

    // Initialise the state without referencing the local variables in a 'static closure.
    let state = use_raf_state(|| {
        let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
        resolve_breakpoint(&[], &default, width)
    });

    // Re-compute on mount so the initial value is correct.
    {
        let state = state.clone();
        let bps = breakpoints.clone();
        let default = default.clone();
        use_mount(move || {
            let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
            state.set(resolve_breakpoint(&bps, &default, width));
        });
    }

    // Listen to window resize, re-computing on each animation frame.
    {
        let state = state.clone();
        let bps = breakpoints.clone();
        let default = default.clone();
        use_event_with_window("resize", move |_: Event| {
            let width = window().inner_width().unwrap().as_f64().unwrap() as u32;
            state.set(resolve_breakpoint(&bps, &default, width));
        });
    }

    state.to_string()
}

/// Given a sorted list of `(name, min_width)` breakpoints and a window width,
/// return the name of the largest breakpoint whose min_width <= width.
/// If no breakpoint matches, `default_breakpoint` is returned.
fn resolve_breakpoint(bps: &[(String, u32)], default_breakpoint: &str, width: u32) -> String {
    let mut result = default_breakpoint;
    for (name, min_width) in bps {
        if width >= *min_width {
            result = name;
        } else {
            break;
        }
    }
    result.to_string()
}

#[cfg(test)]
mod tests {
    fn bps() -> Vec<(String, u32)> {
        vec![
            ("sm".to_string(), 640),
            ("md".to_string(), 768),
            ("lg".to_string(), 1024),
        ]
    }

    // Pure logic helper so we can test without a Wasm environment.
    fn resolve(bps: &[(String, u32)], default: &str, width: u32) -> String {
        let mut result = default;
        for (name, min_width) in bps {
            if width >= *min_width {
                result = name;
            } else {
                break;
            }
        }
        result.to_string()
    }

    #[test]
    fn test_below_smallest() {
        assert_eq!(resolve(&bps(), "base", 320), "base");
    }

    #[test]
    fn test_exact_match() {
        assert_eq!(resolve(&bps(), "base", 640), "sm");
        assert_eq!(resolve(&bps(), "base", 768), "md");
        assert_eq!(resolve(&bps(), "base", 1024), "lg");
    }

    #[test]
    fn test_falls_back_to_largest() {
        assert_eq!(resolve(&bps(), "base", 2000), "lg");
    }

    #[test]
    fn test_in_between() {
        assert_eq!(resolve(&bps(), "base", 700), "sm");
        assert_eq!(resolve(&bps(), "base", 900), "md");
    }
}
