use yew::{function_component, html, Html, Properties};

#[derive(Clone, PartialEq)]
pub struct Actor {
    pub id: String,
    pub name: String
}

#[derive(Properties, PartialEq)]
pub struct ActorsProps {
    pub actors: Option<Vec<Actor>>
}

#[function_component(Actors)]
pub fn actors(props: &ActorsProps) -> Html {
    let table = match &props.actors {
        Some(actors) => html! {
            <table>
                <tr>
                    <th>{"ID"}</th>
                    <th>{"名前"}</th>
                </tr>
                { actors.iter().map(construct_table_row).collect::<Html>() }
            </table>
        },
        None => html! {
            <p>{"登録情報がありません"}</p>
        }
    };
    html! {
            <div>
                <h2>{"Actor一覧"}</h2>
                { table }
            </div>
    }
}

fn construct_table_row(actor: &Actor) -> Html {
    html! {
        <tr>
            <td>{ &actor.id }</td>
            <td>{ &actor.name }</td>
        </tr>
    }
}
