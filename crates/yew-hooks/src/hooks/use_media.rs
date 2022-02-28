use std::cmp::{max_by, min_by};
use std::rc::Rc;

use wasm_bindgen::{prelude::*, JsValue};
use web_sys::{HtmlMediaElement, TimeRanges};

use yew::{prelude::*, TargetCast};

use super::{use_event, use_mut_latest};

/// Options for media
#[derive(Default)]
pub struct UseMediaOptions {
    /// Auto play the media.
    pub auto_play: bool,
    pub onplay: Option<Box<dyn FnMut(Event)>>,
    pub onplaying: Option<Box<dyn FnMut(Event)>>,
    pub onwaiting: Option<Box<dyn FnMut(Event)>>,
    pub onpause: Option<Box<dyn FnMut(Event)>>,
    pub onvolumechange: Option<Box<dyn FnMut(Event)>>,
    pub ondurationchange: Option<Box<dyn FnMut(Event)>>,
    pub ontimeupdate: Option<Box<dyn FnMut(Event)>>,
    pub onprogress: Option<Box<dyn FnMut(Event)>>,
}

impl UseMediaOptions {
    /// Set `auto_play` to true
    pub fn enable_auto_play() -> Self {
        Self {
            auto_play: true,
            ..Self::default()
        }
    }
}

/// State handle for the [`use_media`] hook.
pub struct UseMediaHandle {
    pub buffered: UseStateHandle<Vec<(f64, f64)>>,
    pub duration: UseStateHandle<f64>,
    pub paused: UseStateHandle<bool>,
    pub muted: UseStateHandle<bool>,
    pub time: UseStateHandle<f64>,
    pub volume: UseStateHandle<f64>,
    pub playing: UseStateHandle<bool>,

    play: Rc<dyn Fn()>,
    pause: Rc<dyn Fn()>,
    seek: Rc<dyn Fn(f64)>,
    set_volume: Rc<dyn Fn(f64)>,
    mute: Rc<dyn Fn()>,
    unmute: Rc<dyn Fn()>,
}

impl UseMediaHandle {
    /// Play the media.
    pub fn play(&self) {
        (&self.play)()
    }

    /// Pause the media.
    pub fn pause(&self) {
        (&self.pause)()
    }

    /// Mute the media.
    pub fn mute(&self) {
        (&self.mute)()
    }

    /// Unmute the media.
    pub fn unmute(&self) {
        (&self.unmute)()
    }

    /// Set volume of the media.
    pub fn set_volume(&self, value: f64) {
        (&self.set_volume)(value)
    }

    /// Seek the media.
    pub fn seek(&self, value: f64) {
        (&self.seek)(value)
    }
}

impl Clone for UseMediaHandle {
    fn clone(&self) -> Self {
        Self {
            buffered: self.buffered.clone(),
            duration: self.duration.clone(),
            paused: self.paused.clone(),
            muted: self.muted.clone(),
            time: self.time.clone(),
            volume: self.volume.clone(),
            playing: self.playing.clone(),

            play: self.play.clone(),
            pause: self.pause.clone(),
            seek: self.seek.clone(),
            set_volume: self.set_volume.clone(),
            mute: self.mute.clone(),
            unmute: self.unmute.clone(),
        }
    }
}

/// This hook plays video or audio and exposes its controls.
///
/// # Example
///
/// ```rust
/// # use yew::prelude::*;
/// #
/// use yew_hooks::use_media;
///
/// #[function_component(UseMedia)]
/// fn media() -> Html {
///     let node_video = use_node_ref();
///
///     let video = use_media(
///         node_video.clone(),
///         "http://clips.vorwaerts-gmbh.de/big_buck_bunny.mp4".to_string(),
///     );
///
///     let onplay = {
///         let video = video.clone();
///         Callback::from(move |_| video.play())
///     };
///
///     let onpause = {
///         let video = video.clone();
///         Callback::from(move |_| video.pause())
///     };
///
///     let onmute = {
///         let video = video.clone();
///         Callback::from(move |_| video.mute())
///     };
///
///     let onunmute = {
///         let video = video.clone();
///         Callback::from(move |_| video.unmute())
///     };
///
///     let onvol = {
///         let video = video.clone();
///         Callback::from(move |_| video.set_volume(0.5))
///     };
///     let onseek = {
///         let video = video.clone();
///         Callback::from(move |_| video.seek(*video.time + 5.0))
///     };
///
///     html! {
///         <>
///             <p><video ref={node_video} width="640" height="360" /></p>
///             <p>
///                 { " Duration: " } { *video.duration }
///                 { " Time: " } { *video.time }
///                 { " Volume: " } { *video.volume }
///             </p>
///             <p>
///                 <button onclick={onplay} disabled={*video.playing}>{ "Play" }</button>
///                 <button onclick={onpause} disabled={*video.paused}>{ "Pause" }</button>
///                 <button onclick={onmute} disabled={*video.muted}>{ "Mute" }</button>
///                 <button onclick={onunmute} disabled={!*video.muted}>{ "Unmute" }</button>
///                 <button onclick={onvol}>{ "Volume: 50%" }</button>
///                 <button onclick={onseek1}>{ "Seek: +5 secs" }</button>
///             </p>
///         </>
///     }
/// }
/// ```
pub fn use_media(node: NodeRef, src: String) -> UseMediaHandle {
    use_media_with_options(node, src, UseMediaOptions::default())
}

