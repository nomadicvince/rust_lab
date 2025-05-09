use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ToolbarProps {
    pub on_sync: Callback<()>,
    pub is_syncing: bool,
    pub is_online: bool,
}

#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProps) -> Html {
    let on_sync_click = {
        let on_sync = props.on_sync.clone();
        Callback::from(move |_| {
            on_sync.emit(());
        })
    };

    html! {
        <div class="toolbar">
            <div class="logo">
                <h1>{ "CosmiqNotz" }</h1>
            </div>
            <div class="actions">
                <div class={classes!("status-indicator", if props.is_online { "online" } else { "offline" })}>
                    { if props.is_online { "Online" } else { "Offline" } }
                </div>
                <button 
                    onclick={on_sync_click}
                    disabled={props.is_syncing || !props.is_online}
                    class={classes!("sync-button", (props.is_syncing || !props.is_online).then(|| "disabled"))}
                >
                    { if props.is_syncing { "Syncing..." } else { "Sync" } }
                </button>
            </div>
        </div>
    }
}