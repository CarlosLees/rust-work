use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct ApplicationConfig {
    pub clap_select: Vec<String>
}