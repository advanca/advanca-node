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

#![cfg_attr(not(feature = "std"), no_std)]
/// The runtime module for Advanca core functions
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::traits::{Currency, ReservableCurrency};
use frame_support::{
    codec::{Decode, Encode},
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    ensure,
};
use smart_default::SmartDefault;
use sp_api::HashT;
use sp_runtime::RuntimeDebug;

use sp_std::prelude::*;
use system::ensure_signed;

pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
pub type TaskId<T> = <T as system::Trait>::Hash;
pub type Index<T> = <T as system::Trait>::Index;

//TODO: check if there's better type alias defined in substrate
/// Duration of the task. 0 means unlimited.
pub type Duration = u64;

/// The encrypted bytes
pub type Ciphertext = Vec<u8>;

/// The module's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The currency to be handled in this module
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq)]
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

#[derive(Encode, Decode, Default, RuntimeDebug, PartialEq, Eq)]
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
    /// The owner-determined task duration. Default to 0 for unlimited time.
    pub lease: Duration,
    /// The detailed specification of a task
    pub task_spec: TaskSpec,
    /// The worker who accepted the task. It may be none at the beginning.
    pub worker: Option<AccountId>,
    /// The worker's ephemeral key for the particular task
    pub worker_ephemeral_pubkey: Option<Vec<u8>>,
    /// The signature for the worker's ephemeral pubkey using the worker's registered private key
    pub worker_ephemeral_signature: Option<Vec<u8>>,
    /// The worker's service url saved in ciphertext encrypted by owner's public key
    pub worker_url: Option<Ciphertext>,
}

// Error from the module
decl_error! {
    pub enum Error for Module<T: Trait> {
        //TODO: add more explicit error variants
        AlreadyRegistered,
        NotFound,
    }
}

// This module's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as AdvancaCore {
        /// Registered users
        Users get(fn get_user): map hasher(blake2_256) T::AccountId => User<T::AccountId>;
        /// Registered workers
        Workers get(fn get_worker): map hasher(blake2_256) T::AccountId => Worker<T::AccountId>;

        /// Saved tasks
        ///
        /// Note only the tasks in unscheduled or scheduled state are saved in this map.
        /// Any completed or aborted tasks are removed from chain to save space
        Tasks get(fn get_task): map hasher(blake2_256) TaskId<T> => Task<TaskId<T>, T::AccountId, Duration, TaskSpec<Privacy>, TaskStatus, Ciphertext>;

        /// Unscheduled tasks
        ///
        /// A convenient place to find unscheduled tasks. Only IDs are kept.
        /// If a task becomes scheduled, it will be removed from this vector.
        UnscheduledTasks get(fn unscheduled_tasks): Vec<TaskId<T>>;
    }
}

