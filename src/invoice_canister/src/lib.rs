mod env;
mod lifetime;

use crate::env::{CanisterEnv, EmptyEnv, Environment};
use candid::{candid_method, CandidType, Principal};

use ic_cdk_macros::*;
use serde::Deserialize;

use ic_ledger_types::{
    account_balance, AccountBalanceArgs, AccountIdentifier, Subaccount, Tokens, DEFAULT_SUBACCOUNT,
    MAINNET_LEDGER_CANISTER_ID,
};

use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    str::FromStr,
};

use ic_cdk::print;

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
    principals_requests_count: HashMap<Principal, u16>,
    requests: HashMap<String, RequestState>,
}

#[derive(CandidType, Deserialize, Clone)]
struct RequestState {
    principal: Principal,
    subaccount: Subaccount,
    status: RequestStatus,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
enum RequestStatus {
    Pending,
    Paid,
    Empty,
}

#[candid_method(query)]
#[query]
fn greet(name: String) -> String {
    format!("Hello, {}! Welcome!", name)
}

#[candid_method(update, rename = "checkPayment")]
#[update(name = "checkPayment")]
async fn check_payment(acc_id: String) -> bool {
    let req_state = RUNTIME_STATE.with(|state| state.borrow().data.requests.get(&acc_id).cloned());

    match req_state {
        Some(rs) => match rs.status {
            // Only check the balance for pending requests
            RequestStatus::Pending => {
                let balance = account_balance(
                    MAINNET_LEDGER_CANISTER_ID,
                    AccountBalanceArgs {
                        account: AccountIdentifier::new(&ic_cdk::id(), &rs.subaccount),
                    },
                )
                .await
                .expect("call to ledger failed");

                // 0.001 ICP
                if balance >= Tokens::from_e8s(100000) {
                    RUNTIME_STATE.with(|state| {
                        state
                            .borrow_mut()
                            .data
                            .requests
                            .entry(acc_id)
                            .and_modify(|s| s.status = RequestStatus::Paid);
                    });

                    return true;
                } else {
                    return false;
                }
            }
            _ => return false,
        },
        None => return false,
    }
}

#[candid_method(update, rename = "verify_payment")]
#[update(name = "verify_payment")]
fn verify_payment(acc_id: String) -> Option<RequestStatus> {
    RUNTIME_STATE.with(|state| verify_payment_impl(acc_id, &mut state.borrow_mut()))
}

fn verify_payment_impl(acc_id: String, runtime_state: &mut RuntimeState) -> Option<RequestStatus> {
    let req = runtime_state.data.requests.get(&acc_id);

    match req {
        Some(x) => {
            print(format!("Status: {:?}", x.status));
            Some(x.status.clone())
        }
        None => {
            print(format!("Address not found: {}", acc_id));
            None
        }
    }
}

#[candid_method(update, rename = "upgrade_premium")]
#[update(name = "upgrade_premium")]
fn upgrade_premium(principal: Principal) -> Option<AccountIdentifier> {
    RUNTIME_STATE.with(|state| upgrade_premium_impl(principal, &mut state.borrow_mut()))
}

fn upgrade_premium_impl(
    principal: Principal,
    runtime_state: &mut RuntimeState,
) -> Option<AccountIdentifier> {
    let request_count_entry = runtime_state
        .data
        .principals_requests_count
        .entry(principal)
        .or_insert(0);

    *request_count_entry += 1;
    let request_count = *request_count_entry;

    let mut subaccount: [u8; 32] = Default::default();

    if principal.as_slice().len() != 29 {
        panic!("Invalid principal received")
    }
    for i in 0..29 {
        subaccount[i] = principal.as_slice()[i];
    }

    subaccount[30] = request_count.to_be_bytes()[0];
    subaccount[31] = request_count.to_be_bytes()[1];

    print(format!("Subaccount: {:?}", subaccount));

    print(format!(
        "{}",
        hex::encode(AccountIdentifier::new(
            &runtime_state.env.canister_id(),
            &Subaccount(subaccount)
        ))
    ));

    runtime_state.data.requests.insert(
        format!(
            "{}",
            hex::encode(AccountIdentifier::new(
                &runtime_state.env.canister_id(),
                &Subaccount(subaccount)
            ))
        ),
        RequestState {
            principal,
            subaccount: Subaccount(subaccount),
            status: RequestStatus::Pending,
        },
    );

    Some(AccountIdentifier::new(
        &runtime_state.env.canister_id(),
        &Subaccount(subaccount),
    ))
}

#[candid_method(update, rename = "upgrade_ultimate")]
#[update(name = "upgrade_ultimate")]
fn upgrade_ultimate() -> Option<AccountIdentifier> {
    RUNTIME_STATE.with(|state| upgrade_ultimate_impl(&mut state.borrow_mut()))
}

fn upgrade_ultimate_impl(runtime_state: &mut RuntimeState) -> Option<AccountIdentifier> {
    Some(AccountIdentifier::new(
        &runtime_state.env.canister_id(),
        &DEFAULT_SUBACCOUNT,
    ))
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
        let expected = String::from_utf8(std::fs::read("invoice_canister.did").unwrap()).unwrap();

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
