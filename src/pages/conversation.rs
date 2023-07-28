use crate::{components::Chat, Entity, Message, Payload, InferenceStats};
use leptos::*;
use leptos_icons::*;
use tauri_sys::{tauri, dialog};

#[component]
pub fn Conversation(cx: Scope) -> impl IntoView {
    let (is_model_connected, _) = use_context::<(ReadSignal<bool>, WriteSignal<bool>)>(cx)
        .expect("to have found the getter provided for model status");
    let (messages, set_messages) =
        use_context::<(ReadSignal<Vec<Message>>, WriteSignal<Vec<Message>>)>(cx)
            .expect("to have found the setter and getter provided for messages");
    let (user_input, set_user_input) = create_signal(cx, String::new());
    let (is_valid_template, set_is_valid_template) = create_signal(cx, true);
    let (is_model_predicting, set_is_model_predicting) = create_signal(cx, false);
    let (prompt, set_prompt) = create_signal(cx, r#"A chat between a human ("User") and an AI assistant ("Assistant"). The assistant gives helpful, detailed, and polite answers to the human's questions.
Assistant: How may I help you?
User: {{PROMPT}}
Assistant: "#.to_string());

    create_effect(cx, move |_| {
        set_is_valid_template(prompt().contains("{{PROMPT}}"));
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_messages.update(|messages| {
            messages.push(Message {
                content: user_input(),
                ..Default::default()
            })
        });
        set_messages.update(|messages| {
            messages.push(Message {
                content: "".into(),
                entity: Entity::Bot,
                is_loading: true,
                ..Default::default()
            })
        });
        set_is_model_predicting.set(true);
        let payload = Payload {
            message: prompt().replace("{{PROMPT}}", user_input().as_ref()),
        };
        log!("Payload\n{payload:#?}");
        set_user_input.update(|user_input| user_input.clear());

        spawn_local(async move {
            // match tauri::invoke::<_, InferenceStats>("predict_simulated", &payload).await {
            match tauri::invoke::<_, InferenceStats>("predict", &payload).await {
                Ok(stats) => {
                    set_messages.update(|messages| messages.last_mut().unwrap().done());
                    set_is_model_predicting.set(false);
                        match dialog::MessageDialogBuilder::new()
                            .set_title("Model prediction")
                            .set_kind(dialog::MessageDialogKind::Info)
                            .message(format!("{stats}").as_str())
                            .await
                        {
                            Ok(()) => (),
                            Err(err) => error!("Dialog info model prediction: {err}"),
                        };
                },
                Err(err) => {
                    set_messages.update(|messages| messages.last_mut().unwrap().done());
                    set_is_model_predicting.set(false);
                    match dialog::MessageDialogBuilder::new()
                        .set_title("Model prediction")
                        .set_kind(dialog::MessageDialogKind::Error)
                        .message(format!("{err:#?}").as_str())
                        .await
                    {
                        Ok(()) => (),
                        Err(err) => error!("Dialog error model prediction: {err}"),
                    };
                },
            };
        });
    };

    view! { cx,
        // Prompt template
        <div class="flex-0 flex flex-col items-start border border-gray-700 rounded-lg m-2 p-2">
            <h2 class="mb-3">"Template"</h2>
            <textarea
                placeholder="Model Template"
                rows="4"
                prop:value=prompt
                on:change=move |ev| set_prompt(event_target_value(&ev))
                class=move || format!("textarea {} textarea-md w-full h-full", if is_valid_template() {"textarea-bordered"} else {"textarea-error"})
            ></textarea>
              <Show
                when=move || { !is_valid_template() }
                fallback=|cx| view! { cx, <p class="text-green-500 text-xs mx-2">"Valid template !"</p> }
              >
                <p class="text-red-500 text-xs mx-2">"The template should contain "<br>"{{PROMPT}}"</br>" which will be use to inject the user message !"</p>
              </Show>
        </div>
        // Conversation area"
        // There is a bug with using both justify-end and overflow-scroll
        <div class="flex-1 flex flex-col justify-end overflow-scroll border border-gray-700 rounded-lg m-2 p-2">
            {move || {
                messages()
                    .into_iter()
                    .map(|message| {
                        view! { cx, <Chat message=message/> }
                    })
                    .collect_view(cx)
            }}
        </div>
        // User input area
        <div class="flex-0">
            <form
                class="flex flex-row items-center border border-gray-700 rounded-lg m-2 p-2"
                on:submit=on_submit
            >
                <input
                    class="flex-1 input w-full mx-2"
                    placeholder="Say something to the AI !"
                    type="text"
                    prop:value=user_input
                    on:input=move |ev| set_user_input(event_target_value(&ev))
                />
                <button
                    type="submit"
                    prop:disabled=move || user_input.with(String::is_empty) | !is_valid_template() | !is_model_connected() | is_model_predicting()
                    class="btn flex-0 mx-2"
                >
                    <Icon class="h-5 w-5" icon=icon!(BsSendFill)/>
                </button>
            </form>
        </div>
    }
}
