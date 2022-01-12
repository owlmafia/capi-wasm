use crate::{
    js::common::{parse_bridge_pars, to_bridge_res},
    model::{
        project_for_users::project_to_project_for_users,
        project_for_users_view_data::ProjectForUsersViewData,
    },
    teal::programs,
};
use anyhow::Result;
use core::{
    dependencies::{algod, env, indexer},
    flows::create_project::storage::load_project::{load_project, ProjectId},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn bridge_load_project_user_view(pars: JsValue) -> Result<JsValue, JsValue> {
    log::debug!("bridge_load_project_user_view, pars: {:?}", pars);

    to_bridge_res(_bridge_load_project_user_view(parse_bridge_pars(pars)?).await)
}

async fn _bridge_load_project_user_view(project_id: String) -> Result<ProjectForUsersViewData> {
    log::debug!("load_project, hash: {:?}", project_id);

    let algod = algod();
    let indexer = indexer();
    let env = env();

    let project_id = ProjectId(project_id);

    let project = load_project(&algod, &indexer, &project_id, &programs().escrows).await?;

    Ok(project_to_project_for_users(&env, &project, &project_id)?.into())
}
