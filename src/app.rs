use crate::components::*;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
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

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet href="https://cdn.jsdelivr.net/npm/@picocss/pico@1/css/pico.min.css"/>

        // sets the document title
        <Title text="Climbing Website"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/archive" view=NotFound/>
                    <Route path="/hof" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Await
            future = |cx| get_sets()
            bind:sets
        >
        <main class="container">
            <h1 style="text-align:center">"Climbing Website"</h1>
            <Sets data=sets.clone()/>
        </main>
        </Await>
    }
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
    let mut conn = match db().await {
        Ok(x) => {
            println!("Successfully connected to the databse!");
            x
        }
        Err(e) => {
            println!("Unsuccessful connection : {e:?}");
            return Err(e);
        }
    };

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

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Debug)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u32,
}

impl Date {
    pub fn new(day: u8, month: u8, year: u32) -> Self {
        Date { day, month, year }
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        let mut value = value.split("/");
        let mut day = value.nth(0);
        let mut month = value.nth(1);
        let mut year = value.nth(2);

        Date {
            day: day.map(|x| x.parse().unwrap_or(0)).unwrap_or(0),
            month: month.map(|x| x.parse().unwrap_or(0)).unwrap_or(0),
            year: year.map(|x| x.parse().unwrap_or(0)).unwrap_or(0),
        }
    }
}
