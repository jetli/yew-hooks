use yew::prelude::*;
use yew_hooks::prelude::*;

/// `use_list` demo
#[function_component(UseList)]
pub fn list() -> Html {
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
        <div class="app">
            <header class="app-header">
                <div>
                    <button onclick={onset}>{ "Set to [6, 7, 8, 9, 10]" }</button>
                    <button onclick={oninsert}>{ "Insert 9 at position 0" }</button>
                    <button onclick={onupdate}>{ "Update to 4 at position 0" }</button>
                    <button onclick={onremove}>{ "Remove position 0" }</button>
                    <button onclick={onpush}>{ "Push 6" }</button>
                    <button onclick={onpop}>{ "Pop" }</button>
                    <button onclick={onretain}>{ "Retain even numbers" }</button>
                    <button onclick={onreverse}>{ "Reverse" }</button>
                    <button onclick={onappend}>{ "Append [11, 12, 13]" }</button>
                    <button onclick={onsort}>{ "Sort" }</button>
                    <button onclick={onsortdesc}>{ "Sort desc" }</button>
                    <button onclick={onswap}>{ "Swap position 0 and 1" }</button>
                    <button onclick={onclear}>{ "Clear all" }</button>
                    <p>
                        <b>{ "Current list: " }</b>
                    </p>
                    <p>
                        {
                            for list.current().iter().map(|element| {
                                html! {
                                    <b style="padding: 10px">{ element }</b>
                                }
                            })
                        }
                    </p>
                </div>
            </header>
        </div>
    }
}
