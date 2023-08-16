use crate::components::*;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

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
            <TopNavBar />
            <main class="container">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/archive" view=NotFound/>
                    <Route path="/admin" view=AdminPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn TopNavBar(cx: Scope) -> impl IntoView {
    view! { cx,
        <header class="container-fluid">
            <nav>
                <ul>
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/admin">"Admin"</a></li>
                    <li><a href="/archive">"Archive"</a></li>
                </ul>
            </nav>
        </header>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <Await
            future = |cx| routeset::get_sets()
            bind:sets
        >
            <article id="sets">
                <header style="text-align:center"><h1>"Climbing Website"</h1></header>
                <Sets data=sets.clone()/>
            </article>
        </Await>
    }
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
