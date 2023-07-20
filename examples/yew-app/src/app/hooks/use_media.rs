use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_media` demo
#[function_component]
pub fn UseMedia() -> Html {
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
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <p>
                        <b>{ " Video " }</b>
                        { " Duration: " } { *video.duration }
                        { " Time: " } { *video.time }
                        { " Volume: " } { *video.volume }
                    </p>
                    <p><video class="mx-auto" ref={node_video} width="640" height="360" /></p>
                    <p class="space-x-2">
                        <Button onclick={onplay} disabled={*video.playing}>{ "Play" }</Button>
                        <Button onclick={onpause} disabled={*video.paused}>{ "Pause" }</Button>
                        <Button onclick={onmute} disabled={*video.muted}>{ "Mute" }</Button>
                        <Button onclick={onunmute} disabled={!*video.muted}>{ "Unmute" }</Button>
                        <Button onclick={onvol1}>{ "Volume: 10%" }</Button>
                        <Button onclick={onvol2}>{ "Volume: 50%" }</Button>
                        <Button onclick={onvol3}>{ "Volume: 100%" }</Button>
                        <Button onclick={onseek1}>{ "Seek: -5 secs" }</Button>
                        <Button onclick={onseek2}>{ "Seek: +5 secs" }</Button>
                    </p>
                    <p><b>{ " Audio " }</b></p>
                    <p><audio ref={node_audio} /></p>
                    <p class="space-x-2">
                        <Button onclick={onplay_audio} disabled={*audio.playing}>{ "Play" }</Button>
                        <Button onclick={onpause_audio} disabled={*audio.paused}>{ "Pause" }</Button>
                    </p>
                </div>
            </header>
        </div>
    }
}
