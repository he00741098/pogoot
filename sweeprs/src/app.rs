use icondata as i;
use leptos::{logging::log, prelude::*};
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes, A},
    path, StaticSegment,
};
use reactive_stores::Store;
use serde::{Deserialize, Serialize};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // let global_state_store = GlobalStateStore { test_counter: 0 };
    // provide_context(Store::new(global_state_store));
    view! {
        <Stylesheet id="leptos" href="/pkg/sweeprs.css" />
        <Title text="Sweep-rs" />
        <Nav />
        <Router>
            <Main>
                <main>
                    <Routes fallback=NothingHere>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=path!("dashboard") view=Dashboard />
                        <Route path=path!("lib") view=NothingHere />
                        <Route path=path!("create") view=NothingHere />
                    </Routes>
                </main>
            </Main>
            <End />
        </Router>
    }
}

#[island]
fn Nav() -> impl IntoView {
    let show = RwSignal::new(0);
    provide_context(show);
    view! {
        <NavBar />
        <Slider />
    }
}

#[component]
fn End() -> impl IntoView {
    view! {
        <div class="endSection">
            <div class="legal">
                <Link href="/tos".into() text="Terms Of Service".into() />
                <Link href="/privacy".into() text="Privacy Policy".into() />
                <Link href="/delete".into() text="Delete Account and Related Data".into() />
            </div>
            <div class="contact">
                <Link href="/github".into() text="Github".into() />
            </div>
            <div class="other">
                <Link href="/about".into() text="About".into() />
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // let state = expect_context::<Store<GlobalStateStore>>();
    // log!("{}", state.test_counter().get());
    // Creates a reactive value to update the button
    view! { <Title text="Home | Sweep-rs" /> }
}

fn get_card_recommendations(userId: String, quantity: usize) -> Vec<Card> {
    std::iter::repeat(Card {
        title: "Title".into(),
        description: "Description".into(),
        creator: "Creator".into(),
        url: "url".into(),
    })
    .take(quantity)
    .collect::<Vec<Card>>()
}
fn get_recent_cards(userId: String, quantity: usize) -> Vec<Card> {
    std::iter::repeat(Card {
        title: "Title".into(),
        description: "Description".into(),
        creator: "Creator".into(),
        url: "url".into(),
    })
    .take(quantity)
    .collect::<Vec<Card>>()
}

#[component]
fn Dashboard() -> impl IntoView {
    let cards = get_card_recommendations("userId".into(), 20);
    let recent_cards = get_recent_cards("userId".into(), 20);
    view! {
        <Title text="Dashboard | Sweep-rs" />
        <SetRow cards=cards text="Recommended For You".into() />
        <SetRow cards=recent_cards text="Recently Accessed".into() />
    }
}

#[component]
fn NothingHere() -> impl IntoView {
    view! {
        <Title text="404 Not Found | Sweep-rs" />
        <h1>Not Found</h1>
    }
}
#[component]
fn NavBar() -> impl IntoView {
    view! {
        <div id="navbar">
            <Pancake />
            <Link href="/dashboard".into() text="Home".into() />
            <Link href="/lib".into() text="Library".into() />
            <Link href="/create".into() text="Create".into() />
        </div>
    }
}

#[component]
fn Link(href: String, text: String) -> impl IntoView {
    view! {
        <a href=href>
            <div class="link_box">{text}</div>
        </a>
    }
}
#[island]
fn Slider() -> impl IntoView {
    let show = use_context::<RwSignal<i32>>().expect("to have context");
    view! {
        <div
            class:slider=move || show.get() % 2 == 0
            class:hide=move || show.get() % 2 != 0
            class:sliderAnimation=move || { show.get() % 2 == 0 && show.get() > 0 }
        >
            <div class="link_box">Hi</div>
            <div class="link_box">Whats up</div>
            <div class="link_box">Yo</div>
            <div class="link_box">Ex</div>
        </div>
    }
}
#[island]
fn Pancake() -> impl IntoView {
    let show = use_context::<RwSignal<i32>>().expect("to have context");
    let on_click = move |_| {
        show.update(|show| *show += 1);
    };

    view! {
        <div class="link_box pointer" on:click=on_click>
            <Icon icon=i::CgMenu height="1.3em" />
        </div>
    }
}

#[island]
fn Main(children: Children) -> impl IntoView {
    let show = use_context::<RwSignal<i32>>().expect("to have context");
    view! {
        <div
            class:primaryOpen=move || { show.get() % 2 == 0 }
            class="primary"
            class:primaryOpenAnimation=move || { show.get() % 2 != 0 && show.get() > 0 }
            class:primaryCloseAnimation=move || { show.get() % 2 == 0 && show.get() > 0 }
        >
            {children()}
        </div>
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Card {
    title: String,
    description: String,
    creator: String,
    url: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default, Store)]
struct GlobalStateStore {
    test_counter: usize,
}

///Initially planned to be a row, but instead it will be simply a block of cards.
#[component]
fn SetRow(cards: Vec<Card>, text: String) -> impl IntoView {
    let children = cards
        .into_iter()
        .map(|child| {
            view! {
                <div class="card">
                    <span class="cardTitle">{child.title}</span>
                    <span class="cardDescription">{child.description}</span>
                    <span class="cardCreator">{child.creator}</span>
                </div>
            }
        })
        .collect::<Vec<_>>();
    view! {
        <h3 class="recommend">{text}</h3>
        <div class="setrow">{children}</div>
    }
}
