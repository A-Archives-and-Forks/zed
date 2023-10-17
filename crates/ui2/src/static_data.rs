use std::path::PathBuf;
use std::str::FromStr;

use gpui3::WindowContext;
use rand::Rng;

use crate::{
    theme, Buffer, BufferRow, BufferRows, EditorPane, FileSystemStatus, GitStatus, HighlightColor,
    HighlightedLine, HighlightedText, Icon, Keybinding, Label, LabelColor, ListEntry,
    ListEntrySize, ListItem, Livestream, MicStatus, ModifierKeys, PaletteItem, Player,
    PlayerCallStatus, PlayerWithCallStatus, ScreenShareStatus, Symbol, Tab, Theme, ToggleState,
    VideoStatus,
};

pub fn static_tabs_example<S: 'static + Send + Sync + Clone>() -> Vec<Tab<S>> {
    vec![
        Tab::new()
            .title("wip.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new()
            .title("Cargo.toml".to_string())
            .icon(Icon::FileToml)
            .current(false)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("Channels Panel".to_string())
            .icon(Icon::Hash)
            .current(false),
        Tab::new()
            .title("channels_panel.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("workspace.rs".to_string())
            .current(false)
            .icon(Icon::FileRust)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("icon_button.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("storybook.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .git_status(GitStatus::Created),
        Tab::new()
            .title("theme.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("theme_registry.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("styleable_helpers.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
    ]
}

pub fn static_tabs_1<S: 'static + Send + Sync + Clone>() -> Vec<Tab<S>> {
    vec![
        Tab::new()
            .title("project_panel.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new()
            .title("tab_bar.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .git_status(GitStatus::Modified),
        Tab::new()
            .title("workspace.rs".to_string())
            .icon(Icon::FileRust)
            .current(false),
        Tab::new()
            .title("tab.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
    ]
}

pub fn static_tabs_2<S: 'static + Send + Sync + Clone>() -> Vec<Tab<S>> {
    vec![
        Tab::new()
            .title("tab_bar.rs".to_string())
            .icon(Icon::FileRust)
            .current(false)
            .fs_status(FileSystemStatus::Deleted),
        Tab::new()
            .title("static_data.rs".to_string())
            .icon(Icon::FileRust)
            .current(true)
            .git_status(GitStatus::Modified),
    ]
}

pub fn static_tabs_3<S: 'static + Send + Sync + Clone>() -> Vec<Tab<S>> {
    vec![Tab::new().git_status(GitStatus::Created).current(true)]
}

pub fn static_players() -> Vec<Player> {
    vec![
        Player::new(
            0,
            "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
            "nathansobo".into(),
        ),
        Player::new(
            1,
            "https://avatars.githubusercontent.com/u/326587?v=4".into(),
            "maxbrunsfeld".into(),
        ),
        Player::new(
            2,
            "https://avatars.githubusercontent.com/u/482957?v=4".into(),
            "as-cii".into(),
        ),
        Player::new(
            3,
            "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
            "iamnbutler".into(),
        ),
        Player::new(
            4,
            "https://avatars.githubusercontent.com/u/1486634?v=4".into(),
            "maxdeviant".into(),
        ),
    ]
}

#[derive(Debug)]
pub struct PlayerData {
    pub url: String,
    pub name: String,
}

pub fn static_player_data() -> Vec<PlayerData> {
    vec![
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1714999?v=4".into(),
            name: "iamnbutler".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/326587?v=4".into(),
            name: "maxbrunsfeld".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/482957?v=4".into(),
            name: "as-cii".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1789?v=4".into(),
            name: "nathansobo".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1486634?v=4".into(),
            name: "ForLoveOfCats".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/2690773?v=4".into(),
            name: "SomeoneToIgnore".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/19867440?v=4".into(),
            name: "JosephTLyons".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/24362066?v=4".into(),
            name: "osiewicz".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/22121886?v=4".into(),
            name: "KCaverly".into(),
        },
        PlayerData {
            url: "https://avatars.githubusercontent.com/u/1486634?v=4".into(),
            name: "maxdeviant".into(),
        },
    ]
}

pub fn create_static_players(player_data: Vec<PlayerData>) -> Vec<Player> {
    let mut players = Vec::new();
    for data in player_data {
        players.push(Player::new(players.len(), data.url, data.name));
    }
    players
}

pub fn static_player_1(data: &Vec<PlayerData>) -> Player {
    Player::new(1, data[0].url.clone(), data[0].name.clone())
}

pub fn static_player_2(data: &Vec<PlayerData>) -> Player {
    Player::new(2, data[1].url.clone(), data[1].name.clone())
}

pub fn static_player_3(data: &Vec<PlayerData>) -> Player {
    Player::new(3, data[2].url.clone(), data[2].name.clone())
}

pub fn static_player_4(data: &Vec<PlayerData>) -> Player {
    Player::new(4, data[3].url.clone(), data[3].name.clone())
}

pub fn static_player_5(data: &Vec<PlayerData>) -> Player {
    Player::new(5, data[4].url.clone(), data[4].name.clone())
}

pub fn static_player_6(data: &Vec<PlayerData>) -> Player {
    Player::new(6, data[5].url.clone(), data[5].name.clone())
}

pub fn static_player_7(data: &Vec<PlayerData>) -> Player {
    Player::new(7, data[6].url.clone(), data[6].name.clone())
}

pub fn static_player_8(data: &Vec<PlayerData>) -> Player {
    Player::new(8, data[7].url.clone(), data[7].name.clone())
}

pub fn static_player_9(data: &Vec<PlayerData>) -> Player {
    Player::new(9, data[8].url.clone(), data[8].name.clone())
}

pub fn static_player_10(data: &Vec<PlayerData>) -> Player {
    Player::new(10, data[9].url.clone(), data[9].name.clone())
}

pub fn static_livestream() -> Livestream {
    Livestream {
        players: random_players_with_call_status(7),
        channel: Some("gpui2-ui".to_string()),
    }
}

pub fn populate_player_call_status(
    player: Player,
    followers: Option<Vec<Player>>,
) -> PlayerCallStatus {
    let mut rng = rand::thread_rng();
    let in_current_project: bool = rng.gen();
    let disconnected: bool = rng.gen();
    let voice_activity: f32 = rng.gen();
    let mic_status = if rng.gen_bool(0.5) {
        MicStatus::Muted
    } else {
        MicStatus::Unmuted
    };
    let video_status = if rng.gen_bool(0.5) {
        VideoStatus::On
    } else {
        VideoStatus::Off
    };
    let screen_share_status = if rng.gen_bool(0.5) {
        ScreenShareStatus::Shared
    } else {
        ScreenShareStatus::NotShared
    };
    PlayerCallStatus {
        mic_status,
        voice_activity,
        video_status,
        screen_share_status,
        in_current_project,
        disconnected,
        following: None,
        followers,
    }
}

pub fn random_players_with_call_status(number_of_players: usize) -> Vec<PlayerWithCallStatus> {
    let players = create_static_players(static_player_data());
    let mut player_status = vec![];
    for i in 0..number_of_players {
        let followers = if i == 0 {
            Some(vec![
                players[1].clone(),
                players[3].clone(),
                players[5].clone(),
                players[6].clone(),
            ])
        } else if i == 1 {
            Some(vec![players[2].clone(), players[6].clone()])
        } else {
            None
        };
        let call_status = populate_player_call_status(players[i].clone(), followers);
        player_status.push(PlayerWithCallStatus::new(players[i].clone(), call_status));
    }
    player_status
}

pub fn static_players_with_call_status() -> Vec<PlayerWithCallStatus> {
    let players = static_players();
    let mut player_0_status = PlayerCallStatus::new();
    let player_1_status = PlayerCallStatus::new();
    let player_2_status = PlayerCallStatus::new();
    let mut player_3_status = PlayerCallStatus::new();
    let mut player_4_status = PlayerCallStatus::new();

    player_0_status.screen_share_status = ScreenShareStatus::Shared;
    player_0_status.followers = Some(vec![players[1].clone(), players[3].clone()]);

    player_3_status.voice_activity = 0.5;
    player_4_status.mic_status = MicStatus::Muted;
    player_4_status.in_current_project = false;

    vec![
        PlayerWithCallStatus::new(players[0].clone(), player_0_status),
        PlayerWithCallStatus::new(players[1].clone(), player_1_status),
        PlayerWithCallStatus::new(players[2].clone(), player_2_status),
        PlayerWithCallStatus::new(players[3].clone(), player_3_status),
        PlayerWithCallStatus::new(players[4].clone(), player_4_status),
    ]
}

pub fn static_project_panel_project_items<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("zed"))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(0)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new(".cargo"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new(".config"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new(".git").color(LabelColor::Hidden))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new(".cargo"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new(".idea").color(LabelColor::Hidden))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("assets"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("cargo-target").color(LabelColor::Hidden))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("crates"))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("activity_indicator"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2),
        ListEntry::new(Label::new("ai"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2),
        ListEntry::new(Label::new("audio"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2),
        ListEntry::new(Label::new("auto_update"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2),
        ListEntry::new(Label::new("breadcrumbs"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2),
        ListEntry::new(Label::new("call"))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2),
        ListEntry::new(Label::new("sqlez").color(LabelColor::Modified))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2)
            .set_toggle(ToggleState::NotToggled),
        ListEntry::new(Label::new("gpui2"))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("src"))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("derive_element.rs"))
            .set_left_icon(Icon::FileRust.into())
            .set_indent_level(4),
        ListEntry::new(Label::new("storybook").color(LabelColor::Modified))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(1)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("docs").color(LabelColor::Default))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(2)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("src").color(LabelColor::Modified))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(3)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("ui").color(LabelColor::Modified))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(4)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("component").color(LabelColor::Created))
            .set_left_icon(Icon::FolderOpen.into())
            .set_indent_level(5)
            .set_toggle(ToggleState::Toggled),
        ListEntry::new(Label::new("facepile.rs").color(LabelColor::Default))
            .set_left_icon(Icon::FileRust.into())
            .set_indent_level(6),
        ListEntry::new(Label::new("follow_group.rs").color(LabelColor::Default))
            .set_left_icon(Icon::FileRust.into())
            .set_indent_level(6),
        ListEntry::new(Label::new("list_item.rs").color(LabelColor::Created))
            .set_left_icon(Icon::FileRust.into())
            .set_indent_level(6),
        ListEntry::new(Label::new("tab.rs").color(LabelColor::Default))
            .set_left_icon(Icon::FileRust.into())
            .set_indent_level(6),
        ListEntry::new(Label::new("target").color(LabelColor::Hidden))
            .set_left_icon(Icon::Folder.into())
            .set_indent_level(1),
        ListEntry::new(Label::new(".dockerignore"))
            .set_left_icon(Icon::FileGeneric.into())
            .set_indent_level(1),
        ListEntry::new(Label::new(".DS_Store").color(LabelColor::Hidden))
            .set_left_icon(Icon::FileGeneric.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("Cargo.lock"))
            .set_left_icon(Icon::FileLock.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("Cargo.toml"))
            .set_left_icon(Icon::FileToml.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("Dockerfile"))
            .set_left_icon(Icon::FileGeneric.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("Procfile"))
            .set_left_icon(Icon::FileGeneric.into())
            .set_indent_level(1),
        ListEntry::new(Label::new("README.md"))
            .set_left_icon(Icon::FileDoc.into())
            .set_indent_level(1),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_project_panel_single_items<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("todo.md"))
            .set_left_icon(Icon::FileDoc.into())
            .set_indent_level(0),
        ListEntry::new(Label::new("README.md"))
            .set_left_icon(Icon::FileDoc.into())
            .set_indent_level(0),
        ListEntry::new(Label::new("config.json"))
            .set_left_icon(Icon::FileGeneric.into())
            .set_indent_level(0),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_notification_items<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("maxbrunsfeld acceped your contact request."))
            .set_left_icon(Icon::Check.into())
            .set_indent_level(0),
        ListEntry::new(Label::new("nathansobo acceped your contact request."))
            .set_left_icon(Icon::Check.into())
            .set_indent_level(0),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_collab_panel_current_call<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("as-cii")).set_left_avatar("http://github.com/as-cii.png?s=50"),
        ListEntry::new(Label::new("nathansobo"))
            .set_left_avatar("http://github.com/nathansobo.png?s=50"),
        ListEntry::new(Label::new("maxbrunsfeld"))
            .set_left_avatar("http://github.com/maxbrunsfeld.png?s=50"),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn static_collab_panel_channels<S: 'static + Send + Sync + Clone>() -> Vec<ListItem<S>> {
    vec![
        ListEntry::new(Label::new("zed"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(0),
        ListEntry::new(Label::new("community"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(1),
        ListEntry::new(Label::new("dashboards"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("feedback"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("teams-in-channels-alpha"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("current-projects"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(1),
        ListEntry::new(Label::new("codegen"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("gpui2"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("livestreaming"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("open-source"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("replace"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("semantic-index"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("vim"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
        ListEntry::new(Label::new("web-tech"))
            .set_left_icon(Icon::Hash.into())
            .set_size(ListEntrySize::Medium)
            .set_indent_level(2),
    ]
    .into_iter()
    .map(From::from)
    .collect()
}

pub fn example_editor_actions<S: 'static + Send + Sync + Clone>() -> Vec<PaletteItem<S>> {
    vec![
        PaletteItem::new("New File").keybinding(Keybinding::new(
            "N".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Open File").keybinding(Keybinding::new(
            "O".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Save File").keybinding(Keybinding::new(
            "S".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Cut").keybinding(Keybinding::new(
            "X".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Copy").keybinding(Keybinding::new(
            "C".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Paste").keybinding(Keybinding::new(
            "V".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Undo").keybinding(Keybinding::new(
            "Z".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Redo").keybinding(Keybinding::new(
            "Z".to_string(),
            ModifierKeys::new().command(true).shift(true),
        )),
        PaletteItem::new("Find").keybinding(Keybinding::new(
            "F".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Replace").keybinding(Keybinding::new(
            "R".to_string(),
            ModifierKeys::new().command(true),
        )),
        PaletteItem::new("Jump to Line"),
        PaletteItem::new("Select All"),
        PaletteItem::new("Deselect All"),
        PaletteItem::new("Switch Document"),
        PaletteItem::new("Insert Line Below"),
        PaletteItem::new("Insert Line Above"),
        PaletteItem::new("Move Line Up"),
        PaletteItem::new("Move Line Down"),
        PaletteItem::new("Toggle Comment"),
        PaletteItem::new("Delete Line"),
    ]
}

pub fn empty_editor_example(cx: &mut WindowContext) -> EditorPane {
    EditorPane::new(
        cx,
        static_tabs_example(),
        PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        vec![],
        empty_buffer_example(),
    )
}

pub fn empty_buffer_example<S: 'static + Send + Sync + Clone>() -> Buffer<S> {
    Buffer::new().set_rows(Some(BufferRows::default()))
}

pub fn hello_world_rust_editor_example(cx: &mut WindowContext) -> EditorPane {
    let theme = theme(cx);

    EditorPane::new(
        cx,
        static_tabs_example(),
        PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        vec![Symbol(vec![
            HighlightedText {
                text: "fn ".to_string(),
                color: HighlightColor::Keyword.hsla(&theme),
            },
            HighlightedText {
                text: "main".to_string(),
                color: HighlightColor::Function.hsla(&theme),
            },
        ])],
        hello_world_rust_buffer_example(&theme),
    )
}

pub fn hello_world_rust_buffer_example<S: 'static + Send + Sync + Clone>(
    theme: &Theme,
) -> Buffer<S> {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_buffer_rows(theme),
        }))
}

pub fn hello_world_rust_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Statements here are executed when the compiled binary is called."
                        .to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 3,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 4,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Print text to the console.".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 5,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "    println!(".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".to_string(),
                        color: HighlightColor::String.hsla(&theme),
                    },
                    HighlightedText {
                        text: ");".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 6,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}

pub fn hello_world_rust_editor_with_status_example(cx: &mut WindowContext) -> EditorPane {
    let theme = theme(cx);

    EditorPane::new(
        cx,
        static_tabs_example(),
        PathBuf::from_str("crates/ui/src/static_data.rs").unwrap(),
        vec![Symbol(vec![
            HighlightedText {
                text: "fn ".to_string(),
                color: HighlightColor::Keyword.hsla(&theme),
            },
            HighlightedText {
                text: "main".to_string(),
                color: HighlightColor::Function.hsla(&theme),
            },
        ])],
        hello_world_rust_buffer_with_status_example(&theme),
    )
}

pub fn hello_world_rust_buffer_with_status_example<S: 'static + Send + Sync + Clone>(
    theme: &Theme,
) -> Buffer<S> {
    Buffer::new()
        .set_title("hello_world.rs".to_string())
        .set_path("src/hello_world.rs".to_string())
        .set_language("rust".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: true,
            rows: hello_world_rust_with_status_buffer_rows(theme),
        }))
}

pub fn hello_world_rust_with_status_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = true;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: true,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "fn ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "main".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "() {".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "// Statements here are executed when the compiled binary is called."
                        .to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Modified,
            show_line_number,
        },
        BufferRow {
            line_number: 3,
            code_action: false,
            current: false,
            line: None,
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 4,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "    // Print text to the console.".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 5,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "    println!(".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "\"Hello, world!\"".to_string(),
                        color: HighlightColor::String.hsla(&theme),
                    },
                    HighlightedText {
                        text: ");".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 6,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "}".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 7,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "".to_string(),
                    color: HighlightColor::Default.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
        BufferRow {
            line_number: 8,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "// Marshall and Nate were here".to_string(),
                    color: HighlightColor::Comment.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::Created,
            show_line_number,
        },
    ]
}

pub fn terminal_buffer<S: 'static + Send + Sync + Clone>(theme: &Theme) -> Buffer<S> {
    Buffer::new()
        .set_title("zed — fish".to_string())
        .set_rows(Some(BufferRows {
            show_line_numbers: false,
            rows: terminal_buffer_rows(theme),
        }))
}

pub fn terminal_buffer_rows(theme: &Theme) -> Vec<BufferRow> {
    let show_line_number = false;

    vec![
        BufferRow {
            line_number: 1,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![
                    HighlightedText {
                        text: "maxdeviant ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                    HighlightedText {
                        text: "in ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "profaned-capital ".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "in ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: "~/p/zed ".to_string(),
                        color: HighlightColor::Function.hsla(&theme),
                    },
                    HighlightedText {
                        text: "on ".to_string(),
                        color: HighlightColor::Default.hsla(&theme),
                    },
                    HighlightedText {
                        text: " gpui2-ui ".to_string(),
                        color: HighlightColor::Keyword.hsla(&theme),
                    },
                ],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
        BufferRow {
            line_number: 2,
            code_action: false,
            current: false,
            line: Some(HighlightedLine {
                highlighted_texts: vec![HighlightedText {
                    text: "λ ".to_string(),
                    color: HighlightColor::String.hsla(&theme),
                }],
            }),
            cursors: None,
            status: GitStatus::None,
            show_line_number,
        },
    ]
}
