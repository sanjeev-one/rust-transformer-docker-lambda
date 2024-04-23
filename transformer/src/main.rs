use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{Value, json};
use rust_bert::pipelines::sequence_classification::{SequenceClassificationModel, SequenceClassificationConfig};
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    lambda_runtime::run(handler_fn(my_handler)).await?;
    Ok(())
}

async fn my_handler(event: Value, _: Context) -> Result<Value, Error> {
    let config = SequenceClassificationConfig::default();
    let model = SequenceClassificationModel::new(config)?;

    // Assume event is a simple JSON containing { "text": "example" }
    let input_text = event["text"].as_str().unwrap_or("");

    let output = model.predict(&[input_text.to_string()]);

    Ok(json!({ "result": output }))
}
