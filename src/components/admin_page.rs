use super::SetData;
use crate::app::TopNavBar;
use crate::components::routeset::{add_problem, get_problems, get_sets};
use crate::components::ProblemData;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[server(HelloWorld, "/api")]
async fn hello_world(input: String) -> Result<(), ServerFnError> {
    println!("hello_world");
    Ok(())
}

#[component]
pub fn AdminPage(cx: Scope) -> impl IntoView {
    let (is_submit, set_is_submit) = create_signal(cx, false);
    let action = create_server_action::<HelloWorld>(cx);
    view! {
    cx,
    <button on:click= move |_| {
        set_is_submit(true)
    } >"Add Item"</button>

    <Await future=|cx| get_problems() bind:data>

      <Show
         when = move || is_submit.get()
        fallback= |_| ()
    >
        <AddItemModal write_flag = set_is_submit/>
      </Show>

      <ProblemTable data=data.to_owned().unwrap_or(vec![]) />


    </Await>
    }
}

#[component]
pub fn ProblemTable(cx: Scope, data: Vec<ProblemData>) -> impl IntoView {
    let (id, set_id) = create_signal(cx, 0);

    view! {cx,
    <table>
      <thead>
        <tr>
          <th scope="col">"#"</th>
          <th scope="col">"image"</th>
          <th scope="col">"grade"</th>
          <th scope="col">"setter"</th>
          <th scope="col">"likes"</th>
          <th scope="col">"date"</th>
        </tr>
      </thead>
      <tbody>
        <For each=move || data.clone() key=|item| item.get_date() view=move |cx, problem_data: ProblemData| { view!{ cx,
          {set_id(id.get() + 1)} <tr>
          <th scope="row">{id.get()}</th>
          <td> {problem_data.image} </td>
          <td> {problem_data.grade} </td>
          <td> {problem_data.setter} </td>
          <td> {problem_data.likes} </td>
          <td> {problem_data.date} </td>
          </tr>
          }
          }
          />
      </tbody>
    </table>
    }
}

#[component]
pub fn AddItemModal(cx: Scope, write_flag: WriteSignal<bool>) -> impl IntoView {
    use super::routeset::AddProblem;
    let add_problem = create_server_action::<AddProblem>(cx);

    view! {cx,
    <dialog open>
      <article>
        <ActionForm action=add_problem>
          <label for="image">
            Photo
            <input type="text" id="image" name="image" placeholder="Image" required/>
          </label>
          <label for="grade">
            Grade
            <input type="text" id="grade" name="grade" placeholder="grade" required/>
          </label>
          <label for="setter">
            Setter
            <input type="text" id="setter" name="setter" placeholder="setter" required/>
          </label>
          <label for="date">
            Date
            <input type="text" id="date" name="date" placeholder="date" required/>
          </label>

          <button type="submit">Submit</button>
        </ActionForm>
      </article>
    </dialog>
    }
}