// The module's dispatchable functions.
decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        pub fn register_worker(origin, deposit: BalanceOf<T>, enclave: Enclave<T::AccountId>) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(!<Workers<T>>::contains_key(&worker), "The worker is already registered");

            T::Currency::reserve(&worker, deposit)?;
            <Workers<T>>::insert(&worker, Worker{account_id: worker.clone(), enclave});

            Self::deposit_event(RawEvent::WorkerAdded(worker.clone()));
            Ok(())
        }

        pub fn deregister_worker(origin) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(<Workers<T>>::contains_key(&worker), "The worker is not existed");

            <Workers<T>>::remove(&worker);
            let reserved = T::Currency::reserved_balance(&worker);
            T::Currency::unreserve(&worker, reserved);

            Self::deposit_event(RawEvent::WorkerRemoved(worker.clone()));
            Ok(())
        }

        pub fn register_user(origin, deposit: BalanceOf<T>, public_key: Vec<u8>) -> DispatchResult {
            let user = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(!<Users<T>>::contains_key(&user), "The user is already registered");

            T::Currency::reserve(&user, deposit)?;
            <Users<T>>::insert(&user, User{account_id: user.clone(), public_key});

            Self::deposit_event(RawEvent::UserAdded(user.clone()));
            Ok(())
        }

        pub fn deregister_user(origin) -> DispatchResult {
            let user = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(<Users<T>>::contains_key(&user), "The user is not existed");

            <Users<T>>::remove(&user);
            let reserved = T::Currency::reserved_balance(&user);
            T::Currency::unreserve(&user, reserved);

            Self::deposit_event(RawEvent::UserRemoved(user.clone()));
            Ok(())
        }

        pub fn submit_task(origin, lease: Duration, task_spec: TaskSpec<Privacy>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let task_id = Self::task_id(&owner, <system::Module<T>>::account_nonce(&owner));
            Tasks::<T>::insert(task_id.clone(), Task{
                owner, task_id, lease, task_spec, ..Default::default()
            });

            <UnscheduledTasks<T>>::mutate(|v| v.push(task_id.clone()));

            Self::deposit_event(RawEvent::TaskSubmitted(task_id));
            Ok(())
        }

        /// Updates a task
        ///
        /// Currently only updating TaskSpec is allowed.
        pub fn update_task(origin, task_id: TaskId<T>, task_spec: TaskSpec<Privacy>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");

            let task = Tasks::<T>::get(task_id.clone());
            //TODO: use pre-defined error
            ensure!(task.owner == owner, "only owner can update this task");

            Tasks::<T>::mutate(task_id.clone(), |t| t.task_spec = task_spec);

            Self::deposit_event(RawEvent::TaskUpdated(task_id));
            Ok(())
        }

        /// Worker accepts a task
        ///
        /// `task_id`: Selects whichs task to accept
        /// `url`: The worker service url in ciphertext (only viewable by task owner)
        pub fn accept_task(origin, task_id: TaskId<T>, eph_pubkey: Vec<u8>, eph_signature: Vec<u8>, url: Ciphertext) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");
            let task = Tasks::<T>::get(task_id.clone());

            //TODO: use pre-defined error
            ensure!(task.status == TaskStatus::Unscheduled, "task must not be scheduled");

            ensure!(Workers::<T>::contains_key(worker.clone()), Error::<T>::NotFound);

            Tasks::<T>::mutate(task_id.clone(), |t| {
                t.status = TaskStatus::Scheduled;
                t.worker = Some(worker);
                t.worker_ephemeral_pubkey = Some(eph_pubkey);
                t.worker_ephemeral_signature = Some(eph_signature);
                t.worker_url = Some(url);
            });

            // or use v.remove_item if it becomes stable feature in rust
            UnscheduledTasks::<T>::mutate(|v| v.retain(|&h| h != task_id));

            Self::deposit_event(RawEvent::TaskAccepted(task_id));
            Ok(())
        }

        pub fn abort_task(origin, task_id: TaskId<T>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");
            let task = Tasks::<T>::get(task_id.clone());
            //TODO: use pre-defined error
            ensure!(task.owner == owner, "only owner can abort the task");

            Tasks::<T>::remove(task_id.clone());

            Self::deposit_event(RawEvent::TaskAborted(task_id));
            Ok(())
        }

        pub fn complete_task(origin, task_id: TaskId<T>) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");
            let task = Tasks::<T>::get(task_id.clone());
            //TODO: use pre-defined error
            ensure!(task.worker == Some(worker), "only the worker can complete this task");

            Tasks::<T>::remove(task_id);

            Self::deposit_event(RawEvent::TaskCompleted(task_id));
            Ok(())
        }

        //TODO: TBD
        pub fn progress_task(origin) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }
    }
}

// Helper functions for the module
impl<T: Trait> Module<T> {
    /// Calculate task id as hash of the address + index
    fn task_id(account_id: &T::AccountId, account_nonce: Index<T>) -> TaskId<T> {
        let mut x = account_id.encode();
        account_nonce.using_encoded(|a| x.append(&mut a.to_vec()));
        T::Hashing::hash(&x)
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
        TaskId = TaskId<T>,
    {
        UserAdded(AccountId),
        UserRemoved(AccountId),
        TaskSubmitted(TaskId),
        TaskUpdated(TaskId),
        TaskAccepted(TaskId),
        TaskCompleted(TaskId),
        TaskAborted(TaskId),
        WorkerAdded(AccountId),
        WorkerRemoved(AccountId),
    }
);
