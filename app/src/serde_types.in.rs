
extern crate uuid;

use self::uuid::Uuid;
use std::collections::BTreeMap as Map;

#[derive(Serialize, Deserialize)]
pub struct Census{
    pub ring_id: Uuid,
    pub ring_alias: String,
    pub last_update: String,
    pub prism_ip_address: String,
    pub service_groups : Vec<ServiceGroup>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceGroup{
    pub name: String,
    pub in_event: bool,
    pub service: String,
    pub group: String,
    pub organization: Option<String>,
    pub members: Map<Uuid, Member>,
}

#[derive(Serialize, Deserialize)]
pub struct Member {
    pub census_id: Uuid,
    pub member_id: Uuid,
    pub ip: String,
    pub leader: bool,
    pub hostname: String,
    pub status: String,
    pub vote: Option<String>,
    pub election: Option<String>,
    pub needs_write: Option<bool>,
    pub initialized: bool,
    pub suitability: u32,
    pub port: Option<String>,
    pub exposes: Option<Vec<String>>,
    pub incarnation: LamportClock,
    pub configuration: Configuration
}

#[derive(Serialize, Deserialize)]
pub struct LamportClock {
    pub counter: u64
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub error: Option<Error>,
    pub bind: Option<Map<String, String>>,
    pub hab: Option<Map<String, String>>,
    pub pkg: Option<String>,
    pub deps: Option<Vec<String>>,
    pub sys: Option<Sys>
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub status: u32,
    pub message: String
}

#[derive(Serialize, Deserialize)]
pub struct Sys {
    pub gossip_ip: Option<String>,
    pub gossip_port: Option<u32>,
    pub hostname: Option<String>,
    pub ip: Option<String>,
    pub sidecar_ip: Option<String>,
    pub sidecar_port: Option<u32>
}
