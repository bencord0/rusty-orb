/// A Reimplementation of the continuation-orb, in rust 🦀
use core::convert::Infallible;
use std::{
    env,
    fs,
    path::Path,
};
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let continuation_key = env::var("CIRCLE_CONTINUATION_KEY")
        .expect("Missing `CIRCLE_CONTINUATION_KEY");

    let configuration_path = env::var("PARAM_CONFIGURATION")
        .expect("Missing `PARAM_CONFIGURATION`");

    let parameters_path = env::var("PARAM_PARAMETERS")
        .expect("MISSING `PARAM_PARAMETERS");

    let continuation = Continuation::builder()
        .continuation_key(continuation_key)
        .configuration_path(configuration_path)
        .parameters_path(parameters_path)
        .build()?;

    let client = reqwest::blocking::Client::new();
    let _ = client.post("https://circleci.com/api/v2/pipeline/continue")
        .json(&continuation)
        .send()?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct Continuation {
    #[serde(rename = "continuation-key")]
    continuation_key: String,

    configuration: String,
    parameters: serde_json::Value,
}

impl Continuation {
  fn builder() -> ContinuationBuilder {
      ContinuationBuilder::default()
  }
}

// A basic builder pattern, consider using the `typed-builder` crate?
#[derive(Debug)]
struct ContinuationBuilder {
    continuation_key: Option<String>,
    configuration: Option<String>,
    parameters: Option<serde_json::Value>,
}

impl Default for ContinuationBuilder {
    fn default() -> Self {
        Self {
            continuation_key: None,
            configuration: None,
            parameters: None,
        }
    }
}

impl ContinuationBuilder {
    fn continuation_key<S>(mut self, continuation_key: S) -> Self
    where
        S: Into<String>,
    {
        self.continuation_key = Some(continuation_key.into());
        self
    }

    fn configuration_path<P>(mut self, configuration_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let content = fs::read(configuration_path)
            .expect("Read configuration path");
        let configuration = String::from_utf8_lossy(&content);
        self.configuration = Some(configuration.to_string());
        self
    }

    fn parameters_path<P>(mut self, parameters_path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let content = fs::read(parameters_path)
            .expect("Read parameters path");
        let parameters: serde_json::Value = serde_yaml::from_slice(&content)
            .unwrap_or_default();

        self.parameters = Some(parameters);
        self
    }

    fn build(self) -> Result<Continuation, Infallible> {
        Ok(Continuation {
            continuation_key: self.continuation_key.unwrap(),
            configuration: self.configuration.unwrap(),
            parameters: self.parameters.unwrap(),
        })
    }
}
