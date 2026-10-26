pub mod api_gateway;
pub mod app_server;
pub mod cache;
pub mod cdn;
pub mod client_pool;
pub mod consensus_coordinator;
pub mod database;
pub mod dns;
pub mod load_balancer;
pub mod message_queue;
pub mod object_storage;
pub mod search_index;
pub mod stream_processor;
pub mod traits;

use crate::error::SimResult;
use crate::event::EventContext;
use crate::nodes::traits::{NodeHealthStatus, NodeTelemetrySnapshot, SimulatedNode};
use crate::payloads::{FluidDensityWave, RequestPacket, ResponsePacket};
use crate::topology::NodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeKind {
    ClientPool(client_pool::ClientPoolNode),
    DnsResolver(dns::DnsNode),
    CdnEdge(cdn::CdnEdgeNode),
    LoadBalancer(load_balancer::LoadBalancerNode),
    ApiGateway(api_gateway::ApiGatewayNode),
    AppServer(app_server::AppServerNode),
    CacheCluster(cache::CacheNode),
    MessageQueue(message_queue::MessageQueueNode),
    StreamProcessor(stream_processor::StreamProcessorNode),
    Database(database::DatabaseNode),
    SearchIndex(search_index::SearchIndexNode),
    ObjectStorage(object_storage::ObjectStorageNode),
    ConsensusCoordinator(consensus_coordinator::ConsensusCoordinatorNode),
}

impl SimulatedNode for NodeKind {
    fn node_id(&self) -> NodeId {
        todo!()
    }

    fn health_status(&self) -> NodeHealthStatus {
        todo!()
    }

    fn on_request_arrival(
        &mut self,
        packet: RequestPacket,
        ctx: &mut EventContext<'_>,
    ) -> SimResult<()> {
        todo!()
    }

    fn on_response_arrival(
        &mut self,
        response: ResponsePacket,
        ctx: &mut EventContext<'_>,
    ) -> SimResult<()> {
        todo!()
    }

    fn on_fluid_wave(
        &mut self,
        wave: FluidDensityWave,
        ctx: &mut EventContext<'_>,
    ) -> SimResult<()> {
        todo!()
    }

    fn collect_telemetry(&self) -> NodeTelemetrySnapshot {
        todo!()
    }
}
