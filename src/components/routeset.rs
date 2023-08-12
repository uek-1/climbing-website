use super::*;
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProblemData {
    image: String,
    grade: u8,
    setter: String,
    likes: u32,
}

impl Default for ProblemData {
    fn default() -> Self {
        ProblemData {
            image: String::from("false"),
            grade: 0,
            setter: String::from("Unknown"),
            likes: 0,
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SetData {
    problems: Vec<ProblemData>,
    date: Date,
}

impl SetData {
    pub fn new(problems: Vec<ProblemData>, date: Date) -> Self {
        SetData { problems, date }
    }
}

#[component]
pub fn Set(cx: Scope, set_data: SetData) -> impl IntoView {
    view! { cx,
        <article>
            <header>
                <h2>"Set of "{set_data.date.day}"/" {set_data.date.month} "/" {set_data.date.year} </h2>
            </header>

            <For
            each=move || set_data.clone().problems
            key= move |x| x.clone()
            view=move|cx, data: ProblemData| view!{cx,
                <ProblemItem problem_data=data />
            }/>
        </article>
    }
}

#[component]
pub fn ProblemItem(cx: Scope, problem_data: ProblemData) -> impl IntoView {
    view! {cx,

        <style>
        "
        .center {
          display: block;
          margin-left: auto;
          margin-right: auto;
          width: 50%;
        "
        </style>

        <article>
            <img src=problem_data.image class="center" style="width:100px; height:100px"/>
            <div class="grid">
                <p>"Grade: " {problem_data.grade}</p>
                <p>"Setter: " {problem_data.setter}</p>
                <p>"Likes: " {problem_data.likes}</p>
            </div>
        </article>
    }
}
