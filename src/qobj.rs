use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
pub struct QobjConfig {
    max_credits: i32,
    seed: i32,
    memory_slots: i32,
    n_qubits: i32,
    shots: i32,
}

#[derive(Serialize, Deserialize)]
pub struct QobjExperiment {
    instructions: Vec<String>,

    // Optional properties.
    header: String,
    config: String,
}

#[derive(Serialize, Deserialize)]
pub struct QobjHeader {
    backend_name: String,
    backend_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Qobj {
    qobj_id: String,
    config: QobjConfig,
    experiments: Vec<QobjExperiment>,
    header: QobjHeader,
    r#type: String,
    schema_version: String,
}
