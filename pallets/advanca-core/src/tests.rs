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

// Tests to be written here

use crate::{mock::*, *};
use frame_support::{assert_noop, assert_ok, StorageMap};
use sp_runtime::DispatchError;
use sp_std::prelude::*;

#[test]
fn task_id_generation() {
    let account = 0x0;

    assert_eq!(
        AdvancaCore::task_id(&account, 1),
        AdvancaCore::task_id(&account, 1)
    );
    assert_ne!(
        AdvancaCore::task_id(&account, 1),
        AdvancaCore::task_id(&account, 2)
    );
}

#[test]
fn registration() {
    new_test_ext().execute_with(|| {
        let deposit = 0;
        let user_account = 0x0;
        let user_public_key: Vec<u8> = "user_public_key".into();
        let worker_account = 0x1;
        let enclave: Enclave<u64> = Default::default();

        assert_noop!(
            AdvancaCore::register_user(Origin::NONE, deposit, user_public_key.clone()),
            DispatchError::BadOrigin
        );
        assert_noop!(
            AdvancaCore::register_worker(Origin::NONE, deposit, enclave.clone()),
            DispatchError::BadOrigin
        );

        assert_noop!(
            AdvancaCore::register_user(
                Origin::signed(user_account),
                deposit + 1,
                user_public_key.clone()
            ),
            BalancesError::InsufficientBalance
        );
        assert_noop!(
            AdvancaCore::register_worker(
                Origin::signed(worker_account),
                deposit + 1,
                enclave.clone()
            ),
            BalancesError::InsufficientBalance
        );

        assert_ok!(AdvancaCore::register_user(
            Origin::signed(user_account),
            deposit,
            user_public_key.clone()
        ));
        assert_eq!(
            AdvancaCore::get_user(&user_account),
            User {
                account_id: user_account.clone(),
                public_key: user_public_key.clone()
            }
        );
        assert_eq!(
            System::events().last().unwrap().event,
            TestEvent::advanca_core(RawEvent::UserAdded(user_account.clone()))
        );

        assert_ok!(AdvancaCore::register_worker(
            Origin::signed(worker_account),
            deposit,
            enclave.clone()
        ));
        assert_eq!(
            AdvancaCore::get_worker(&worker_account),
            Worker {
                account_id: worker_account.clone(),
                enclave: enclave.clone()
            }
        );
        assert_eq!(
            System::events().last().unwrap().event,
            TestEvent::advanca_core(RawEvent::WorkerAdded(worker_account.clone()))
        );
    })
}

#[test]
//TODO: check freeBalance after deposit (blocked by injecting balance at beginning)
fn submit_task() {
    new_test_ext().execute_with(|| {
        let account = 0x0;
        let lease = Default::default();
        let task_spec: TaskSpec<Privacy> = Default::default();
        // make sure panics panic
        assert_noop!(
            AdvancaCore::submit_task(Origin::NONE, lease, task_spec.clone()),
            DispatchError::BadOrigin
        );

        // verify storage change are expected
        let task_id1 = AdvancaCore::task_id(&account, 0);
        let task_id2 = AdvancaCore::task_id(&account, 1);

        assert_ok!(AdvancaCore::submit_task(
            Origin::signed(account),
            lease,
            task_spec.clone(),
        ));
        assert_eq!(
            System::events().last().expect("should have an event").event,
            TestEvent::advanca_core(RawEvent::TaskSubmitted(task_id1))
        );
        //FIXME: the current mock runtime doesn't increase nonce automatically after dispatchable functions are called
        assert_eq!(System::account_nonce(&account), 0); // so the nonce is expected to be 0;
        System::inc_account_nonce(&account); // then we manually increase the nonce by one

        assert_ok!(AdvancaCore::submit_task(
            Origin::signed(account),
            lease,
            task_spec.clone(),
        ));
        System::inc_account_nonce(&account);
        assert_eq!(System::account_nonce(&account), 2); // now the nonce is 2
        assert_eq!(
            System::events().last().expect("should have an event").event,
            TestEvent::advanca_core(RawEvent::TaskSubmitted(task_id2))
        );

        assert_eq!(AdvancaCore::unscheduled_tasks(), vec![task_id1, task_id2]);
        assert_eq!(Tasks::<Test>::contains_key(task_id1), true);
        assert_eq!(Tasks::<Test>::contains_key(task_id2), true);

        assert_eq!(
            AdvancaCore::get_task(task_id1),
            Task {
                task_id: task_id1,
                status: TaskStatus::Unscheduled,
                owner: account,
                task_spec: task_spec.clone(),
                lease: lease,
                worker: None,
                worker_url: None,
            }
        );
        assert_eq!(
            AdvancaCore::get_task(task_id2),
            Task {
                task_id: task_id2,
                status: TaskStatus::Unscheduled,
                owner: account,
                task_spec: task_spec.clone(),
                lease: lease,
                worker: None,
                worker_url: None,
            }
        );
    });
}

