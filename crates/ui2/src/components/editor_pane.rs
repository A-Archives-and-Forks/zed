use std::path::PathBuf;
use std::sync::Arc;

use gpui3::{view, Context, View};

use crate::prelude::*;
use crate::{
    hello_world_rust_editor_with_status_example, v_stack, Breadcrumb, Buffer, BufferSearch, Icon,
    IconButton, IconColor, Symbol, Tab, TabBar, Toolbar,
};

#[derive(Clone)]
pub struct EditorPane {
    tabs: Vec<Tab<Self>>,
    path: PathBuf,
    symbols: Vec<Symbol>,
    buffer: Buffer<Self>,
    buffer_search: View<BufferSearch>,
    is_buffer_search_open: bool,
}

impl EditorPane {
    pub fn new(
        cx: &mut WindowContext,
        tabs: Vec<Tab<Self>>,
        path: PathBuf,
        symbols: Vec<Symbol>,
        buffer: Buffer<Self>,
    ) -> Self {
        Self {
            tabs,
            path,
            symbols,
            buffer,
            buffer_search: BufferSearch::view(cx),
            is_buffer_search_open: false,
        }
    }

    pub fn toggle_buffer_search(&mut self, cx: &mut ViewContext<Self>) {
        self.is_buffer_search_open = !self.is_buffer_search_open;

        cx.notify();
    }

    pub fn view(cx: &mut WindowContext) -> View<Self> {
        let theme = theme(cx);

        view(
            cx.entity(|cx| hello_world_rust_editor_with_status_example(cx)),
            Self::render,
        )
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element<ViewState = Self> {
        v_stack()
            .w_full()
            .h_full()
            .flex_1()
            .child(TabBar::new(self.tabs.clone()))
            .child(
                Toolbar::new()
                    .left_item(Breadcrumb::new(self.path.clone(), self.symbols.clone()))
                    .right_items(vec![
                        IconButton::new(Icon::InlayHint),
                        IconButton::<Self>::new(Icon::MagnifyingGlass)
                            .when(self.is_buffer_search_open, |this| {
                                this.color(IconColor::Accent)
                            })
                            .on_click(Arc::new(|editor, cx| {
                                editor.toggle_buffer_search(cx);
                            })),
                        IconButton::new(Icon::MagicWand),
                    ]),
            )
            .children(Some(self.buffer_search.clone()).filter(|_| self.is_buffer_search_open))
            .child(self.buffer.clone())
    }
}
