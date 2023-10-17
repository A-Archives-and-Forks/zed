use std::marker::PhantomData;
use std::sync::Arc;

use gpui3::{svg, Hsla};
use strum::EnumIter;

use crate::prelude::*;
use crate::theme::{theme, Theme};

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    Small,
    #[default]
    Large,
}

#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconColor {
    #[default]
    Default,
    Muted,
    Disabled,
    Placeholder,
    Accent,
    Error,
    Warning,
    Success,
    Info,
}

impl IconColor {
    pub fn color(self, theme: Arc<Theme>) -> Hsla {
        match self {
            IconColor::Default => theme.lowest.base.default.foreground,
            IconColor::Muted => theme.lowest.variant.default.foreground,
            IconColor::Disabled => theme.lowest.base.disabled.foreground,
            IconColor::Placeholder => theme.lowest.base.disabled.foreground,
            IconColor::Accent => theme.lowest.accent.default.foreground,
            IconColor::Error => theme.lowest.negative.default.foreground,
            IconColor::Warning => theme.lowest.warning.default.foreground,
            IconColor::Success => theme.lowest.positive.default.foreground,
            IconColor::Info => theme.lowest.accent.default.foreground,
        }
    }
}

#[derive(Default, PartialEq, Copy, Clone, EnumIter)]
pub enum Icon {
    Ai,
    ArrowLeft,
    ArrowRight,
    ArrowUpRight,
    AudioOff,
    AudioOn,
    Bolt,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    Close,
    Check,
    ExclamationTriangle,
    File,
    FileGeneric,
    FileDoc,
    FileGit,
    FileLock,
    FileRust,
    FileToml,
    FileTree,
    Folder,
    FolderOpen,
    FolderX,
    #[default]
    Hash,
    InlayHint,
    MagicWand,
    MagnifyingGlass,
    Maximize,
    Menu,
    MessageBubbles,
    Mic,
    MicMute,
    Plus,
    Quote,
    Replace,
    ReplaceAll,
    Screen,
    SelectAll,
    Split,
    SplitMessage,
    Terminal,
    XCircle,
    Copilot,
    Envelope,
}

impl Icon {
    pub fn path(self) -> &'static str {
        match self {
            Icon::Ai => "icons/ai.svg",
            Icon::ArrowLeft => "icons/arrow_left.svg",
            Icon::ArrowRight => "icons/arrow_right.svg",
            Icon::ArrowUpRight => "icons/arrow_up_right.svg",
            Icon::AudioOff => "icons/speaker-off.svg",
            Icon::AudioOn => "icons/speaker-loud.svg",
            Icon::Bolt => "icons/bolt.svg",
            Icon::ChevronDown => "icons/chevron_down.svg",
            Icon::ChevronLeft => "icons/chevron_left.svg",
            Icon::ChevronRight => "icons/chevron_right.svg",
            Icon::ChevronUp => "icons/chevron_up.svg",
            Icon::Close => "icons/x.svg",
            Icon::Check => "icons/check.svg",
            Icon::ExclamationTriangle => "icons/warning.svg",
            Icon::File => "icons/file.svg",
            Icon::FileGeneric => "icons/file_icons/file.svg",
            Icon::FileDoc => "icons/file_icons/book.svg",
            Icon::FileGit => "icons/file_icons/git.svg",
            Icon::FileLock => "icons/file_icons/lock.svg",
            Icon::FileRust => "icons/file_icons/rust.svg",
            Icon::FileToml => "icons/file_icons/toml.svg",
            Icon::FileTree => "icons/project.svg",
            Icon::Folder => "icons/file_icons/folder.svg",
            Icon::FolderOpen => "icons/file_icons/folder_open.svg",
            Icon::FolderX => "icons/stop_sharing.svg",
            Icon::Hash => "icons/hash.svg",
            Icon::InlayHint => "icons/inlay_hint.svg",
            Icon::MagicWand => "icons/magic-wand.svg",
            Icon::MagnifyingGlass => "icons/magnifying_glass.svg",
            Icon::Maximize => "icons/maximize.svg",
            Icon::Menu => "icons/menu.svg",
            Icon::MessageBubbles => "icons/conversations.svg",
            Icon::Mic => "icons/mic.svg",
            Icon::MicMute => "icons/mic-mute.svg",
            Icon::Plus => "icons/plus.svg",
            Icon::Quote => "icons/quote.svg",
            Icon::Replace => "icons/replace.svg",
            Icon::ReplaceAll => "icons/replace_all.svg",
            Icon::Screen => "icons/desktop.svg",
            Icon::SelectAll => "icons/select-all.svg",
            Icon::Split => "icons/split.svg",
            Icon::SplitMessage => "icons/split_message.svg",
            Icon::Terminal => "icons/terminal.svg",
            Icon::XCircle => "icons/error.svg",
            Icon::Copilot => "icons/copilot.svg",
            Icon::Envelope => "icons/feedback.svg",
        }
    }
}

#[derive(Element, Clone)]
pub struct IconElement<S: 'static + Send + Sync> {
    state_type: PhantomData<S>,
    icon: Icon,
    color: IconColor,
    size: IconSize,
}

impl<S: 'static + Send + Sync> IconElement<S> {
    pub fn new(icon: Icon) -> Self {
        Self {
            state_type: PhantomData,
            icon,
            color: IconColor::default(),
            size: IconSize::default(),
        }
    }

    pub fn color(mut self, color: IconColor) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }

    fn render(&mut self, _view: &mut S, cx: &mut ViewContext<S>) -> impl Element<ViewState = S> {
        let theme = theme(cx);
        let fill = self.color.color(theme);

        let sized_svg = match self.size {
            IconSize::Small => svg().size_3p5(),
            IconSize::Large => svg().size_4(),
        };

        sized_svg.flex_none().path(self.icon.path()).fill(fill)
    }
}

#[cfg(feature = "stories")]
pub use stories::*;

#[cfg(feature = "stories")]
mod stories {
    use strum::IntoEnumIterator;

    use crate::Story;

    use super::*;

    #[derive(Element, Default)]
    pub struct IconStory<S: 'static + Send + Sync> {
        state_type: PhantomData<S>,
    }

    impl<S: 'static + Send + Sync> IconStory<S> {
        pub fn new() -> Self {
            Self {
                state_type: PhantomData,
            }
        }

        fn render(
            &mut self,
            _view: &mut S,
            cx: &mut ViewContext<S>,
        ) -> impl Element<ViewState = S> {
            let icons = Icon::iter();

            Story::container(cx)
                .child(Story::title_for::<_, IconElement<S>>(cx))
                .child(Story::label(cx, "All Icons"))
                .child(div().flex().gap_3().children(icons.map(IconElement::new)))
        }
    }
}
