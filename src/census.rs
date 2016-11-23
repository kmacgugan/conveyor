
extern crate serde_derive;
extern crate serde_json;
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
    pub organization: String,
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
    pub counter: u32
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub error: Option<Error>
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub status: u32,
    pub message: String
}
