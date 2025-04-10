use anyhow::Result;
use rusty_charm_framework::model::ActionModel;
use rusty_charm_framework::types::ActionResult;
use rusty_charm_framework::{
    Framework,
    backend::{Backend, JujuBackend},
    model::EventModel,
    types::{Event, Status},
};
use serde::Deserialize;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(rename_all_fields(deserialize = "kebab-case"))]
enum Action {}

fn action_handler(_model: ActionModel<Action, impl Backend>) -> Result<ActionResult> {
    let data = HashMap::new();
    Ok(Ok(data))
}

fn event_handler(model: EventModel<impl Backend>) -> Result<Status> {
    match model.event {
        Event::Install => {
            model.status.maintenance("installing Qdrant")?;
            sleep(Duration::from_secs(10));
        }
        _ => {}
    }
    Ok(Status::Active(""))
}

fn main() -> Result<()> {
    let charm = Framework::new(JujuBackend {}, event_handler, action_handler);
    charm.execute()
}
