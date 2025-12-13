use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::ui::button::Button;

/// `use_list` demo
#[function_component]
pub fn UseList() -> Html {
    let list = use_list(vec![1, 2, 3, 4, 5]);

    let onset = {
        let list = list.clone();
        Callback::from(move |_| list.set(vec![6, 7, 8, 9, 10]))
    };
    let oninsert = {
        let list = list.clone();
        Callback::from(move |_| list.insert(0, 9))
    };
    let onupdate = {
        let list = list.clone();
        Callback::from(move |_| list.update(0, 4))
    };
    let onremove = {
        let list = list.clone();
        Callback::from(move |_| {
            let _ = list.remove(0);
        })
    };
    let onpush = {
        let list = list.clone();
        Callback::from(move |_| list.push(6))
    };
    let onpop = {
        let list = list.clone();
        Callback::from(move |_| {
            let _ = list.pop();
        })
    };
    let onretain = {
        let list = list.clone();
        Callback::from(move |_| list.retain(|x| x % 2 == 0))
    };
    let onreverse = {
        let list = list.clone();
        Callback::from(move |_| list.reverse())
    };
    let onappend = {
        let list = list.clone();
        Callback::from(move |_| list.append(&mut vec![11, 12, 13]))
    };
    let onsort = {
        let list = list.clone();
        Callback::from(move |_| list.sort())
    };
    let onsortdesc = {
        let list = list.clone();
        Callback::from(move |_| list.sort_by(|a, b| b.cmp(a)))
    };
    let onswap = {
        let list = list.clone();
        Callback::from(move |_| list.swap(0, 1))
    };
    let onclear = {
        let list = list.clone();
        Callback::from(move |_| list.clear())
    };

    html! {
        <div class="container">
            <header class="mt-24 text-xl text-center">
                <div class="space-x-4 space-y-4">
                    <Button onclick={onset}>{ "Set to [6, 7, 8, 9, 10]" }</Button>
                    <Button onclick={oninsert}>{ "Insert 9 at position 0" }</Button>
                    <Button onclick={onupdate}>{ "Update to 4 at position 0" }</Button>
                    <Button onclick={onremove}>{ "Remove position 0" }</Button>
                    <Button onclick={onpush}>{ "Push 6" }</Button>
                    <Button onclick={onpop}>{ "Pop" }</Button>
                    <Button onclick={onretain}>{ "Retain even numbers" }</Button>
                    <Button onclick={onreverse}>{ "Reverse" }</Button>
                    <Button onclick={onappend}>{ "Append [11, 12, 13]" }</Button>
                    <Button onclick={onsort}>{ "Sort" }</Button>
                    <Button onclick={onsortdesc}>{ "Sort desc" }</Button>
                    <Button onclick={onswap}>{ "Swap position 0 and 1" }</Button>
                    <Button onclick={onclear}>{ "Clear all" }</Button>
                    <p>
                        <b>{ "Current list: " }</b>
                    </p>
                    <p>
                        {
                            for list.current().iter().map(|element| {
                                html! {
                                    <b class="p-8">{ *element }</b>
                                }
                            })
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}
