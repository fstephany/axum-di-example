//! Use traits but still use a static dispatch.

use crate::build_templates;
use axum::{routing::get, Router};
use database::MemoryDB;
use handlebars::Handlebars;

pub trait AppState: Clone + Send + Sync + 'static {
    type D: database::DB;

    fn db(&self) -> &Self::D;
    fn templates(&self) -> &Handlebars<'static>;
}

#[derive(Clone)]
pub struct RegularAppState {
    pub templates: Handlebars<'static>,
    pub db: MemoryDB,
}

impl AppState for RegularAppState {
    type D = MemoryDB;

    fn db(&self) -> &Self::D {
        &self.db
    }

    fn templates(&self) -> &Handlebars<'static> {
        &self.templates
    }
}

pub fn build_router() -> Router {
    let state = RegularAppState {
        templates: build_templates(),
        db: MemoryDB::new(),
    };
    build(state)

    // Being Explicit works but we lose the flexibility
    // Router::new()
    //     .route("/", get(handlers::index::<RegularAppState>))
    //     .route("/item/:id", get(handlers::show::<RegularAppState>))
    //     .with_state(state)
}

fn build<A: AppState>(state: A) -> Router {
    // Router::new()
    //     .route("/", get(handlers::index))
    //     .route("/item/:id", get(handlers::show))
    //     .with_state(state)

    // Doesnt work neither
    Router::new()
        .route("/", get(handlers::index::<A>))
        .route("/item/:id", get(handlers::show::<A>))
        .with_state(state)
}

mod handlers {
    use super::{database::DB, AppState};
    use axum::{
        extract::{Path, State},
        response::Html,
    };
    use serde::Serialize;
    use uuid::Uuid;

    #[derive(Serialize)]
    struct ItemViewModel<'a> {
        pub name: &'a str,
        pub uuid: &'a Uuid,
    }

    #[derive(Serialize)]
    struct IndexViewModel<'a> {
        pub title: &'a str,
        pub items: Vec<ItemViewModel<'a>>,
    }

    pub async fn index<S: AppState>(State(state): State<S>) -> Html<String> {
        let db = state.db();
        let items = db
            .all_items()
            .await
            .into_iter()
            .map(|(uuid, name)| ItemViewModel { name, uuid })
            .collect();

        let view = IndexViewModel {
            title: "All Items",
            items,
        };

        Html(state.templates().render("index", &view).unwrap())
    }

    pub async fn show<S: AppState>(Path(id): Path<Uuid>, State(state): State<S>) -> Html<String> {
        let db = state.db();

        let item = db
            .get_item(&id)
            .await
            .map(|name| ItemViewModel { name, uuid: &id })
            .unwrap();

        Html(state.templates().render("show", &item).unwrap())
    }
}

mod database {
    use axum::async_trait;
    use std::{collections::HashMap, sync::Arc};
    use uuid::{uuid, Uuid};

    #[async_trait]
    pub trait DB {
        async fn all_items(&self) -> Vec<(&Uuid, &String)>;
        async fn get_item(&self, item_id: &Uuid) -> Option<&String>;
    }

    #[derive(Clone)]
    pub struct MemoryDB {
        items: Arc<HashMap<Uuid, String>>,
    }

    impl MemoryDB {
        pub fn new() -> Self {
            let items = [
                (
                    uuid!("fd03f48c-af4f-4485-8a56-03e5354277ce"),
                    "Apple Pie".to_owned(),
                ),
                (
                    uuid!("deba1d8c-81fd-4273-9fcd-f4c5b5666fe2"),
                    "Marshmallow".to_owned(),
                ),
                (
                    uuid!("29cf7887-d228-41ca-883c-516cf3105634"),
                    "Eclair au chocolat".to_owned(),
                ),
                (
                    uuid!("9103a2b0-af58-4db5-a9a8-cbdd7274e15a"),
                    "Merveilleux".to_owned(),
                ),
            ];

            Self {
                items: Arc::new(items.into_iter().collect()),
            }
        }
    }

    #[async_trait]
    impl DB for MemoryDB {
        async fn all_items(&self) -> Vec<(&Uuid, &String)> {
            self.items.iter().collect()
        }

        async fn get_item(&self, item_id: &Uuid) -> Option<&String> {
            self.items.get(item_id)
        }
    }
}
