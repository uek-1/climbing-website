use crate::components::*;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
    let date = Date::new(1, 1, 2003);

    view! { cx,
        <Await
            future = |cx| get_problems()
            bind: problem_data
        >
        <main class="container">
            <h1 style="text-align:center">"Climbing Website"</h1>
            <Set set_data={
                SetData::new(problem_data.clone().unwrap_or(vec![]), date.clone())
        }/>
        </main>
        </Await>
    }
}

fn handle_problems_resource(
    cx: Scope,
    problem_data: Resource<(), Result<Vec<ProblemData>, ServerFnError>>,
    date: Date,
) -> routeset::SetData {
    match problem_data.read(cx) {
        None => {
            println!("Loading");
            routeset::SetData::new(vec![], date)
        }
        Some(x) => {
            let res = match x {
                Ok(y) => {
                    println!("Read {y:?} from database");
                    y
                }
                Err(e) => {
                    println!("Error reading from databse");
                    vec![]
                }
            };
            routeset::SetData::new(res, date)
        }
    }
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
        likes int
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

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
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
