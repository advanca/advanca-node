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

use advanca_crypto_types::*;

use frame_support::traits::BalanceStatus;
use frame_support::traits::{Currency, ReservableCurrency};

use frame_support::{
    codec::Encode, decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult,
    ensure,
};
// use smart_default::SmartDefault;
// use sp_runtime::RuntimeDebug;
use sp_api::HashT;

use frame_system::{self as system, ensure_signed};
use sp_std::prelude::*;

use frame_support::debug;

pub use advanca_node_primitives::*;

const PER_BLOCK_COST: u32 = 1_000_000;
const PER_DAY_BLOCKS: u32 = 14_400;

pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
pub type TaskId<T> = <T as system::Trait>::Hash;
pub type Index<T> = <T as system::Trait>::Index;


/// The module's configuration trait.
pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// The currency to be handled in this module
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
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
        Users get(fn get_user): map hasher(opaque_blake2_256) T::AccountId => User<T::AccountId>;
        /// Registered workers
        Workers get(fn get_worker): map hasher(opaque_blake2_256) T::AccountId => Worker<T::AccountId>;

        /// Saved tasks
        ///
        /// Note only the tasks in unscheduled or scheduled state are saved in this map.
        /// Any completed or aborted tasks are removed from chain to save space
        Tasks get(fn get_task): map hasher(opaque_blake2_256) TaskId<T> => Task<TaskId<T>, T::AccountId, Duration, TaskSpec<Privacy>, TaskStatus, Ciphertext>;

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

        #[weight = 0] //FIXME: use meaningful weight
        pub fn register_worker(origin, deposit: BalanceOf<T>, enclave: Enclave<T::AccountId>) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(!<Workers<T>>::contains_key(&worker), "The worker is already registered");

            T::Currency::reserve(&worker, deposit)?;
            <Workers<T>>::insert(&worker, Worker{account_id: worker.clone(), enclave});

            Self::deposit_event(RawEvent::WorkerAdded(worker.clone()));
            Ok(())
        }

        #[weight = 0] //FIXME: use meaningful weight
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

        #[weight = 0] //FIXME: use meaningful weight
        pub fn register_user(origin, deposit: BalanceOf<T>, public_keys: PublicKeys) -> DispatchResult {
            let user = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(!<Users<T>>::contains_key(&user), "The user is already registered");

            T::Currency::reserve(&user, deposit)?;
            <Users<T>>::insert(&user, User{account_id: user.clone(), public_keys});

            Self::deposit_event(RawEvent::UserAdded(user.clone()));
            Ok(())
        }

        #[weight = 0] //FIXME: use meaningful weight
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

        #[weight = 0] //FIXME: use meaningful weight
        pub fn submit_task(origin, signed_owner_task_secp256r1_pubkey: Vec<u8>, signed_owner_task_sr25519_pubkey: Vec<u8>, lease: Duration, task_spec: TaskSpec<Privacy>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            // we'll reserve the amount for task creation here
            let reserved_amount = if lease == 0 {
                // keep at least 10 days worth of deposit
                BalanceOf::<T>::from(PER_BLOCK_COST) * BalanceOf::<T>::from(PER_DAY_BLOCKS) * BalanceOf::<T>::from(10)
            } else {
                BalanceOf::<T>::from(PER_BLOCK_COST) * BalanceOf::<T>::from(lease as u32)
            };

            T::Currency::reserve(&owner, reserved_amount)?;

            let task_id = Self::task_id(&owner, <system::Module<T>>::account_nonce(&owner));
            Tasks::<T>::insert(task_id.clone(), Task{
                signed_owner_task_secp256r1_pubkey, signed_owner_task_sr25519_pubkey,
                owner, task_id, lease, task_spec, ..Default::default()
            });

            <UnscheduledTasks<T>>::mutate(|v| v.push(task_id.clone()));

            Self::deposit_event(RawEvent::TaskSubmitted(task_id));
            Ok(())
        }

        #[weight = 0] //FIXME: use meaningful weight
        pub fn submit_task_evidence(origin, task_id: TaskId<T>, evidences: Vec<Vec<u8>>) -> DispatchResult {
            debug::RuntimeLogger::init();
            let worker = ensure_signed(origin)?;

            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");
            let task = Tasks::<T>::get(task_id.clone());
            ensure!(task.worker == Some(worker), "only worker can update this task");
            for evidence in evidences {
                let timestamp_signed_msg: Secp256r1SignedMsg = serde_json::from_slice(&evidence).unwrap();
                debug::info!("timestamp: {:?}", timestamp_signed_msg);
                // sp_runtime::print(format!("{:?}", timestamp_signed_msg).as_str());
                Tasks::<T>::mutate(task_id.clone(), |t| t.worker_heartbeat_evidence.push(evidence));
            }
            Ok(())
        }

        /// Updates a task
        ///
        /// Currently only updating TaskSpec is allowed.
        #[weight = 0] //FIXME: use meaningful weight
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
        #[weight = 0] //FIXME: use meaningful weight
        pub fn accept_task(origin, task_id: TaskId<T>, signed_eph_pubkey: Vec<u8>, signed_sr25519_pubkey: Vec<u8>, url: Ciphertext) -> DispatchResult {
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
                t.signed_enclave_task_secp256r1_pubkey = Some(signed_eph_pubkey);
                t.signed_enclave_task_sr25519_pubkey = Some(signed_sr25519_pubkey);
                t.worker_url = Some(url);
            });

            // or use v.remove_item if it becomes stable feature in rust
            UnscheduledTasks::<T>::mutate(|v| v.retain(|&h| h != task_id));

            Self::deposit_event(RawEvent::TaskAccepted(task_id));
            Ok(())
        }

        #[weight = 0] //FIXME: use meaningful weight
        pub fn abort_task(origin, task_id: TaskId<T>) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");
            let task = Tasks::<T>::get(task_id.clone());
            //TODO: use pre-defined error
            ensure!(task.owner == owner, "only owner can abort the task");

            //Tasks::<T>::remove(task_id);
            Tasks::<T>::mutate(task_id.clone(), |t| {
                t.status = TaskStatus::Done;
            });

            Self::deposit_event(RawEvent::TaskAborted(task_id));
            Ok(())
        }

        #[weight = 0] //FIXME: use meaningful weight
        pub fn complete_task(origin, task_id: TaskId<T>) -> DispatchResult {
            let worker = ensure_signed(origin)?;

            //TODO: use pre-defined error
            ensure!(Tasks::<T>::contains_key(task_id.clone()), "task_id must exist");
            let task = Tasks::<T>::get(task_id.clone());
            //TODO: use pre-defined error
            ensure!(task.worker == Some(worker.clone()), "only the worker can complete this task");

            // get the task information
            let task = Tasks::<T>::get(task_id.clone());

            // here, we are recalculating the amount that is reserved for this task, maybe we
            // should consider storing it somewhere
            let lease = task.lease;
            let reserved_amount = if lease == 0 {
                // keep at least 10 days worth of deposit
                BalanceOf::<T>::from(PER_BLOCK_COST) * BalanceOf::<T>::from(PER_DAY_BLOCKS) * BalanceOf::<T>::from(10)
            } else {
                BalanceOf::<T>::from(PER_BLOCK_COST) * BalanceOf::<T>::from(lease as u32)
            };

            let blocks_alive: BalanceOf<T> = (task.worker_heartbeat_evidence.len() as u32).into();

            // we multiply by 1000 to make the value larger for demo purpose
            let task_fees: BalanceOf<T> = blocks_alive * PER_BLOCK_COST.into() * 1000.into();

            T::Currency::repatriate_reserved(&task.owner, &worker, task_fees, BalanceStatus::Free)?;

            //Tasks::<T>::remove(task_id);
            Tasks::<T>::mutate(task_id.clone(), |t| {
                t.status = TaskStatus::Done;
            });

            // unreserve the remaining amount for this job
            T::Currency::unreserve(&task.owner, reserved_amount - task_fees);

            Self::deposit_event(RawEvent::TaskCompleted(task_id));
            Ok(())
        }

        //TODO: TBD
        #[weight = 0] //FIXME: use meaningful weight
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
