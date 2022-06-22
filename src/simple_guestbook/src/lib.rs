mod env;
mod lifetime;

use crate::env::{CanisterEnv, EmptyEnv, Environment};
use candid::{candid_method, CandidType, Principal};

use ic_cdk_macros::*;
use serde::Deserialize;

use ic_ledger_types::AccountIdentifier;

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    str::FromStr,
};

use ic_cdk::print;

const INVOICE_CANISTER: &str = "ttkup-saaaa-aaaai-qngiq-cai";

thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
}

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            env: Box::new(EmptyEnv {}),
            data: Data::default(),
        }
    }
}

#[derive(CandidType, Default, Deserialize)]
struct Data {
    guestbook: Vec<GuestBookEntry>,
    user_status: HashMap<Principal, UserStatus>,
    upgrade_requests: HashMap<Principal, AccountIdentifier>,
}

#[derive(CandidType, Deserialize, Clone)]
struct UpgradeRequest {
    requested_by: Principal,
    status: UpgradeRequestStatus,
}

#[derive(CandidType, Deserialize, Clone)]
enum UpgradeRequestStatus {
    Pending,
    Done,
}

#[derive(CandidType, Deserialize, Clone)]
struct GuestBookEntry {
    text: String,
    author: Principal,
    status: UserStatus,
}

#[derive(CandidType, Deserialize, Clone)]
enum UserStatus {
    Basic,
    Premium,
    Ultimate,
}

// fn get_user_status(principal: &Principal) -> UserStatus {
//     let user_status = RUNTIME_STATE.with(|state| {
//         state
//             .borrow()
//             .data
//             .user_status
//             .get(principal)
//             .unwrap_or(&UserStatus::Basic)
//             .clone()
//     });

//     user_status
// }

#[candid_method(query)]
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}! Welcome!", name)
}

#[candid_method(update)]
#[update(name = "add")]
fn add(text: String) -> bool {
    RUNTIME_STATE.with(|state| add_impl(text, &mut state.borrow_mut()))
}

fn add_impl(text: String, runtime_state: &mut RuntimeState) -> bool {
    let user_status = runtime_state
        .data
        .user_status
        .get(&runtime_state.env.caller())
        .unwrap_or(&UserStatus::Basic)
        .clone();

    runtime_state.data.guestbook.push(GuestBookEntry {
        text,
        author: runtime_state.env.caller(),
        status: user_status,
    });

    true
}

#[candid_method(query, rename = "getAll")]
#[query(name = "getAll")]
fn get_all() -> Vec<GuestBookEntry> {
    RUNTIME_STATE.with(|state| get_all_impl(state.borrow()))
}

fn get_all_impl(runtime_state: Ref<RuntimeState>) -> Vec<GuestBookEntry> {
    runtime_state.data.guestbook.iter().cloned().collect()
}

#[derive(CandidType, Deserialize, Clone)]
struct UserDetails {
    principal: Principal,
    status: UserStatus,
}

#[candid_method(query, rename = "getUserDetails")]
#[query(name = "getUserDetails")]
fn get_user_details() -> UserDetails {
    RUNTIME_STATE.with(|state| get_user_details_impl(state.borrow()))
}

fn get_user_details_impl(runtime_state: Ref<RuntimeState>) -> UserDetails {
    UserDetails {
        principal: runtime_state.env.caller(),
        status: runtime_state
            .data
            .user_status
            .get(&runtime_state.env.caller())
            .unwrap_or(&UserStatus::Basic)
            .clone(),
    }
}

#[derive(CandidType, Deserialize, Clone, Debug)]
enum RequestStatus {
    Pending,
    Paid,
    Empty,
}

#[candid_method(update, rename = "verifyPremium")]
#[update(name = "verifyPremium")]
async fn verify_premium() -> String {
    let caller = RUNTIME_STATE.with(|state| state.borrow().env.caller());

    let acc_id = RUNTIME_STATE.with(|state| {
        state
            .borrow()
            .data
            .upgrade_requests
            .get(&caller)
            .unwrap()
            .clone()
    });

    let (upgrade_result,): (Option<RequestStatus>,) = match ic_cdk::api::call::call(
        Principal::from_str(INVOICE_CANISTER).unwrap(),
        "verify_payment",
        (acc_id.to_string(),),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            print(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ));

            (None,)
        }
    };

    print(format! {"{:?}", upgrade_result});

    match upgrade_result {
        Some(x) => match x {
            RequestStatus::Paid => {
                RUNTIME_STATE.with(|state| {
                    state
                        .borrow_mut()
                        .data
                        .user_status
                        .insert(caller, UserStatus::Premium)
                });
                "You have been promoted to premium membership".into()
            }
            _ => "Invoice has not been paid".into(),
        },
        None => "There was an error at upgrade_premium".into(),
    }
}

#[candid_method(update, rename = "upgradePremium")]
#[update(name = "upgradePremium")]
async fn upgrade_premium() -> String {
    let caller = RUNTIME_STATE.with(|state| state.borrow().env.caller());

    let (upgrade_result,): (Option<AccountIdentifier>,) = match ic_cdk::api::call::call(
        Principal::from_str(INVOICE_CANISTER).unwrap(),
        "upgrade_premium",
        (caller,),
    )
    .await
    {
        Ok(x) => x,
        Err((code, msg)) => {
            print(format!(
                "An error happened during the call: {}: {}",
                code as u8, msg
            ));

            (None,)
        }
    };

    print(format! {"{:?}", upgrade_result});

    match upgrade_result {
        Some(x) => {
            RUNTIME_STATE.with(|state| state.borrow_mut().data.upgrade_requests.insert(caller, x));
            x.to_string()
        }
        None => "There was an error at upgrade_premium".into(),
    }
}

#[candid_method(update, rename = "upgradeUltimate")]
#[update(name = "upgradeUltimate")]
fn upgrade_ultimate() -> bool {
    RUNTIME_STATE.with(|state| upgrade_ultimate_impl(&mut state.borrow_mut()))
}

fn upgrade_ultimate_impl(runtime_state: &mut RuntimeState) -> bool {
    false
}

// Auto export the candid interface
candid::export_service!();

#[query(name = "__get_candid_interface_tmp_hack")]
#[candid_method(query, rename = "__get_candid_interface_tmp_hack")]
fn __export_did_tmp_() -> String {
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_candid() {
        let expected = String::from_utf8(std::fs::read("simple_guestbook.did").unwrap()).unwrap();

        let actual = __export_service();

        if actual != expected {
            println!("{}", actual);
        }

        assert_eq!(
            actual, expected,
            "Generated candid definition does not match expected did file"
        );
    }
}
