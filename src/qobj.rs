
pub mod qobj {

use serde::{Deserialize, Serialize};
use serde_json::Result;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use std::iter;

#[derive(Serialize, Deserialize)]
pub struct QobjConfig {
    max_credits: i32,
    seed: i32,
    memory_slots: i32,
    n_qubits: i32,
    shots: i32,
}

impl QobjConfig {
    pub fn new(max_credits: i32, seed: i32, memory_slots: i32, n_qubits: i32, shots: i32) -> Self {
        QobjConfig{
            max_credits: max_credits,
            seed: seed,
            memory_slots: memory_slots,
            n_qubits: n_qubits,
            shots: shots,
        }
    }

    pub fn iter(self) -> impl Iterator<Item = String> {
        let max_credits = iter::once(self.max_credits.to_string());
        let seed = iter::once(self.seed.to_string());
        let memory_slots = iter::once(self.memory_slots.to_string());
        let n_qubits = iter::once(self.n_qubits.to_string());
        let shots = iter::once(self.shots.to_string());
        max_credits.chain(seed)
            .chain(memory_slots)
            .chain(n_qubits)
            .chain(shots)
    }
}


#[derive(Serialize, Deserialize)]
pub struct QobjExperiment {
    instructions: Vec<String>,
    // Optional properties.
    header: String,
    config: String,
}

impl QobjExperiment {
    pub fn new(instructions: Vec<String>, header: String, config: String) -> Self {
        QobjExperiment {
            instructions: instructions,
            header: header,
            config: config,
        }
    }

    pub fn iter(self) -> impl Iterator<Item = String>{
        let header = iter::once(self.header);
        let config = iter::once(self.config);
        self.instructions.into_iter().chain(header)
            .chain(config)
    }
}

#[derive(Serialize, Deserialize)]
pub struct QobjHeader {
    backend_name: String,
    backend_version: String,
}

impl QobjHeader {
    pub fn new(backend_name: String, backend_version: String) -> Self {
        QobjHeader {
            backend_name: backend_name,
            backend_version: backend_version
        }
    }

    pub fn iter(self) -> impl Iterator<Item = String> {
        let backend_name = iter::once(self.backend_name);
        let backend_version = iter::once(self.backend_version);

        backend_name.chain(backend_version)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Qobj {
    pub qobj_id: String,
    pub config: QobjConfig,
    pub experiments: Vec<QobjExperiment>,
    pub header: QobjHeader,
    pub r#type: String,
    pub schema_version: String,
}

impl Qobj {
    pub fn new(qobj_id: String, config: QobjConfig, experiments: Vec<QobjExperiment>, header: QobjHeader,
            r#type: String, schema_version: String) -> Self {
        Qobj{
            qobj_id: qobj_id,
            config: config,
            experiments: experiments,
            header: header,
            r#type: r#type,
            schema_version: schema_version,
        }
    }

    pub fn iter(self) -> impl Iterator<Item = String>{
        let qobj_id = iter::once(self.qobj_id);
        let experiments = self.experiments.into_iter().map(|e| e.iter());
        let r#type = iter::once(self.r#type);
        let schema_version = iter::once(self.schema_version);

        qobj_id.chain(self.config.iter())
            //.chain(experiments)
            .chain(self.header.iter())
            .chain(r#type)
            .chain(schema_version)
    }
}

}
