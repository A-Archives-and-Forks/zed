use crate::{settings_store::SettingsStore, Setting};
use anyhow::Result;
use fs::Fs;
use futures::{channel::mpsc, StreamExt};
use gpui2::{AppContext, Executor};
use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
    str,
    sync::Arc,
    time::Duration,
};
use util::{paths, ResultExt};

pub fn register<T: Setting>(cx: &mut AppContext) {
    cx.update_global(|store: &mut SettingsStore, cx| {
        store.register_setting::<T>(cx);
    });
}

pub fn get<'a, T: Setting>(cx: &'a AppContext) -> &'a T {
    cx.global::<SettingsStore>().get(None)
}

pub fn get_local<'a, T: Setting>(location: Option<(usize, &Path)>, cx: &'a AppContext) -> &'a T {
    cx.global::<SettingsStore>().get(location)
}

pub const EMPTY_THEME_NAME: &'static str = "empty-theme";

#[cfg(any(test, feature = "test-support"))]
pub fn test_settings() -> String {
    let mut value = crate::settings_store::parse_json_with_comments::<serde_json::Value>(
        crate::default_settings().as_ref(),
    )
    .unwrap();
    util::merge_non_null_json_value_into(
        serde_json::json!({
            "buffer_font_family": "Courier",
            "buffer_font_features": {},
            "buffer_font_size": 14,
            "theme": EMPTY_THEME_NAME,
        }),
        &mut value,
    );
    value.as_object_mut().unwrap().remove("languages");
    serde_json::to_string(&value).unwrap()
}

pub fn watch_config_file(
    path: PathBuf,
    fs: Arc<dyn Fs>,
    executor: &Executor,
) -> mpsc::UnboundedReceiver<String> {
    let (tx, rx) = mpsc::unbounded();
    executor
        .spawn(async move {
            let events = fs.watch(&path, Duration::from_millis(100)).await;
            futures::pin_mut!(events);

            let contents = fs.load(&path).await.unwrap_or_default();
            if tx.unbounded_send(contents).is_err() {
                return;
            }

            loop {
                if events.next().await.is_none() {
                    break;
                }

                if let Ok(contents) = fs.load(&path).await {
                    if !tx.unbounded_send(contents).is_ok() {
                        break;
                    }
                }
            }
        })
        .detach();
    rx
}

pub fn handle_settings_file_changes(
    mut user_settings_file_rx: mpsc::UnboundedReceiver<String>,
    cx: &mut AppContext,
) {
    let user_settings_content = cx.executor().block(user_settings_file_rx.next()).unwrap();
    cx.update_global(|store: &mut SettingsStore, cx| {
        store
            .set_user_settings(&user_settings_content, cx)
            .log_err();
    });
    cx.spawn(move |mut cx| async move {
        while let Some(user_settings_content) = user_settings_file_rx.next().await {
            let result = cx.update_global(|store: &mut SettingsStore, cx| {
                store
                    .set_user_settings(&user_settings_content, cx)
                    .log_err();
                cx.refresh();
            });
            if result.is_err() {
                break; // App dropped
            }
        }
    })
    .detach();
}

async fn load_settings(fs: &Arc<dyn Fs>) -> Result<String> {
    match fs.load(&paths::SETTINGS).await {
        result @ Ok(_) => result,
        Err(err) => {
            if let Some(e) = err.downcast_ref::<std::io::Error>() {
                if e.kind() == ErrorKind::NotFound {
                    return Ok(crate::initial_user_settings_content().to_string());
                }
            }
            return Err(err);
        }
    }
}

pub fn update_settings_file<T: Setting>(
    fs: Arc<dyn Fs>,
    cx: &mut AppContext,
    update: impl 'static + Send + FnOnce(&mut T::FileContent),
) {
    cx.spawn(|cx| async move {
        let old_text = load_settings(&fs).await?;
        let new_text = cx.read_global(|store: &SettingsStore, _cx| {
            store.new_text_for_update::<T>(old_text, update)
        })?;
        fs.atomic_write(paths::SETTINGS.clone(), new_text).await?;
        anyhow::Ok(())
    })
    .detach_and_log_err(cx);
}
