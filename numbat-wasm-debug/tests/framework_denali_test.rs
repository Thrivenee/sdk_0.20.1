use numbat_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
    ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/external_steps/external_steps.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err1_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-addr-len.err1.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_addr_len_err2_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-addr-len.err2.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err1_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err1.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err2_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err2.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_account_sc_addr_err3_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-account-sc-addr.err3.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_balance_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-balance.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_balance_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-balance.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_code_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-code.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_code() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-code.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
#[should_panic]
fn set_check_dcdt_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-dcdt.err.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn set_check_dcdt_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-dcdt.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_nonce_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-nonce.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_nonce_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-nonce.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err1_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err1.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err2_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err2.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err3_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err3.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err4_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err4.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_storage_err5_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.err5.json",
        &contract_map(),
    );
}

#[test]
fn set_check_storage_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-storage.scen.json",
        &contract_map(),
    );
}

#[test]
#[should_panic]
fn set_check_username_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-username.err.json",
        &contract_map(),
    );
}

#[test]
fn set_check_username_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/set-check/set-check-username.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn builtin_func_dcdt_transfer() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/builtin-func-dcdt-transfer.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn dcdt_non_zero_balance_check_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/dcdt-non-zero-balance-check-err.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn dcdt_zero_balance_check_err_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/dcdt-zero-balance-check-err.scen.json",
        &contract_map(),
    );
}

#[test]
#[ignore]
fn multi_transfer_dcdt_rs() {
    numbat_wasm_debug::denali_rs(
        "tests/denali/multi-transfer-dcdt.scen.json",
        &contract_map(),
    );
}

#[test]
fn transfer_rewa_rs() {
    numbat_wasm_debug::denali_rs("tests/denali/transfer-rewa.scen.json", &contract_map());
}

#[test]
#[ignore]
fn transfer_dcdt_rs() {
    numbat_wasm_debug::denali_rs("tests/denali/transfer-dcdt.scen.json", &contract_map());
}

#[test]
fn validator_reward_rs() {
    numbat_wasm_debug::denali_rs("tests/denali/validatorReward.scen.json", &contract_map());
}
