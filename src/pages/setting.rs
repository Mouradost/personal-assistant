use std::path::PathBuf;
use std::str::FromStr;

use leptos::*;
use leptos_icons::*;
use tauri_sys::{dialog, tauri};

use crate::{ModelArchitecture, ModelConfig, ModelConfigState, PayloadModelConfig, PayloadId, ModelParameters, PayloadModelParams, ModelParams};

// FIXME: We are not using correctly the signals there is some signals that might not be working as
// intended
#[component]
pub fn Setting(cx: Scope) -> impl IntoView {
    // Setup the signals
    let (is_model_connected, set_is_model_connected) =
        use_context::<(ReadSignal<bool>, WriteSignal<bool>)>(cx)
            .expect("to have found the setter and getter provided for model status");
    let (model_config_loaded, set_model_config_loaded) =
        use_context::<(ReadSignal<ModelConfigState>, WriteSignal<ModelConfigState>)>(cx)
            .expect("to have found the setter and getter provided for model config state");
    let (model_configs, _set_model_configs) =
        use_context::<(ReadSignal<Vec<ModelConfig>>, WriteSignal<Vec<ModelConfig>>)>(cx)
            .expect("to have found the setter and getter provided for model config state");
    let (model_params, _) =
    use_context::<(ReadSignal<ModelParameters>, WriteSignal<ModelParameters>)>(cx)
        .expect("to have found the setter and getter provided for model status");
    let (model_config, set_model_config) = create_signal(cx, ModelConfig::default());
    let (model_file_path, set_model_file_path) = create_signal(cx, String::new());

    // Setup the on_click functions
    let on_click_load_unload_model = move |ev| {
        log!("on_click_load_unload_model: {ev:#?}");
        if event_target_checked(&ev) {
            spawn_local(async move {
                let model_params_payload = PayloadModelParams{
                    params: ModelParams {
                        model_params: model_params()
                    }
                };
                // match tauri::invoke::<_, String>("load_dynamic_model_simulated", &()).await {
                match tauri::invoke::<PayloadModelParams, String>("load_dynamic_model", &model_params_payload).await {
                    Ok(msg) => {
                        set_is_model_connected
                            .update(|is_model_connected| *is_model_connected = true);
                        match dialog::MessageDialogBuilder::new()
                            .set_title("Model loading")
                            .set_kind(dialog::MessageDialogKind::Info)
                            .message(msg.as_str())
                            .await
                        {
                            Ok(()) => (),
                            Err(err) => error!("Dialog model loading: {err}"),
                        };
                    }
                    Err(err) => {
                        set_is_model_connected
                            .update(|is_model_connected| *is_model_connected = false);
                        match dialog::MessageDialogBuilder::new()
                            .set_title("invoking load_dynamic_model")
                            .set_kind(dialog::MessageDialogKind::Error)
                            .message(format!("{err}").as_str())
                            .await
                        {
                            Ok(()) => (),
                            Err(err) => error!("Dialog model loading: {err}"),
                        };
                    }
                };
            });
        } else {
            spawn_local(async move {
                // match tauri::invoke::<_, String>("unload_dynamic_model_simulated", &()).await {
                match tauri::invoke::<_, String>("unload_dynamic_model", &()).await {
                    Ok(msg) => {
                        set_is_model_connected
                            .update(|is_model_connected| *is_model_connected = false);
                        match dialog::MessageDialogBuilder::new()
                            .set_title("Model unloading")
                            .set_kind(dialog::MessageDialogKind::Info)
                            .message(msg.as_str())
                            .await
                        {
                            Ok(()) => (),
                            Err(err) => error!("Dialog model unloading: {err}"),
                        };
                    }
                    Err(err) => {
                        set_is_model_connected
                            .update(|is_model_connected| *is_model_connected = true);
                        match dialog::MessageDialogBuilder::new()
                            .set_title("Model unloading")
                            .set_kind(dialog::MessageDialogKind::Error)
                            .message(format!("{err:#?}").as_str())
                            .await
                        {
                            Ok(()) => (),
                            Err(err) => error!("Dialog error model unloading: {err}"),
                        };
                    }
                };
            });
        }
    };

    let on_click_add_model_config = move |ev: leptos::ev::SubmitEvent| {
        log!("on_click_load_model_config: {ev:#?}");
        ev.prevent_default();
        spawn_local(async move {
            let model_config_payload = PayloadModelConfig {
                model_config: model_config(),
            };
            match tauri::invoke::<PayloadModelConfig, String>(
                "add_model_config",
                &model_config_payload,
            )
            .await
            {
                Ok(msg) => {
                    set_is_model_connected.update(|is_model_connected| *is_model_connected = false);
                    match dialog::MessageDialogBuilder::new()
                        .set_title("New model config")
                        .set_kind(dialog::MessageDialogKind::Info)
                        .message(msg.as_str())
                        .await
                    {
                        Ok(()) => (),
                        Err(err) => error!("Dialog new model config: {err}"),
                    };
                }
                Err(err) => {
                    set_is_model_connected.update(|is_model_connected| *is_model_connected = true);
                    match dialog::MessageDialogBuilder::new()
                        .set_title("Add new model config")
                        .set_kind(dialog::MessageDialogKind::Error)
                        .message(format!("{err:#?}").as_str())
                        .await
                    {
                        Ok(()) => (),
                        Err(err) => error!("Dialog error add new model config: {err}"),
                    };
                }
            };
        });
    };

    // let on_click_load_model_config = move |ev: leptos::ev::SubmitEvent| {
    let on_click_load_model_config =
        move |ev: leptos::ev::MouseEvent, selected_model_config: ModelConfig| {
            log!("on_click_load_model_config: {ev:#?}");
            ev.prevent_default();
            spawn_local(async move {
                let current_model_config = PayloadModelConfig {
                    model_config: selected_model_config,
                };
                // match tauri::invoke::<_, String>("load_model_config_simulated", &current_model_config).await {
                match tauri::invoke::<_, String>("load_model_config", &current_model_config).await {
                    Ok(msg) => {
                        set_model_config_loaded(ModelConfigState(true));
                        log!("Loading the model config with response: {msg}")
                    }
                    Err(err) => {
                        set_model_config_loaded(ModelConfigState(false));
                        error!("Got an error while invoking load_model_config: {err}")
                    }
                };
            });
        };

    let on_click_delete_model_config =
        move |ev: leptos::ev::MouseEvent, selected_model_config_name: String| {
            log!("on_click_delete_model_config: {selected_model_config_name:#?}");
            ev.prevent_default();
            spawn_local(async move {
                let payload_id = PayloadId {
                    name: selected_model_config_name,
                };
                match tauri::invoke::<PayloadId, String>("delete_model_config", &payload_id)
                    .await
                {
                    Ok(msg) => {
                        log!("Model config deleted with response: {msg}")
                    }
                    Err(err) => {
                        error!("Got an error while invoking delete_model_config: {err}")
                    }
                };
            });
        };

    let on_click_open_file = move |ev: leptos::ev::MouseEvent| {
        log!("on_click_open_file: {ev:#?}");
        ev.prevent_default();
        spawn_local(async move {
            match dialog::FileDialogBuilder::new()
                .set_title("Pick a model")
                .add_filter("Binary (.bin)", &["bin"])
                .pick_file()
                .await
            {
                Ok(Some(file_path)) => {
                    set_model_file_path
                        .set(file_path.into_os_string().into_string().unwrap_or_default());
                    // log!("Getting the model file path {:#?}", model_config())
                    log!("Getting the model file path {:#?}", model_file_path())
                }
                Ok(None) => {
                    warn!("Model file path picking canceled")
                }
                Err(err) => {
                    error!("Got an error while invoking load_model_config: {err}")
                }
            };
        });
    };

    // Effects
    create_effect(cx, move |past| {
        if !model_file_path.with(String::is_empty) {
            log! {"{:#?}", past};
            set_model_config.update(|model_config| {
                let model_path = PathBuf::from(model_file_path());
                model_config.name = format!(
                    "{}",
                    model_path
                        .clone()
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default()
                );
                model_config.model_path = model_path;
            })
        }
    });

    // Body
    view! { cx,
        // Load model config
        <div class="flex-0 flex flex-row justify-between border border-gray-700 rounded-lg m-2 p-2">
            <h2>"Load the model"</h2>
            <input
                type="checkbox"
                class="toggle toggle-success"
                prop:disabled=move || !model_config_loaded().0
                on:change=on_click_load_unload_model
                prop:checked=is_model_connected
            />
        </div>
        // Advance settings
        <div class="flex-0 flex flex-row justify-between border border-gray-700 rounded-lg m-2 p-2">
            <ModelParamsDiv disabled=is_model_connected/>
        </div>
        // Model Path
        <div
            class="flex flex-col flex-0 border border-gray-700 rounded-lg m-2 p-2"
            prop:disabled=is_model_connected
        >
            <h2 class="mb-3">"Add a model"</h2>
            <form
                class="flex flex-row justify-between gap-4"
                // on:submit=on_click_load_model_config
                on:submit=on_click_add_model_config
            >
                <button
                    class="flex-1 flex flex-row"
                    required=true
                    on:click=on_click_open_file
                    prop:disabled=is_model_connected
                >
                    <div class="btn" prop:disabled=is_model_connected>
                        <Icon class="h-5 w-5" icon=icon!(BsFileEarmarkBinary)/>
                    </div>
                    <input
                        type="text"
                        class="input w-full mx-2 cursor-pointer"
                        placeholder="Model path"
                        required=true
                        readonly=true
                        prop:disabled=is_model_connected
                        prop:value=model_file_path
                        on:input=move |ev| set_model_file_path(
                            event_target_value(&ev),
                        )
                    />
                </button>
                // Model Architecture
                <select
                    class="select flex-shrink max-w-xs"
                    required=true
                    prop:disabled=is_model_connected
                    on:change=move |ev| {
                        set_model_config
                            .update(|model_config| {
                                model_config
                                    .model_architecture = ModelArchitecture::from_str(
                                        event_target_value(&ev).as_str(),
                                    )
                                    .unwrap_or_default();
                            })
                    }
                >
                    <option disabled=true selected=true>
                        "Choose the model type"
                    </option>
                    <option>"Bloom"</option>
                    <option>"Gpt2"</option>
                    <option>"GptJ"</option>
                    <option>"GptNeoX"</option>
                    <option>"Llama"</option>
                    <option>"Mpt"</option>
                // <option>"Falcon"</option>
                </select>
                // Upload btn
                <button
                    type="submit"
                    class="btn"
                    prop:disabled=is_model_connected
                >
                    <Icon class="h-5 w-5" icon=icon!(BsDatabaseAdd)/>
                </button>
            </form>
        </div>
        // Availible models
        <div
            class="flex-1 overflow-scroll border border-gray-700 rounded-lg m-2 p-2"
            prop:disabled=is_model_connected
        >
            <h2 class="mb-3">"Availible models"</h2>
            <div>
                <table class="table" prop:disabled=is_model_connected>
                    <thead>
                        <tr>
                            <th>"Name"</th>
                            <th>"Path"</th>
                            <th>"Architecture"</th>
                            <th>"Tokinizer"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=model_configs
                            key=|model| model.name.clone()
                            view=move |cx, model: ModelConfig| {
                                let model_clone = model.clone();
                                view! { cx,
                                    <tr>
                                        <th>{model.name.clone()}</th>
                                        <th>
                                            {format!(
                                                "{}", model.model_path.clone().into_os_string()
                                                .into_string().unwrap_or_default()
                                            )}
                                        </th>
                                        <th>{format!("{:?}", model.model_architecture)}</th>
                                        <th>{format!("{:?}", model.tokenizer_source)}</th>
                                        <th>
                                            <button
                                                class="btn"
                                                // prop:disabled=is_model_connected
                                                disabled=is_model_connected
                                                on:click=move |ev| {
                                                    on_click_load_model_config(ev, model.clone())
                                                }
                                            >
                                                <Icon class="h-5 w-5" icon=icon!(BiUploadRegular)/>
                                            </button>
                                            <button
                                                class="btn"
                                                // prop:disabled=is_model_connected
                                                disabled=is_model_connected
                                                on:click=move |ev| {
                                                    on_click_delete_model_config(ev, model_clone.name.clone())
                                                }
                                            >
                                                <Icon class="h-5 w-5" icon=icon!(AiDeleteOutlined)/>
                                            </button>
                                        </th>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>

        </div>
    }
}


#[component]
fn ModelParamsDiv(cx: Scope, disabled: ReadSignal<bool>) -> impl IntoView{
    let (model_params, set_model_params) =
    use_context::<(ReadSignal<ModelParameters>, WriteSignal<ModelParameters>)>(cx)
        .expect("to have found the setter and getter provided for model status");

    create_effect(cx, move |_| {
        log!("{:#?}", model_params());
    });

    view! { cx,
        <div class="flex flex-col justify-between p-2 w-full">
            <div class="flex w-full justify-between">
                <h2>"Advanced"</h2>
                <button
                    class="btn"
                    prop:disabled=disabled
                    on:click=move |_| set_model_params(
                        ModelParameters::default(),
                    )
                >
                    "Reset"
                </button>
            </div>
            <div class="flex w-full justify-between p-2">
                <label for="use_gpu">"Use GPU"</label>
                <input
                    id="use_gpu"
                    type="checkbox"
                    prop:disabled=disabled
                    class="toggle toggle-success"
                    on:change=move |ev| {
                        set_model_params
                            .update(|model_params| {
                                model_params.use_gpu = event_target_checked(&ev);
                            })
                    }
                    prop:checked=move || model_params().use_gpu
                />
            </div>
            <div class="flex w-full justify-between p-2">
                <label for="prefer_mmap">"Prefer mmap"</label>
                <input
                    id="prefer_mmap"
                    type="checkbox"
                    prop:disabled=disabled
                    class="toggle toggle-success"
                    on:change=move |ev| {
                        set_model_params
                            .update(|model_params| {
                                model_params.prefer_mmap = event_target_checked(&ev);
                            })
                    }
                    prop:checked=move || model_params().prefer_mmap
                />
            </div>
            <div class="flex w-full justify-between p-2">
                <label class="whitespace-nowrap" for="use_gpu">
                    "Context-size"
                </label>
                <input
                    class="range mx-4"
                    id="context_size"
                    type="range"
                    min=512
                    max=4096
                    step=1
                    prop:disabled=disabled
                    prop:value=move || model_params().context_size
                    on:change=move |ev| {
                        set_model_params
                            .update(|model_params| {
                                log!("{}", event_target_value(& ev));
                                model_params
                                    .context_size = event_target_value(&ev)
                                    .parse::<usize>()
                                    .unwrap_or_default();
                            })
                    }
                />
                <span>{move || model_params().context_size}</span>
            </div>
        </div>
    }
}
