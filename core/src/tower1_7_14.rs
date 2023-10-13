use {
    crate::consensus::{SwitchForkDecision, TowerError},
    put_sdk::{
        clock::Slot,
        hash::Hash,
        pubkey::Pubkey,
        signature::{Signature, Signer},
    },
    put_vote_program::vote_state::{/*vote_state_1_14_11::VoteState1_14_11,*/ BlockTimestamp, Vote,VoteState},
};

#[frozen_abi(digest = "F5kVFF5ds84DJV7EMmJndGN3Mo84UQcKHbCF4yyzy8UM")]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, AbiExample)]
pub struct Tower1_7_14 {
    pub(crate) node_pubkey: Pubkey,
    pub(crate) threshold_depth: usize,
    pub(crate) threshold_size: f64,
    pub(crate) vote_state: VoteState,
    pub(crate) last_vote: Vote,
    #[serde(skip)]
    // The blockhash used in the last vote transaction, may or may not equal the
    // blockhash of the voted block itself, depending if the vote slot was refreshed.
    // For instance, a vote for slot 5, may be refreshed/resubmitted for inclusion in
    //  block 10, in  which case `last_vote_tx_blockhash` equals the blockhash of 10, not 5.
    pub(crate) last_vote_tx_blockhash: Option<Hash>,
    pub(crate) last_timestamp: BlockTimestamp,
    #[serde(skip)]
    // Restored last voted slot which cannot be found in SlotHistory at replayed root
    // (This is a special field for slashing-free validator restart with edge cases).
    // This could be emptied after some time; but left intact indefinitely for easier
    // implementation
    // Further, stray slot can be stale or not. `Stale` here means whether given
    // bank_forks (=~ ledger) lacks the slot or not.
    pub(crate) stray_restored_slot: Option<Slot>,
    #[serde(skip)]
    pub(crate) last_switch_threshold_check: Option<(Slot, SwitchForkDecision)>,
}

#[frozen_abi(digest = "G4E9DtHMitZmwCujzwAgnQm4QUQQBjtDQn88kW8V1L8A")]
#[derive(Default, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, AbiExample)]
pub struct SavedTower1_7_14 {
    pub(crate) signature: Signature,
    pub(crate) data: Vec<u8>,
    #[serde(skip)]
    pub(crate) node_pubkey: Pubkey,
}

impl SavedTower1_7_14 {
    pub fn new<T: Signer>(tower: &Tower1_7_14, keypair: &T) -> Result<Self, TowerError> {
        let node_pubkey = keypair.pubkey();
        if tower.node_pubkey != node_pubkey {
            return Err(TowerError::WrongTower(format!(
                "node_pubkey is {:?} but found tower for {:?}",
                node_pubkey, tower.node_pubkey
            )));
        }

        let data = bincode::serialize(tower)?;
        let signature = keypair.sign_message(&data);
        Ok(Self {
            signature,
            data,
            node_pubkey,
        })
    }
}