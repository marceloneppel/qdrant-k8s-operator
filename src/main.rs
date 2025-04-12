use anyhow::Result;
use curl::easy::Easy;
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

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
#[serde(rename_all_fields(deserialize = "kebab-case"))]
enum Action {}

fn action_handler(_model: ActionModel<Action, impl Backend>) -> Result<ActionResult> {
    let data = HashMap::new();
    Ok(Ok(data))
}

fn event_handler(model: EventModel<impl Backend>) -> Result<Status> {
    let pattern = "qdrant".to_string();
    match model.event {
        Event::PebbleReady(pattern) => {
            // Bootstrap the Pebble client.
            let mut easy = Easy::new();
            match easy.unix_socket("/charm/containers/qdrant/pebble.socket") {
                Ok(_) => {
                    model.log.info("Socket connected")?;
                }
                Err(e) => {
                    model.log.error(format!("Socket error: {}", e).as_str())?;
                    return Ok(Status::Blocked("Socket error"));
                }
            };

            // Add the layer.
            let mut data = r##"{"action": "add","combine": true,"inner": true,"label": "qdrant","format": "yaml","layer": "summary: Qdrant layer\ndescription: Layer to execute Qdrant.\nservices:\n  qdrant:\n    override: replace\n    summary: Qdrant\n    command: /qdrant/entrypoint.sh\n    startup: enabled"}"##;
            easy.url("localhost/v1/layers")?;
            easy.post_field_size(data.len() as u64)?;
            easy.post(true)?;
            execute_post_calls(&mut easy, data)?;

            // Start the service.
            data = r##"{"action": "start","services": ["qdrant"]}"##;
            easy.url("localhost/v1/services")?;
            easy.post_field_size(data.len() as u64)?;
            easy.post(true)?;
            execute_post_calls(&mut easy, data)?;
        }
        _ => {}
    }
    Ok(Status::Active(""))
}

fn execute_post_calls(easy: &mut Easy, mut data: &str) -> Result<()> {
    let mut dst = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.read_function(|buf| {
            let len = std::cmp::min(buf.len(), data.len());
            buf[..len].copy_from_slice((&data[..len]).as_ref());
            data = &data[len..];
            Ok(len)
        })?;
        transfer.write_function(|buf| {
            dst.extend_from_slice(buf);
            Ok(buf.len())
        })?;
        transfer.perform()?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let charm = Framework::new(JujuBackend {}, event_handler, action_handler);
    charm.execute()
}
