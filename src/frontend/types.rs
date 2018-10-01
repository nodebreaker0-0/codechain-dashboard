use std::net::SocketAddr;

use super::super::agent;
use super::super::common_rpc_types;
use super::super::common_rpc_types::{BlockId, NodeName, NodeStatus, NodeVersion};
use super::super::db;

#[derive(Clone)]
pub struct Context {
    pub agent_service: agent::ServiceSender,
    pub db_service: db::ServiceSender,
}

pub type Event = String;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parcel {
    // FIXME: fill structure
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkPermission {
    pub list: Vec<SocketAddr>,
    pub enabled: bool,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareUsage {
    pub total: i64,
    pub available: i64,
    pub percentage_used: f64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HardwareInfo {
    pub cpu_usage: Vec<f64>,
    pub disk_usage: HardwareUsage,
    pub memory_usage: HardwareUsage,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum DashboardNode {
    #[serde(rename_all = "camelCase")]
    Normal {
        status: NodeStatus,
        address: Option<SocketAddr>,
        version: Option<NodeVersion>,
        best_block_id: Option<BlockId>,
        name: NodeName,
    },
    #[serde(rename_all = "camelCase")]
    UFO {
        status: NodeStatus,
        name: NodeName,
        address: Option<SocketAddr>,
    },
}

impl DashboardNode {
    pub fn from_db_state(state: &db::AgentState) -> Self {
        DashboardNode::Normal {
            status: state.status,
            name: state.name.clone(),
            address: state.address,
            version: state.version.clone(),
            best_block_id: state.best_block_id.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeConnection {
    pub node_a: String,
    pub node_b: String,
}

impl NodeConnection {
    pub fn from_connection(connection: &common_rpc_types::Connection) -> Self {
        let (node_a, node_b) = connection;
        Self {
            node_a: node_a.clone(),
            node_b: node_b.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardGetNetworkResponse {
    pub nodes: Vec<DashboardNode>,
    pub connections: Vec<NodeConnection>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartOption {
    pub env: String,
    pub args: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeGetInfoResponse {
    pub name: NodeName,
    pub status: NodeStatus,
    pub start_option: Option<StartOption>,
    pub address: Option<SocketAddr>,
    pub version: Option<NodeVersion>,
    pub best_block_id: Option<BlockId>,
    pub pending_parcels: Vec<Parcel>,
    pub peers: Vec<SocketAddr>,
    pub whitelist: NetworkPermission,
    pub blacklist: NetworkPermission,
    pub hardware: HardwareInfo,
    pub events: Vec<Event>,
}

impl NodeGetInfoResponse {
    fn dummy() -> Self {
        NodeGetInfoResponse {
            name: "Dummy".to_string(),
            address: Some("127.0.0.1:3485".parse().unwrap()),
            version: Some(NodeVersion {
                version: "0.1.0".to_string(),
                hash: "d6fb3195876b6b175902d25dd621db99527ccb6f".to_string(),
            }),
            status: NodeStatus::Run,
            start_option: None,
            best_block_id: Some(BlockId {
                block_number: 0,
                hash: Default::default(),
            }),
            pending_parcels: Vec::new(),
            peers: Vec::new(),
            whitelist: NetworkPermission {
                list: Vec::new(),
                enabled: false,
            },
            blacklist: NetworkPermission {
                list: Vec::new(),
                enabled: false,
            },
            hardware: HardwareInfo {
                cpu_usage: vec![0.34, 0.03, 0.58],
                disk_usage: HardwareUsage {
                    total: 8 * 1000 * 1000 * 1000,
                    available: 5 * 1000 * 1000 * 1000,
                    percentage_used: 0.6,
                },
                memory_usage: HardwareUsage {
                    total: 8 * 1000 * 1000 * 1000,
                    available: 5 * 1000 * 1000 * 1000,
                    percentage_used: 0.6,
                },
            },
            events: vec!["Network connected".to_string(), "Block received".to_string()],
        }
    }

    pub fn from_db_state(state: &db::AgentState) -> Self {
        let mut dummy = Self::dummy();
        dummy.address = state.address;
        dummy.status = state.status;
        dummy.name = state.name.clone();
        dummy.peers = state.peers.clone();
        dummy.best_block_id = state.best_block_id.clone();
        dummy.version = state.version.clone();
        dummy
    }
}
