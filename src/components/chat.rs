use leptos::*;

use crate::{Entity, Message};

#[component]
pub fn Chat(cx: Scope, message: Message) -> impl IntoView {
    let chat_class;
    let chat_bubble_class;
    let avatar_img;
    let header_str;
    match message.entity {
        Entity::User => {
            chat_class = "chat chat-end";
            chat_bubble_class = "chat-bubble chat-bubble-primary text-justify";
            avatar_img = "assets/user.png";
            header_str = "User";
        }
        Entity::Bot => {
            chat_class = "chat chat-start";
            chat_bubble_class = "chat-bubble chat-bubble-secondary text-justify";
            avatar_img = "assets/bot.webp";
            header_str = "AI";
        }
    };
    let loading = move || {
        message
            .is_loading
            .then(|| view! {cx, <span class="loading loading-dots loading-xs ms-2"></span>})
    };

    view! { cx,
        <div class=chat_class>
            <div class="chat-image avatar">
                <div class="w-10 rounded-full">
                    <img src=avatar_img/>
                </div>
            </div>
            <div class="chat-header">{header_str}</div>
            <div class=chat_bubble_class>
                <p>{message.content} {loading}</p>
            </div>
            <div class="chat-footer opacity-50">
                <time class="text-xs opacity-50">{message.time}</time>
            </div>
        </div>
    }
}
