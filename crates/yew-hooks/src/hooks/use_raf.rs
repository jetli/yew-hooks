use std::cmp::min_by;

use gloo::render::request_animation_frame;
use gloo::timers::callback::Timeout;

use yew::prelude::*;

/// An animation hook that forces component to re-render on each `requestAnimationFrame`,
/// returns percentage of time elapsed. `millis` - milliseconds for how long to keep re-rendering component.
/// `delay` — delay in milliseconds after which to start re-rendering component.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_raf;
///
/// #[function_component(UseRaf)]
/// fn raf() -> Html {
///     let elapsed = use_raf(5000, 1000);
///     
///     html! {
///         <>
///             { elapsed }
///         </>
///     }
/// }
/// ```
pub fn use_raf(millis: u32, delay: u32) -> f64 {
    let elapsed = use_state(|| 0f64);
    let start = use_mut_ref(|| 0f64);
    let raf = use_mut_ref(|| None);
    let on_frame = use_mut_ref(|| None);
    let on_frame_clone = on_frame.clone();
    let timer_stop = use_mut_ref(|| None);
    let timer_delay = use_mut_ref(|| None);

    {
        let elapsed = elapsed.clone();
        use_effect_with_deps(
            move |(millis, delay)| {
                let millis = *millis;
                let delay = *delay;
                *start.borrow_mut() = 0f64;

                {
                    let raf = raf.clone();
                    let elapsed = elapsed.clone();
                    *on_frame_clone.borrow_mut() = Some(Box::new(move |time: f64| {
                        let on_frame = on_frame.clone();
                        if *start.borrow() <= 0f64 {
                            *start.borrow_mut() = time;
                        }
                        let time =
                            min_by(1f64, (time - *start.borrow()) / (millis as f64), |x, y| {
                                x.partial_cmp(y).unwrap()
                            });
                        elapsed.set(time);

                        // Schedule ourself for another requestAnimationFrame callback.
                        // Ref: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/request-animation-frame/src/lib.rs
                        *raf.borrow_mut() = Some(request_animation_frame(move |time| {
                            let on_frame = on_frame.borrow();
                            #[allow(clippy::borrowed_box)]
                            let on_frame: &Box<dyn Fn(f64)> = on_frame.as_ref().unwrap();
                            on_frame(time);
                        }));
                    }) as Box<dyn Fn(f64)>);
                }

                {
                    let raf = raf.clone();
                    let timer_stop = timer_stop.clone();
                    *timer_delay.borrow_mut() = Some(Timeout::new(delay, move || {
                        {
                            let raf = raf.clone();
                            *timer_stop.borrow_mut() = Some(Timeout::new(millis, move || {
                                *raf.borrow_mut() = None;
                                elapsed.set(1f64);
                            }));
                        }

                        *raf.borrow_mut() = Some(request_animation_frame(move |time| {
                            let on_frame_clone = on_frame_clone.borrow();
                            #[allow(clippy::borrowed_box)]
                            let on_frame_clone: &Box<dyn Fn(f64)> =
                                on_frame_clone.as_ref().unwrap();
                            on_frame_clone(time);
                        }));
                    }));
                }

                move || {
                    *raf.borrow_mut() = None;
                    *timer_stop.borrow_mut() = None;
                    *timer_delay.borrow_mut() = None;
                }
            },
            (millis, delay),
        );
    }

    *elapsed
}
