use super::*;
use crate::cfg_if;
use serde::{Deserialize, Serialize};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection, sqlite::SqliteConnectOptions};

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            let options = SqliteConnectOptions::new()
                .filename("problems.db")
                .create_if_missing(true);

            Ok(SqliteConnection::connect_with(&options).await?)
        }
    }
}

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct ProblemData {
    pub image: String,
    pub grade: u8,
    pub setter: String,
    pub likes: u32,
    pub date: String,
}

impl Default for ProblemData {
    fn default() -> Self {
        ProblemData {
            image: String::from("false"),
            grade: 0,
            setter: String::from("Unknown"),
            likes: 0,
            date: String::from(""),
        }
    }
}

impl ProblemData {
    pub fn get_date(&self) -> String {
        self.date.clone()
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Debug)]
pub struct SetData {
    problems: Vec<ProblemData>,
    pub date: Date,
}

impl SetData {
    pub fn new(problems: Vec<ProblemData>, date: Date) -> Self {
        SetData { problems, date }
    }
}

#[component]
pub fn SetItem(cx: Scope, set_data: SetData) -> impl IntoView {
    view! { cx,
        <header>
            <h2>"Set of "{set_data.date.day}"/" {set_data.date.month} "/" {set_data.date.year} </h2>
        </header>

        <For
        each=move || set_data.clone().problems
        key= move |x| x.clone()
        view=move|cx, data: ProblemData| view!{cx,
            <ProblemItem problem_data=data />
        }/>
    }
}

#[component]
pub fn Sets(cx: Scope, data: Vec<SetData>) -> impl IntoView {
    view! {cx,
        <For each=move || data.clone()
         key = |x| x.clone()
         view = move |cx, set_data: SetData| {
            view!{
                cx,
                <SetItem set_data=set_data/>
            }
        }/>

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

#[server(AddProblem, "/api")]
pub async fn add_problem(problem: ProblemData) -> Result<(), ServerFnError> {
    let problem = ProblemData::default();
    let mut conn = db().await?;
    let query_string = format!(
        "INSERT INTO problems VALUES {} {} {} {} {}",
        problem.image, problem.grade, problem.setter, problem.likes, problem.date
    );
    let query = sqlx::query(&query_string).execute(&mut conn).await?;

    Ok(())
}

pub async fn get_sets() -> Vec<SetData> {
    let problems = get_problems().await;
    println!("{problems:?}");
    let problems = problems.unwrap_or(vec![]);
    let mut set_map = std::collections::HashMap::new();
    for problem in problems {
        set_map
            .entry(problem.date.clone())
            .or_insert(vec![])
            .push(problem);
    }

    set_map
        .into_iter()
        .map(|(k, v)| SetData::new(v, Date::from(k)))
        .inspect(|x| println!("{:?}", x))
        .collect()
}

#[server(GetProblems, "/api")]
pub async fn get_problems() -> Result<Vec<ProblemData>, ServerFnError> {
    // Get the database connection
    let mut conn = db().await?;
    let mut problems = vec![];
    // Select * from Problems
    use futures::TryStreamExt;

    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS problems (
        image bool,
        grade int,
        setter text,
        likes int,
        date text
        );"#,
    )
    .execute(&mut conn)
    .await?;

    let mut rows = sqlx::query_as::<_, ProblemData>("SELECT * FROM problems").fetch(&mut conn);

    while let Some(row) = rows.try_next().await? {
        println!("{row:?}");
        problems.push(row);
    }

    Ok(problems)
}
