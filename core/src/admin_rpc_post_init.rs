use {
    put_gossip::cluster_info::ClusterInfo,
    put_runtime::bank_forks::BankForks,
    put_sdk::pubkey::Pubkey,
    std::{
        collections::HashSet,
        sync::{Arc, RwLock},
    },
};

#[derive(Clone)]
pub struct AdminRpcRequestMetadataPostInit {
    pub cluster_info: Arc<ClusterInfo>,
    pub bank_forks: Arc<RwLock<BankForks>>,
    pub vote_account: Pubkey,
    pub repair_whitelist: Arc<RwLock<HashSet<Pubkey>>>,
}