#[test]
fn accept_task() {
    new_test_ext().execute_with(|| {
        let user_account = 0x0;
        let worker_account = 0x1;
        let fake_task_id = Default::default();
        let url: Vec<u8> = "url_in_ciphertext".into();

        // ensure origin is checked
        assert_noop!(
            AdvancaCore::accept_task(Origin::NONE, fake_task_id, url.clone()),
            DispatchError::BadOrigin
        );

        // ensure task_id check is working
        assert_noop!(
            AdvancaCore::accept_task(Origin::signed(worker_account), fake_task_id, url.clone()),
            "task_id must exist"
        );

        let task_id = Default::default();
        Tasks::<Test>::insert(
            &task_id,
            Task {
                status: TaskStatus::Scheduled,
                ..Default::default()
            },
        );

        assert_noop!(
            AdvancaCore::accept_task(Origin::signed(worker_account), task_id, url.clone()),
            "task must not be scheduled"
        );

        let lease = Default::default();
        let task_spec = Default::default();
        // check task is scheduled
        assert_ok!(AdvancaCore::submit_task(
            Origin::signed(user_account),
            lease,
            task_spec,
        ));
        let tasks = AdvancaCore::unscheduled_tasks();
        let task_id = tasks.last().expect("should have a task_id").to_owned();

        // ensure the worker is registered
        assert_noop!(
            AdvancaCore::accept_task(Origin::signed(worker_account), task_id, url.clone()),
            AdvancaCoreError::NotFound
        );

        let deposit = Default::default();
        let enclave: Enclave<u64> = Default::default();
        assert_ok!(AdvancaCore::register_worker(
            Origin::signed(worker_account),
            deposit,
            enclave.clone()
        ));

        assert_ok!(AdvancaCore::accept_task(
            Origin::signed(worker_account),
            task_id.clone(),
            url.clone()
        ));

        let accepted_task = AdvancaCore::get_task(task_id);
        assert_eq!(accepted_task.status, TaskStatus::Scheduled);
        assert_eq!(accepted_task.worker, Some(worker_account));
        assert_eq!(accepted_task.worker_url, Some(url));

        assert_eq!(AdvancaCore::unscheduled_tasks().len(), 0);

        assert_eq!(
            System::events().last().expect("should have an event").event,
            TestEvent::advanca_core(RawEvent::TaskAccepted(task_id))
        );
    })
}

#[test]
//TODO: check emitted event
//TODO: check prohibited log task_spec
fn update_task() {
    // fn update_task(origin, task_id: TaskId<T>, task_spec: TaskSpec)
    new_test_ext().execute_with(|| {
        let account = 0x0;
        let task_id = AdvancaCore::task_id(&account, 0);
        let fake_account = 0x1;
        let fake_task_id = AdvancaCore::task_id(&fake_account, 0);
        let task_spec: TaskSpec<Privacy> = Default::default();

        // make sure panics panic
        assert_noop!(
            AdvancaCore::update_task(Origin::NONE, fake_task_id, task_spec.clone()),
            DispatchError::BadOrigin
        );

        let lease = Default::default();
        assert_ok!(AdvancaCore::submit_task(
            Origin::signed(account),
            lease,
            task_spec.clone()
        ));
        assert_noop!(
            AdvancaCore::update_task(Origin::signed(account), fake_task_id, task_spec.clone()),
            "task_id must exist"
        );

        assert_noop!(
            AdvancaCore::update_task(Origin::signed(fake_account), task_id, task_spec),
            "only owner can update this task"
        );

        let mut task_spec_update: TaskSpec<Privacy> = Default::default();
        task_spec_update.privacy = Privacy::Encryption;
        // check storage
        assert_ok!(AdvancaCore::update_task(
            Origin::signed(account),
            task_id,
            task_spec_update.clone()
        ));
        assert_eq!(
            AdvancaCore::get_task(task_id),
            Task {
                task_id: task_id,
                status: TaskStatus::Unscheduled,
                owner: account,
                worker: None,
                lease: lease,
                task_spec: task_spec_update,
                worker_url: None,
            }
        )
    })
}

#[test]
fn abort_task() {
    new_test_ext().execute_with(|| {
        let account = 0x0;
        let task_id = AdvancaCore::task_id(&account, 0);
        let fake_account = 0x1;
        let fake_task_id = AdvancaCore::task_id(&fake_account, 0);
        let default = Default::default();

        // make sure panics panic
        assert_noop!(
            AdvancaCore::abort_task(Origin::NONE, default),
            DispatchError::BadOrigin
        );

        let lease = Default::default();
        let task_spec = Default::default();
        assert_ok!(AdvancaCore::submit_task(
            Origin::signed(account),
            lease,
            task_spec,
        ));
        assert_noop!(
            AdvancaCore::abort_task(Origin::signed(account), fake_task_id),
            "task_id must exist"
        );
        assert_noop!(
            AdvancaCore::abort_task(Origin::signed(fake_account), task_id),
            "only owner can abort the task"
        );

        assert_ok!(AdvancaCore::abort_task(Origin::signed(account), task_id));
        assert_eq!(Tasks::<Test>::contains_key(task_id), false);

        assert_eq!(
            System::events().last().unwrap().event,
            TestEvent::advanca_core(RawEvent::TaskAborted(task_id))
        );
    })
}
