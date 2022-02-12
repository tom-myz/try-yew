mod actors;

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    let data = create_dummy_data();

    html! {
            <div>
                <button onclick={onclick}>{ "+1" }</button>
                <p>{ *counter }</p>
                <actors::Actors actors={ data }/>
            </div>
    }
}

fn create_dummy_data() -> Vec<actors::Actor> {
    let mut data: Vec<actors::Actor> = Vec::with_capacity(10);
    data.push(create_dummy_actor("your id", "your name"));
    data.push(create_dummy_actor("his id", "his name"));
    return data
}

fn create_dummy_actor(prim_id: &str, prim_name: &str) -> actors::Actor {
    let id = prim_id.to_string();
    let name = prim_name.to_string();
    actors::Actor{id, name}
}