/// This hook plays video or audio and exposes its controls with options.
/// see [`use_media`]
pub fn use_media_with_options(
    node: NodeRef,
    src: String,
    options: UseMediaOptions,
) -> UseMediaHandle {
    let buffered = use_state(Vec::new);
    let duration = use_state(|| 0.0);
    let paused = use_state(|| true);
    let muted = use_state(|| false);
    let time = use_state(|| 0.0);
    let volume = use_state(|| 1.0);
    let playing = use_state(|| false);

    let onplay_ref = use_mut_latest(options.onplay);
    let onplaying_ref = use_mut_latest(options.onplaying);
    let onwaiting_ref = use_mut_latest(options.onwaiting);
    let onpause_ref = use_mut_latest(options.onpause);
    let onvolumechange_ref = use_mut_latest(options.onvolumechange);
    let ondurationchange_ref = use_mut_latest(options.ondurationchange);
    let ontimeupdate_ref = use_mut_latest(options.ontimeupdate);
    let onprogress_ref = use_mut_latest(options.onprogress);

    let play_lock = use_mut_ref(|| false);

    {
        let node = node.clone();
        let paused = paused.clone();
        use_event(node, "play", move |e: Event| {
            paused.set(false);

            let onplay_ref = onplay_ref.current();
            let onplay = &mut *onplay_ref.borrow_mut();
            if let Some(onplay) = onplay {
                onplay(e);
            }
        });
    }

    {
        let node = node.clone();
        let playing = playing.clone();
        use_event(node, "playing", move |e: Event| {
            playing.set(true);

            let onplaying_ref = onplaying_ref.current();
            let onplaying = &mut *onplaying_ref.borrow_mut();
            if let Some(onplaying) = onplaying {
                onplaying(e);
            }
        });
    }

    {
        let node = node.clone();
        let playing = playing.clone();
        use_event(node, "waiting", move |e: Event| {
            playing.set(false);

            let onwaiting_ref = onwaiting_ref.current();
            let onwaiting = &mut *onwaiting_ref.borrow_mut();
            if let Some(onwaiting) = onwaiting {
                onwaiting(e);
            }
        });
    }

    {
        let node = node.clone();
        let paused = paused.clone();
        let playing = playing.clone();
        use_event(node, "pause", move |e: Event| {
            paused.set(true);
            playing.set(false);

            let onpause_ref = onpause_ref.current();
            let onpause = &mut *onpause_ref.borrow_mut();
            if let Some(onpause) = onpause {
                onpause(e);
            }
        });
    }

    {
        let node = node.clone();
        let muted = muted.clone();
        let volume = volume.clone();
        use_event(node, "volumechange", move |e: Event| {
            if let Some(media) = e.target_dyn_into::<HtmlMediaElement>() {
                muted.set(media.muted());
                volume.set(media.volume());
            }

            let onvolumechange_ref = onvolumechange_ref.current();
            let onvolumechange = &mut *onvolumechange_ref.borrow_mut();
            if let Some(onvolumechange) = onvolumechange {
                onvolumechange(e);
            }
        });
    }

    {
        let node = node.clone();
        let duration = duration.clone();
        let buffered = buffered.clone();
        use_event(node, "durationchange", move |e: Event| {
            if let Some(media) = e.target_dyn_into::<HtmlMediaElement>() {
                duration.set(media.duration());
                buffered.set(parse_time_ranges(media.buffered()));
            }

            let ondurationchange_ref = ondurationchange_ref.current();
            let ondurationchange = &mut *ondurationchange_ref.borrow_mut();
            if let Some(ondurationchange) = ondurationchange {
                ondurationchange(e);
            }
        });
    }

    {
        let node = node.clone();
        let time = time.clone();
        use_event(node, "timeupdate", move |e: Event| {
            if let Some(media) = e.target_dyn_into::<HtmlMediaElement>() {
                time.set(media.current_time());
            }

            let ontimeupdate_ref = ontimeupdate_ref.current();
            let ontimeupdate = &mut *ontimeupdate_ref.borrow_mut();
            if let Some(ontimeupdate) = ontimeupdate {
                ontimeupdate(e);
            }
        });
    }

    {
        let node = node.clone();
        let buffered = buffered.clone();
        use_event(node, "progress", move |e: Event| {
            if let Some(media) = e.target_dyn_into::<HtmlMediaElement>() {
                buffered.set(parse_time_ranges(media.buffered()));
            }

            let onprogress_ref = onprogress_ref.current();
            let onprogress = &mut *onprogress_ref.borrow_mut();
            if let Some(onprogress) = onprogress {
                onprogress(e);
            }
        });
    }

    let play = {
        let play_lock = play_lock.clone();
        let node = node.clone();
        Rc::new(move || {
            if !*play_lock.borrow() {
                if let Some(media) = node.cast::<HtmlMediaElement>() {
                    if let Ok(promise) = media.play() {
                        *play_lock.borrow_mut() = true;
                        let play_lock = play_lock.clone();
                        let closure = Closure::wrap(Box::new(move |_| {
                            *play_lock.borrow_mut() = false;
                        })
                            as Box<dyn FnMut(JsValue)>);
                        let _ = promise.then2(&closure, &closure);
                        closure.forget();
                    }
                }
            }
        })
    };

    let pause = {
        let node = node.clone();
        Rc::new(move || {
            if !*play_lock.borrow() {
                if let Some(media) = node.cast::<HtmlMediaElement>() {
                    let _ = media.pause();
                }
            }
        })
    };

    let seek = {
        let duration = duration.clone();
        let node = node.clone();
        Rc::new(move |time: f64| {
            if let Some(media) = node.cast::<HtmlMediaElement>() {
                let time = max_by(0.0, time, |x, y| x.partial_cmp(y).unwrap());
                let time = min_by(*duration, time, |x, y| x.partial_cmp(y).unwrap());
                media.set_current_time(time);
            }
        })
    };

    let set_volume = {
        let volume = volume.clone();
        let node = node.clone();
        Rc::new(move |vol: f64| {
            if let Some(media) = node.cast::<HtmlMediaElement>() {
                let vol = max_by(0.0, vol, |x, y| x.partial_cmp(y).unwrap());
                let vol = min_by(1.0, vol, |x, y| x.partial_cmp(y).unwrap());
                media.set_volume(vol);
                volume.set(vol);
            }
        })
    };

    let mute = {
        let node = node.clone();
        let muted = muted.clone();
        Rc::new(move || {
            if let Some(media) = node.cast::<HtmlMediaElement>() {
                media.set_muted(true);
                muted.set(true);
            }
        })
    };

    let unmute = {
        let node = node.clone();
        let muted = muted.clone();
        Rc::new(move || {
            if let Some(media) = node.cast::<HtmlMediaElement>() {
                media.set_muted(false);
                muted.set(false);
            }
        })
    };

    {
        let volume = volume.clone();
        let muted = muted.clone();
        let paused = paused.clone();
        let play = play.clone();
        let auto_play = options.auto_play;
        use_effect_with_deps(
            move |(node, src)| {
                if let Some(media) = node.cast::<HtmlMediaElement>() {
                    media.set_controls(false);
                    media.set_src(src);

                    volume.set(media.volume());
                    muted.set(media.muted());
                    paused.set(media.paused());

                    if auto_play && media.paused() {
                        play();
                    }
                }

                || ()
            },
            (node, src),
        );
    }

    UseMediaHandle {
        buffered,
        duration,
        paused,
        muted,
        time,
        volume,
        playing,

        play,
        pause,
        seek,
        set_volume,
        mute,
        unmute,
    }
}

fn parse_time_ranges(ranges: TimeRanges) -> Vec<(f64, f64)> {
    let mut result = vec![];
    if ranges.length() > 0 {
        for index in 0..ranges.length() {
            result.push((
                ranges.start(index).unwrap_throw(),
                ranges.end(index).unwrap_throw(),
            ))
        }
    }
    result
}
