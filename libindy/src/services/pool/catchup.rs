use std::collections::HashMap;
use std::collections::HashSet;
use errors::pool::PoolError;
use errors::common::CommonError;
use services::ledger::merkletree::merkletree::MerkleTree;
use services::pool::merkle_tree_factory;
use services::pool::rust_base58::{FromBase58, ToBase58};

pub enum CatchupProgress {
    ShouldBeStarted(
        Vec<u8>, //target_mt_root
        usize, //target_mt_size
        MerkleTree,
    ),
    NotNeeded(MerkleTree),
    InProgress,
}

pub fn check_nodes_responses_on_status(nodes_votes: &HashMap<(String, usize, Option<Vec<String>>), HashSet<String>>,
                                       merkle_tree: &MerkleTree,
                                       node_count: usize,
                                       f: usize,
                                       pool_name: &str) -> Result<CatchupProgress, PoolError> {
    if let Some((most_popular_vote, votes_cnt)) = nodes_votes.iter().map(|(key, val)| (key, val.len())).max_by_key(|entry| entry.1) {
        if votes_cnt == node_count - f {
            return _try_to_catch_up(most_popular_vote, merkle_tree).or_else(|err| {
                if merkle_tree_factory::drop_cache(pool_name).is_ok() {
                    let merkle_tree = merkle_tree_factory::create(pool_name)?;
                    _try_to_catch_up(most_popular_vote, &merkle_tree)
                } else {
                    Err(err)
                }
            })
        }
    }
    Ok(CatchupProgress::InProgress)
}

fn _try_to_catch_up(ledger_status: &(String, usize, Option<Vec<String>>), merkle_tree: &MerkleTree) -> Result<CatchupProgress, PoolError> {
    let &(ref target_mt_root, target_mt_size, ref hashes) = ledger_status;
    let cur_mt_size = merkle_tree.count();
    let cur_mt_hash = merkle_tree.root_hash().to_base58();
    if target_mt_size == cur_mt_size {
        if cur_mt_hash.eq(target_mt_root) {
            return Ok(CatchupProgress::NotNeeded(merkle_tree.clone()));
        } else {
            return Err(PoolError::CommonError(CommonError::InvalidState(
                "Ledger merkle tree doesn't acceptable for current tree.".to_string())));
        }
    } else if target_mt_size > cur_mt_size {
        let target_mt_root = target_mt_root.from_base58().map_err(|_|
            CommonError::InvalidStructure(
                "Can't parse target MerkleTree hash from nodes responses".to_string()))?;
        match hashes {
            &None => (),
            &Some(ref hashes) => {
                match check_cons_proofs(merkle_tree, hashes, &target_mt_root, target_mt_size) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(PoolError::from(err));
                    }
                }
            }
        };
        return Ok(CatchupProgress::ShouldBeStarted(target_mt_root, target_mt_size, merkle_tree.clone()));
    } else {
        return Err(PoolError::CommonError(CommonError::InvalidState(
            "Local merkle tree greater than mt from ledger".to_string())));
    }
}

pub fn check_cons_proofs(mt: &MerkleTree, cons_proofs: &Vec<String>, target_mt_root: &Vec<u8>, target_mt_size: usize) -> Result<(), CommonError> {
    let mut bytes_proofs: Vec<Vec<u8>> = Vec::new();
    for cons_proof in cons_proofs {
        let cons_proof: &String = cons_proof;
        bytes_proofs.push(cons_proof.from_base58().map_err(|err|
            CommonError::InvalidStructure(
                format!("Can't decode node consistency proof: {}", err)))?)
    }
    if !mt.consistency_proof(target_mt_root, target_mt_size, &bytes_proofs)? {
        return Err(CommonError::InvalidState("Consistency proof verification failed".to_string()));
    }
    Ok(())
}