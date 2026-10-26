use crate::error::SimResult;
use crate::serialization::blueprint::TopologyBlueprint;

pub struct SchemaMigrator;

impl SchemaMigrator {
    pub fn upgrade_to_latest(raw_json: &str) -> SimResult<TopologyBlueprint> {
        todo!()
    }
}
