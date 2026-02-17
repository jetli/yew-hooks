use gloo::events::EventListener;
use gloo::utils::window;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;
use yew::prelude::*;

/// Configuration options for the [`use_idle`] hook.
#[derive(Clone, Debug, PartialEq)]
pub struct UseIdleOptions {
    /// The time in milliseconds after which the user is considered idle.
    /// Default: 60_000 (1 minute)
    pub timeout: u32,
    /// Whether to listen for mouse events. Default: true
    pub listen_mouse: bool,
    /// Whether to listen for keyboard events. Default: true
    pub listen_keyboard: bool,
    /// Whether to listen for scroll events. Default: true
    pub listen_scroll: bool,
    /// Whether to listen for visibility change events. Default: true
    pub listen_visibility: bool,
    /// Whether to start in idle state. Default: false
    pub initial_idle: bool,
    /// Additional events to listen for.
    pub events: Vec<std::borrow::Cow<'static, str>>,
}

impl Default for UseIdleOptions {
    fn default() -> Self {
        Self {
            timeout: 60_000,
            listen_mouse: true,
            listen_keyboard: true,
            listen_scroll: true,
            listen_visibility: true,
            initial_idle: false,
            events: Vec::new(),
        }
    }
}

/// State handle for the [`use_idle`] hook.
#[derive(Clone, Debug, PartialEq)]
pub struct UseIdleHandle {
    idle: UseStateHandle<bool>,
    last_active: UseStateHandle<Option<f64>>,
    reset: Callback<()>,
}

impl UseIdleHandle {
    /// Returns whether the user is currently idle.
    pub fn is_idle(&self) -> bool {
        *self.idle
    }

    /// Manually reset the idle timer, marking the user as active.
    pub fn reset_idle(&self) {
        self.reset.emit(());
    }

    /// Get the timestamp of the last user activity, if available.
    pub fn last_active(&self) -> Option<f64> {
        *self.last_active
    }
}

impl Deref for UseIdleHandle {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.idle
    }
}

/// This hook tracks whether the user is idle (not interacting with the page).
/// It listens for various user events (mouse, keyboard, scroll, etc.) and resets
/// an internal timer. When the timer expires, the user is considered idle.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
/// use std::time::Duration;
///
/// #[function_component(UseIdle)]
/// fn idle() -> Html {
///     let idle = use_idle(Duration::from_secs(30));
///
///     html! {
///         <div>
///             <p>
///                 <b>{ "User is idle: " }</b>
///                 { *idle }
///             </p>
///             <p>
///                 { "Move your mouse or press a key to reset the idle timer." }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_idle(timeout: Duration) -> UseIdleHandle {
    let options = UseIdleOptions {
        timeout: timeout.as_millis() as u32,
        ..Default::default()
    };
    use_idle_with_options(options)
}

/// This hook tracks whether the user is idle with custom configuration options.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::prelude::*;
/// use std::time::Duration;
///
/// #[function_component(UseIdleWithOptions)]
/// fn idle_with_options() -> Html {
///     let idle = use_idle_with_options(UseIdleOptions {
///         timeout: Duration::from_secs(10).as_millis() as u32,
///         listen_scroll: false,
///         events: vec!["touchstart".into(), "touchend".into()],
///         ..Default::default()
///     });
///
///     html! {
///         <div>
///             <p>
///                 <b>{ "User is idle: " }</b>
///                 { *idle }
///             </p>
///             <p>
///                 { "Touch the screen or move your mouse to reset the idle timer." }
///             </p>
///         </div>
///     }
/// }
/// ```
#[hook]
pub fn use_idle_with_options(options: UseIdleOptions) -> UseIdleHandle {
    let idle = use_state_eq(|| options.initial_idle);
    let last_active = use_state_eq(|| None::<f64>);

    // Store timeout in a ref so we can cancel it
    let timeout_ref = use_mut_ref(|| None::<gloo::timers::callback::Timeout>);

    // Function to restart the idle timeout
    let restart_idle_timeout = {
        let idle = idle.clone();
        let timeout_ref = timeout_ref.clone();
        let timeout = options.timeout;
        Rc::new(move || {
            // Cancel any existing timeout
            *timeout_ref.borrow_mut() = None;

            // Create new timeout
            if timeout > 0 {
                let idle_clone = idle.clone();
                *timeout_ref.borrow_mut() =
                    Some(gloo::timers::callback::Timeout::new(timeout, move || {
                        idle_clone.set(true);
                    }));
            }
        })
    };

    // Function to handle user activity
    let handle_activity = {
        let idle = idle.clone();
        let last_active = last_active.clone();
        let restart_idle_timeout = restart_idle_timeout.clone();
        Rc::new(move || {
            idle.set(false);
            last_active.set(Some(js_sys::Date::now()));
            restart_idle_timeout();
        })
    };

    // Create a callback to reset the idle state
    let reset = {
        let handle_activity = handle_activity.clone();
        Callback::from(move |()| {
            handle_activity();
        })
    };

    // Set up event listeners and timeout management
    {
        let handle_activity = handle_activity.clone();
        let _restart_idle_timeout = restart_idle_timeout.clone();
        let last_active = last_active.clone();
        let options_clone = options.clone();

        // Store event listeners in Rc<RefCell> so they can be kept alive
        let listeners = Rc::new(RefCell::new(Vec::<EventListener>::new()));

        use_effect_with((), {
            let listeners = listeners.clone();
            let timeout_ref = timeout_ref.clone();
            move |_| {
                let window = window();

                // Clear any existing listeners
                listeners.borrow_mut().clear();

                // Mouse events
                if options_clone.listen_mouse {
                    let ha1 = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        "mousemove",
                        move |_| {
                            ha1();
                        },
                    ));

                    let ha2 = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        "mousedown",
                        move |_| {
                            ha2();
                        },
                    ));

                    let ha3 = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        "mouseup",
                        move |_| {
                            ha3();
                        },
                    ));
                }

                // Keyboard events
                if options_clone.listen_keyboard {
                    let ha4 = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        "keydown",
                        move |_| {
                            ha4();
                        },
                    ));

                    let ha5 = handle_activity.clone();
                    listeners
                        .borrow_mut()
                        .push(EventListener::new(&window, "keyup", move |_| {
                            ha5();
                        }));

                    let ha6 = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        "keypress",
                        move |_| {
                            ha6();
                        },
                    ));
                }

                // Scroll events
                if options_clone.listen_scroll {
                    let ha7 = handle_activity.clone();
                    listeners
                        .borrow_mut()
                        .push(EventListener::new(&window, "scroll", move |_| {
                            ha7();
                        }));
                }

                // Visibility change events
                if options_clone.listen_visibility {
                    let ha8 = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        "visibilitychange",
                        move |_| {
                            ha8();
                        },
                    ));
                }

                // Additional custom events
                for event_name in options_clone.events.clone() {
                    let ha_custom = handle_activity.clone();
                    listeners.borrow_mut().push(EventListener::new(
                        &window,
                        event_name.clone(),
                        move |_| {
                            ha_custom();
                        },
                    ));
                }

                // Initial setup - mark as active and start timeout
                if !options_clone.initial_idle {
                    handle_activity();
                } else {
                    // If starting idle, just set the timestamp
                    last_active.set(Some(js_sys::Date::now()));
                }

                // Cleanup function
                move || {
                    // Cancel any pending timeout
                    *timeout_ref.borrow_mut() = None;
                    // Clear listeners - they will be dropped automatically
                    listeners.borrow_mut().clear();
                }
            }
        });
    }

    UseIdleHandle {
        idle,
        last_active,
        reset,
    }
}
