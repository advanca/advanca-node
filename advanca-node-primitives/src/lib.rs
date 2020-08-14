// Copyright (C) 2020 ADVANCA PTE. LTD.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use smart_default::SmartDefault;
use sp_runtime::{
    generic, RuntimeDebug, MultiSignature,
    traits::{IdentifyAccount, Verify},
};


/// Duration of the task. 0 means unlimited.
pub type Duration = u64;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// The encrypted bytes
pub type Ciphertext = Vec<u8>;

#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq, Clone)]
/// The public information about a user
pub struct User<AccountId> {
    /// User account on chain
    pub account_id: AccountId,
    /// User public key for encryption
    pub public_key: Vec<u8>,
}

#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq, Clone)]
/// The public information about an Enclave
pub struct Enclave<AccountId> {
    /// Enclave account on chain
    pub account_id: AccountId,
    /// Enclave public key for encryption
    pub public_key: Vec<u8>,
    /// Enclave attestation information which certifies all the other fields
    pub attestation: Vec<u8>,
}

#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq, Clone)]
/// The public information about a worker and its enclave
pub struct Worker<AccountId> {
    /// Worker account on chain
    pub account_id: AccountId,
    /// Enclave that the worker is running
    pub enclave: Enclave<AccountId>,
}

#[derive(Encode, Decode, SmartDefault, RuntimeDebug, PartialEq, Eq, Clone)]
/// The status of a task managed on chain
pub enum TaskStatus {
    #[default]
    /// The task is already submitted but not taken by any workers
    Unscheduled,
    /// The task is taken by a worker
    Scheduled,
    /// The task is completed or aborted
    Done,
}

#[derive(Encode, Decode, SmartDefault, RuntimeDebug, PartialEq, Eq, Clone)]
/// The possible privacy levels
pub enum Privacy {
    #[default]
    /// Public
    None,
    /// Only encryption used
    Encryption,
    /// Encryption and Square-Root ORAM
    SqrtOram,
    /// Encryption and Path ORAM
    PathOram,
    /// Encryption and Oblivous P2P
    OblivP2p,
}

#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq, Clone)]
/// The task specification. //TODO: more details
pub struct TaskSpec<Privacy> {
    /// The specification of the pod, which is the same concept in Kubernetes.
    pub pod_spec: Vec<u8>, //TODO: add more details
    /// The specification of the storage volume
    pub volume_spec: Vec<u8>, //TODO: add more details
    /// The privacy level
    pub privacy: Privacy,
}

#[derive(Encode, Decode, Default, RuntimeDebug, Clone, PartialEq, Eq)]
/// Task information
pub struct Task<TaskId, AccountId, Duration, TaskSpec, TaskStatus, Ciphertext> {
    /// A unversial task ID that is unique for every submission by every user.
    pub task_id: TaskId,
    /// The current task status
    pub status: TaskStatus,
    /// The user who submitted the task
    pub owner: AccountId,
    /// The user's signed task key
    pub signed_owner_task_pubkey: Vec<u8>,
    /// The owner-determined task duration. Default to 0 for unlimited time.
    pub lease: Duration,
    /// The detailed specification of a task
    pub task_spec: TaskSpec,
    /// The worker who accepted the task. It may be none at the beginning.
    pub worker: Option<AccountId>,
    /// The worker's ephemeral key for the particular task signed using its registered key
    pub signed_worker_task_pubkey: Option<Vec<u8>>,
    /// The worker's service url saved in ciphertext encrypted by owner's public key
    pub worker_url: Option<Ciphertext>,
    /// Worker's heartbeat evidence
    pub worker_heartbeat_evidence: Vec<Vec<u8>>,
}
