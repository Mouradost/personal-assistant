use leptos::*;
use leptos_icons::*;
use leptos_meta::*;

#[component]
pub fn NavBar(cx: Scope, routes: Vec<(String, String)>) -> impl IntoView {
    let (menu_icon, set_menu_icon) = create_signal(cx, String::from("menu-outline"));
    let menu_clicked = move |_| {
        set_menu_icon.update(|current_icon| {
            *current_icon = if current_icon.contains("menu") {
                "close-outline".to_string()
            } else {
                "menu-outline".to_string()
            };
        })
    };

    let (color_scheme, set_color_scheme) = create_signal(cx, String::from("dark"));
    let dark_mode_toggle = move |_| {
        set_color_scheme.update(|current_color_scheme| {
            *current_color_scheme = if current_color_scheme.contains("light") {
                "dark".to_string()
            } else {
                "light".to_string()
            };
        });
    };

    view! { cx,
        // Injecting
        <Html class=move || format!("h-full w-full {}", color_scheme())/>

        <nav>
            <div class="flex flex-row justify-between p-4 gap-8">
                <a href="/">
                    <div
                        id="logo"
                        class="flex flex-row flex-none w-32 justify-start items-center"
                    >
                        <img
                            class="w-10 rounded-full shadow-xl"
                            src="assets/logo.png"
                            alt="Personal Assisstant"
                        />
                        <div class="flex flex-col p-2">
                            <span class="self-center text-2xl font-bold select-none">
                                "Assisstant"
                            </span>
                            <span class="self-center text-xs font-bold select-none">" by Lablack Mourad"</span>
                        </div>
                    </div>
                </a>
                <div
                    id="nav-list"
                    class="hidden lg:flex justify-center grow md:w-auto"
                >
                    <ul class="flex flex-row justify-center items-center gap-x-8">
                        {routes
                            .clone()
                            .into_iter()
                            .map(|(path, name)| {
                                view! { cx,
                                    <li class="hover:text-sky-800 dark:hover:text-sky-300 hover:underline hover:font-semibold hover:scale-125 transition-all duration-900 ease-in-out">
                                        <a href=path>{name}</a>
                                    </li>
                                }
                            })
                            .collect_view(cx)}
                    </ul>
                </div>

                <div
                    id="nav-menu"
                    class="flex flex-row flex-none w-32 justify-end items-center gap-4"
                >
                    <button on:click=dark_mode_toggle>
                        <Icon class="h-5 w-5" icon=icon!(CgDarkMode)/>
                    </button>
                    <button
                        on:click=menu_clicked
                        class="text-3xl lg:hidden cursor-pointer animate-pulse"
                    >
                        <Icon class="h-5 w-5" icon=icon!(CgMenu)/>
                    </button>
                </div>
            </div>
            <div
                id="nav-list-small"
                class=move || {
                    if menu_icon().contains("menu") { "hidden" } else { "box" }
                }
            >
                <ul class="flex lg:hidden flex-col items-center gap-y-6">
                    {routes
                        .into_iter()
                        .map(|(path, name)| {
                            view! { cx,
                                <li class="hover:text-sky-200 hover:underline hover:font-semibold hover:scale-125 transition-all duration-900 ease-in-out">
                                    <a href=path on:click=menu_clicked>
                                        {name}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view(cx)}
                </ul>
            </div>
        </nav>
    }
}
