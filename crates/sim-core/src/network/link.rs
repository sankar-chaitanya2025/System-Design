use crate::error::SimResult;
use crate::event::EventScheduler;
use crate::network::bandwidth::TokenBucketShaper;
use crate::payloads::RequestPacket;
use crate::topology::{EdgeId, LinkCharacteristics, NodeId};
use rand::Rng;

pub struct VirtualLinkChannel {
    pub edge_id: EdgeId,
    pub src: NodeId,
    pub dst: NodeId,
    pub characteristics: LinkCharacteristics,
    pub shaper: TokenBucketShaper,
}

impl VirtualLinkChannel {
    pub fn transmit_packet<R: Rng>(
        &mut self,
        packet: RequestPacket,
        scheduler: &mut dyn EventScheduler,
        rng: &mut R,
    ) -> SimResult<()> {
        todo!()
    }
}
