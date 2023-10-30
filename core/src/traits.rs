use crate::{
    types::{AggregatedBatch, StateUpdate, TransactionReceipt},
};
use serde::{de::DeserializeOwned, Serialize};
use parity_scale_codec::{Encode, Decode};
use sparse_merkle_tree::MerkleProof;
use sparse_merkle_tree::H256;
use anyhow::Error;

pub trait Leaf<K> {
    fn get_key(&self) -> K;
}

pub trait StateMachine<V, T: Clone + DeserializeOwned + Serialize + Encode + Decode> {
    fn new(root: H256) -> Self;
    fn execute_tx(
        &mut self,
        call: T,
        aggregated_proof: AggregatedBatch,
    ) -> Result<(StateUpdate<V>, TransactionReceipt), Error>;
    fn get_state_with_proof(
        &self, 
        key: &H256, 
    ) -> Result<(V, MerkleProof), Error>;
    fn revert(&mut self, root: H256) -> Result<(), Error>;
    fn get_root(&self) -> Result<H256, Error>;
}

pub trait StateTransition<V, T> {
    //Requiring the Value to be in a vector adds overhead when only one state is modified,
    //but we do it for sake of simplicity.
    fn execute_tx(
        &self,
        pre_state: Vec<V>,
        call_params: T,
        aggregated_proof: AggregatedBatch,
    ) -> Result<(Vec<V>, TransactionReceipt), Error>;
}

pub trait TxHasher {
    fn to_h256(&self) -> H256;
}
