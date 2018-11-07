extern crate aurelius;
extern crate lazy_static;
extern crate xi_core_lib as xi_core;
extern crate xi_plugin_lib as xi_plugin;
extern crate xi_rope;

use std::collections::HashSet;
use std::path::Path;

use aurelius::browser;
use aurelius::{Listening, Server};
use lazy_static::lazy_static;
use xi_core::{ConfigTable, syntax::LanguageId, tabs::ViewId};
use xi_plugin::{mainloop, Cache, ChunkCache, Plugin, View};
use xi_rope::rope::RopeDelta;

lazy_static! {
    static ref MARKDOWN_LANGUAGE_ID: LanguageId = LanguageId::from("Markdown");
}

#[derive(Debug, Default)]
struct AureliusPlugin {
    markdown_views: HashSet<ViewId>,
    server: Option<Listening>,
}

impl AureliusPlugin {
    fn new() -> Self {
        Self::default()
    }

    fn add_markdown_view<C: Cache>(&mut self, view: &mut View<C>) {
        self.markdown_views.insert(view.get_id());

        if self.server.is_none() {
            let server = Server::new();
            let listening = server.start().expect("could not start server");
            browser::open(&format!(
                "http://localhost:{}",
                listening.http_addr().unwrap().port()
            ))
            .unwrap();
            self.server = Some(listening);
        }

        if let Some(server) = &self.server {
            let document = view.get_document().unwrap();
            server.send(&document).unwrap();
        }
    }

    fn remove_markdown_view<C: Cache>(&mut self, view: &View<C>) {
        self.markdown_views.remove(&view.get_id());

        if self.markdown_views.is_empty() {
            self.server = None;
        }
    }
}

impl Plugin for AureliusPlugin {
    type Cache = ChunkCache;

    fn new_view(&mut self, view: &mut View<Self::Cache>) {
        if is_markdown(&view) {
            self.add_markdown_view(view);
        }
    }

    fn language_changed(&mut self, view: &mut View<Self::Cache>, old_lang: LanguageId) {
        if is_markdown(&view) {
            self.add_markdown_view(view);
        } else if old_lang == *MARKDOWN_LANGUAGE_ID {
            self.remove_markdown_view(&view);
        }
    }

    fn did_close(&mut self, view: &View<Self::Cache>) {
        if view.get_language_id() == &LanguageId::from("Markdown") {
            self.remove_markdown_view(&view);
        }
    }

    fn update(
        &mut self,
        view: &mut View<Self::Cache>,
        _delta: Option<&RopeDelta>,
        _edit_type: String,
        _author: String,
    ) {
        if let Some(server) = &self.server {
            // TODO: It'd be nice to do an incremental update here, but aurelius doesn't support
            // it (yet).
            let document = view.get_document().unwrap();
            server.send(&document).unwrap();
        }
    }

    fn config_changed(&mut self, _view: &mut View<Self::Cache>, _changes: &ConfigTable) {}

    fn did_save(&mut self, _view: &mut View<Self::Cache>, _old: Option<&Path>) {}
}

fn is_markdown<C: Cache>(view: &View<C>) -> bool {
    view.get_language_id() == &*MARKDOWN_LANGUAGE_ID
}

fn main() {
    let mut plugin = AureliusPlugin::new();
    mainloop(&mut plugin).unwrap();
}
