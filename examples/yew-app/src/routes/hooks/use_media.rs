use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_media` demo
#[function_component(UseMedia)]
pub fn media() -> Html {
    let node_video = use_node_ref();
    let node_audio = use_node_ref();

    let video = use_media(
        node_video.clone(),
        "http://clips.vorwaerts-gmbh.de/big_buck_bunny.mp4".to_string(),
    );

    let audio = use_media_with_options(
        node_audio.clone(),
        "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-2.mp3".to_string(),
        UseMediaOptions::enable_auto_play(),
    );

    let onplay = {
        let video = video.clone();
        Callback::from(move |_| video.play())
    };

    let onpause = {
        let video = video.clone();
        Callback::from(move |_| video.pause())
    };

    let onmute = {
        let video = video.clone();
        Callback::from(move |_| video.mute())
    };

    let onunmute = {
        let video = video.clone();
        Callback::from(move |_| video.unmute())
    };

    let onvol1 = {
        let video = video.clone();
        Callback::from(move |_| video.set_volume(0.1))
    };

    let onvol2 = {
        let video = video.clone();
        Callback::from(move |_| video.set_volume(0.5))
    };

    let onvol3 = {
        let video = video.clone();
        Callback::from(move |_| video.set_volume(1.0))
    };

    let onseek1 = {
        let video = video.clone();
        Callback::from(move |_| video.seek(*video.time - 5.0))
    };

    let onseek2 = {
        let video = video.clone();
        Callback::from(move |_| video.seek(*video.time + 5.0))
    };

    let onplay_audio = {
        let audio = audio.clone();
        Callback::from(move |_| audio.play())
    };

    let onpause_audio = {
        let audio = audio.clone();
        Callback::from(move |_| audio.pause())
    };

    html! {
        <div class="app">
            <header class="app-header">
                <div>
                    <p>
                        <b>{ " Video " }</b>
                        { " Duration: " } { *video.duration }
                        { " Time: " } { *video.time }
                        { " Volume: " } { *video.volume }
                    </p>
                    <p><video ref={node_video} width="640" height="360" /></p>
                    <p>
                        <button onclick={onplay} disabled={*video.playing}>{ "Play" }</button>
                        <button onclick={onpause} disabled={*video.paused}>{ "Pause" }</button>
                        <button onclick={onmute} disabled={*video.muted}>{ "Mute" }</button>
                        <button onclick={onunmute} disabled={!*video.muted}>{ "Unmute" }</button>
                        <button onclick={onvol1}>{ "Volume: 10%" }</button>
                        <button onclick={onvol2}>{ "Volume: 50%" }</button>
                        <button onclick={onvol3}>{ "Volume: 100%" }</button>
                        <button onclick={onseek1}>{ "Seek: -5 secs" }</button>
                        <button onclick={onseek2}>{ "Seek: +5 secs" }</button>
                    </p>
                    <p><b>{ " Audio " }</b></p>
                    <p><audio ref={node_audio} /></p>
                    <p>
                        <button onclick={onplay_audio} disabled={*audio.playing}>{ "Play" }</button>
                        <button onclick={onpause_audio} disabled={*audio.paused}>{ "Pause" }</button>
                    </p>
                </div>
            </header>
        </div>
    }
}
