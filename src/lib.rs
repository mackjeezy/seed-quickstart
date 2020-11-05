#![allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    //initialize the model
    let model = Model::default();

    //make sure posts are loaded on init
    orders.perform_cmd(async {
        let response = fetch("https://jsonplaceholder.typicode.com/posts")
            .await
            .expect("HTTP request failed");

        let posts: Vec<Post> = response
            .check_status() // ensure we've got 2xx status
            .expect("status check failed")
            .json::<Vec<Post>>()
            .await
            .expect("deserialization failed");

        Msg::PostsReceived(posts)
    });
    model
}

// `Model` describes our app state.
#[derive(Default)]
struct Model {
    posts: Vec<Post>,
}
#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Post {
    id: i32,
    userId: i32,
    title: String,
    body: String,
}

// `Msg` describes the different events you can modify state with.
enum Msg {
    PostsReceived(Vec<Post>),
    ViewPost(i32),
    DeletePost(i32),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    log!("update !!");
    match msg {
        Msg::ViewPost(_id) => {}
        Msg::DeletePost(_id) => {}
        Msg::PostsReceived(posts) => {
            model.posts = posts;
        }
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    log!("view!!");
    div![
        C!["App"],
        div![
            C!["container", "home"],
            h4![C!["center"], "Home",],
            view_nav_bar(),
            IF!(not(model.posts.is_empty()) => vec![
                view_main(model),
            ]),
            IF!(model.posts.is_empty() => vec![
                div![C!["center"],"No posts to show"]
            ])
        ]
    ]
}

fn view_main(model: &Model) -> Node<Msg> {
    log!("view_main");
    let posts = model.posts.iter();
    div![posts.map(|post| {
        div![
            C!["post", "card"],
            img![attrs![
                At::Src => "assets/pokeball.png",
                At::Alt => "A Pokeball"
            ]],
            div![
                C!["card-content"],
                a![
                    attrs![
                        At::Href => format!("/posts/{}",&post.id),
                    ],
                    span![C!["card-title", "red-text"], &post.title],
                ],
                p![&post.body]
            ],
        ]
    }),]
}

fn view_nav_bar() -> Node<Msg> {
    log!("view_nav_bar");
    nav![
        C!["nav-wrapper", "red", "darken-3"],
        div![
            C!["container"],
            a![
                C!["brand-logo"],
                attrs![
                    At::Href => "/",
                ],
                "Poke' Times"
            ],
            ul![
                C!["right"],
                li![a![
                    attrs![
                        At::Href => "/",
                    ],
                    "Home"
                ],],
                li![a![
                    attrs![
                        At::Href => "/about",
                    ],
                    "About"
                ],],
                li![a![
                    attrs![
                        At::Href => "/contact",
                    ],
                    "Contact"
                ],]
            ]
        ]
    ]
}

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    log!("start ");
    // Mount the `app` to the element with the `id` "app".
    App::start("root", init, update, view);
}
