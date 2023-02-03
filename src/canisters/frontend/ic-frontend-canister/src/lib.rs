use ic_cdk::export::candid::Reserved;
use ic_cdk_macros::{init, post_upgrade, pre_upgrade};
use ic_certified_assets::StableState;
use ic_certified_assets::state_machine::{STABLE_VERSION, V1StableState};

#[init]
fn init() {
    ic_certified_assets::init();
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::storage::stable_save((ic_certified_assets::pre_upgrade(), STABLE_VERSION))
        .expect("failed to save stable state");
}

#[post_upgrade]
fn post_upgrade() {
    let stable_state = if let Ok((state, _)) =
        ic_cdk::storage::stable_restore::<(ic_certified_assets::StableState, u32)>()
    {
        state
    } else if let Ok((_, _version)) = ic_cdk::storage::stable_restore::<(Reserved, u32)>() {
        // version must be 1
        let (v1,_v) = ic_cdk::storage::stable_restore::<(V1StableState, u32)>()
            .expect("failed to restore v1 stable state");
        StableState::from_v1(v1)
    } else {
        let (v0,): (ic_certified_assets::V0StableState,) =
            ic_cdk::storage::stable_restore().expect("failed to restore v0 stable state");
        StableState::from_v0(v0)
    };
    ic_certified_assets::post_upgrade(stable_state);
}
