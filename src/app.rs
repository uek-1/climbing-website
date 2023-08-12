use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
    let route_data = vec![RouteData::default(); 5];
    let set_data = SetData::new(route_data, date);

    view! { cx,
        <main class="container">
            <h1 style="text-align:center">"Climbing Website"</h1>
            <Set set_data={set_data}/>
        </main>
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

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Date {
    day: u8,
    month: u8,
    year: u32,
}

impl Date {
    fn new(day: u8, month: u8, year: u32) -> Self {
        Date { day, month, year }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct RouteData {
    image: bool,
    grade: u8,
    setter: String,
    likes: u32,
}

impl Default for RouteData {
    fn default() -> Self {
        RouteData {
            image: false,
            grade: 0,
            setter: String::from("Unknown"),
            likes: 0,
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct SetData {
    routes: Vec<RouteData>,
    date: Date,
}

impl SetData {
    fn new(routes: Vec<RouteData>, date: Date) -> Self {
        SetData { routes, date }
    }
}

#[component]
fn Set(cx: Scope, set_data: SetData) -> impl IntoView {
    view! { cx,
        <article>
            <header>
                <h2>"Set of "{set_data.date.day}"/" {set_data.date.month} "/" {set_data.date.year} </h2>
            </header>

            <For
            each=move || set_data.clone().routes
            key= move |x| x.clone()
            view=move|cx, data: RouteData| view!{cx,
                <ClimbingRouteItem route_data = data />
            }/>
        </article>
    }
}

#[component]
fn ClimbingRouteItem(cx: Scope, route_data: RouteData) -> impl IntoView {
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
            <img src=route_data.image class="center" style="width:100px; height:100px"/>
            <div class="grid">
                <p>"Grade: " {route_data.grade}</p>
                <p>"Setter: " {route_data.setter}</p>
                <p>"Likes: " {route_data.likes}</p>
            </div>
        </article>
    }
}
