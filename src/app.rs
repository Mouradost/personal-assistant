use leptos::*;
use leptos_router::*;
use leptos_meta::*;
use personal_assistant_ui::{pages::{Conversation, Setting}, components::NavBar, setup};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    setup(cx);
    let routes = vec![
        ("/".to_owned(), "Setting".to_owned()),
        ("/conversation".to_owned(), "Conversation".to_owned()),
    ];
    let (is_model_connected, _) = use_context::<(ReadSignal<bool>, WriteSignal<bool>)>(cx)
            .expect("to have found the getter provided for model status");
    view! { cx,
        <Body class="flex flex-col w-screen h-screen"/>
        <Router>
            <header class="flex-0">
                <NavBar routes=routes/>
            </header> //h-14
            <main class="flex-1 flex flex-col overflow-scroll">
                <Routes>
                    <Route path="/" view=|cx| view! { cx, <Setting/> }/>
                    <Route path="/conversation" view=|cx| view! { cx, <Conversation/> }/>
                </Routes>
            </main>
            <footer class="flex-0 flex flex-col">
            
              <Show
                when=is_model_connected
                fallback=|cx| view! { cx, <p class="bg-red-500 text-black text-center w-full p-1">"Model disconnected"</p>}
              >
                <p class="bg-green-500 text-black text-center w-full p-1">"Model connected"</p>
              </Show>
                
            </footer> //h-14
        </Router>
    }
}
