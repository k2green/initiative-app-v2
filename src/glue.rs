use std::path::PathBuf;

use common_data_lib::{BackendError, creatures::{Creature, ConflictGroup}};
use serde::Serialize;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::Callback;

use crate::{Error, emit_callback_if_ok};

#[derive(Debug, Serialize)]
struct PathArgs {
    path: PathBuf
}

#[derive(Debug, Serialize)]
pub struct ExtensionFilter {
    name: String,
    extensions: Vec<String>
}

impl ExtensionFilter {
    pub fn new(name: impl Into<String>, extensions: Vec<&'static str>) -> Self {
        Self {
            name: name.into(),
            extensions: extensions.into_iter().map(|e| e.to_string()).collect()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OpenDialogOptions {
    #[serde(rename = "defaultPath")]
    pub default_path: Option<String>,
    pub directory: bool,
    pub filters: Option<Vec<ExtensionFilter>>,
    pub multiple: bool,
    pub recursive: bool,
}

#[derive(Debug, Serialize)]
pub struct SaveDialogOptions {
    #[serde(rename = "defaultPath")]
    pub default_path: Option<String>,
    pub filters: Option<Vec<ExtensionFilter>>,
}

pub async fn get_creatures() -> Result<Vec<Creature>, Error> {
    let result = invoke_no_args("get_creatures").await.map_err(js_to_error)?;
    serde_wasm_bindgen::from_value(result).map_err(Error::SerdeWasmBindgenError)
}

pub fn get_creatures_with_callback(callback: impl Into<Callback<Vec<Creature>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_creatures(), callback.into()));
}

#[derive(Debug, Serialize)]
struct AddCreaturesArgs {
    creatures: String
}

pub async fn add_creatures(creatures: impl Into<String>) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&AddCreaturesArgs { creatures: creatures.into() }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("add_creatures", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn add_creatures_with_callback(creatures: impl Into<String>, callback: impl Into<Callback<()>>) {
    let creatures = creatures.into();
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(add_creatures(creatures), callback.into()));
}

#[derive(Debug, Serialize)]
struct RemoveCreatureArgs {
    id: Uuid
}

pub async fn remove_creature(id: Uuid) -> Result<Creature, Error> {
    let args = serde_wasm_bindgen::to_value(&RemoveCreatureArgs { id }).map_err(Error::SerdeWasmBindgenError)?;
    let result = invoke("remove_creature", args).await.map_err(js_to_error)?;
    serde_wasm_bindgen::from_value(result).map_err(Error::SerdeWasmBindgenError)
}

pub fn remove_creature_with_callback(id: Uuid, callback: impl Into<Callback<Creature>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(remove_creature(id), callback.into()));
}

#[derive(Debug, Serialize)]
struct SetSelectedArgs {
    id: Uuid,
    selected: bool
}

pub async fn set_creature_selected(id: Uuid, selected: bool) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SetSelectedArgs { id, selected }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("set_creature_selected", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn set_creature_selected_with_callback(id: Uuid, selected: bool, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(set_creature_selected(id, selected), callback.into()));
}

#[derive(Debug, Serialize)]
struct SetInitiativeArgs {
    id: Uuid,
    initiative: isize
}

pub async fn set_creature_initiative(id: Uuid, initiative: isize) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SetInitiativeArgs { id, initiative }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("set_creature_initiative", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn set_creature_initiative_with_callback(id: Uuid, initiative: isize, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(set_creature_initiative(id, initiative), callback.into()));
}

#[derive(Debug, Serialize)]
struct SetAllSelectedArgs {
    selected: bool
}

pub async fn set_all_creatures_selected(selected: bool) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SetAllSelectedArgs { selected }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("set_all_creatures_selected", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn set_all_creatures_selected_with_callback(selected: bool, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(set_all_creatures_selected(selected), callback.into()));
}

pub async fn reset_all_initiatives() -> Result<(), Error> {
    invoke_no_args("reset_all_initiatives").await.map_err(js_to_error)?;
    Ok(())
}

pub fn reset_all_initiatives_with_callback(callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(reset_all_initiatives(), callback.into()));
}

#[derive(Debug, Serialize)]
struct GetConflictsArgs {
    #[serde(rename = "setConflicts")]
    set_conflicts: bool
}

pub async fn get_initiative_conflicts(set_conflicts: bool) -> Result<Vec<ConflictGroup>, Error> {
    let args = GetConflictsArgs { set_conflicts };
    let args_value = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    let value = invoke("get_initiative_conflicts", args_value).await.map_err(js_to_error)?;
    serde_wasm_bindgen::from_value(value).map_err(Error::SerdeWasmBindgenError)
}

pub fn get_initiative_conflicts_with_callback(set_conflicts: bool, callback: impl Into<Callback<Vec<ConflictGroup>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_initiative_conflicts(set_conflicts), callback.into()));
}

#[derive(Debug, Serialize)]
struct MoveConflictArgs {
    #[serde(rename = "groupIndex")]
    group_index: usize,
    #[serde(rename = "moveIndex")]
    move_index: usize,
    #[serde(rename = "targetIndex")]
    target_index: usize
}

pub async fn move_initiative_conflict(group_index: usize, move_index: usize, target_index: usize) -> Result<(), Error> {
    let args = MoveConflictArgs { group_index, move_index, target_index };
    let args_value = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    invoke("move_initiative_conflict", args_value).await.map_err(js_to_error)?;
    Ok(())
}

pub fn move_initiative_conflict_with_callback(group_index: usize, move_index: usize, target_index: usize, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(move_initiative_conflict(group_index, move_index, target_index), callback.into()));
}

pub async fn finalize_initiative_order() -> Result<(), Error> {
    invoke_no_args("finalize_initiative_order").await.map_err(js_to_error)?;
    Ok(())
}

pub fn finalize_initiative_order_with_callback(callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(finalize_initiative_order(), callback.into()));
}

pub async fn get_active_encounter_creatures() -> Result<Vec<Creature>, Error> {
    let value = invoke_no_args("get_active_encounter_creatures").await.map_err(js_to_error)?;
    serde_wasm_bindgen::from_value(value).map_err(Error::SerdeWasmBindgenError)
}

pub fn get_active_encounter_creatures_with_callback(callback: impl Into<Callback<Vec<Creature>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(get_active_encounter_creatures(), callback.into()));
}

#[derive(Debug, Serialize)]
struct AddCreaturesToTncounterArgs {
    creatures: String
}

pub async fn add_creatures_to_active_encounter(creatures: impl Into<String>) -> Result<(), Error> {
    let args = AddCreaturesToTncounterArgs { creatures: creatures.into() };
    let args = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    let value = invoke("add_creatures_to_active_encounter", args).await.map_err(js_to_error)?;
    serde_wasm_bindgen::from_value(value).map_err(Error::SerdeWasmBindgenError)
}

pub fn add_creatures_to_active_encounter_with_callback(creatures: impl Into<String>, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(add_creatures_to_active_encounter(creatures.into()), callback.into()));
}

#[derive(Debug, Serialize)]
struct ChangeEncounterOrderArgs {
    #[serde(rename = "moveIndex")]
    move_index: usize,
    #[serde(rename = "targetIndex")]
    target_index: usize
}

pub async fn change_active_encounter_order(move_index: usize, target_index: usize) -> Result<(), Error> {
    let args = ChangeEncounterOrderArgs { move_index, target_index };
    let args = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    invoke("change_active_encounter_order", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn change_active_encounter_order_with_callback(move_index: usize, target_index: usize, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(change_active_encounter_order(move_index, target_index), callback.into()));
}

#[derive(Debug, Serialize)]
struct RemoveFromActiveEncounterArgs {
    id: Uuid
}

pub async fn remove_from_active_encounter(id: Uuid) -> Result<(), Error> {
    let args = RemoveFromActiveEncounterArgs { id };
    let args = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    invoke("remove_from_active_encounter", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn remove_from_active_encounter_with_callback(id: Uuid, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(remove_from_active_encounter(id), callback.into()));
}

pub async fn save_encounter(path: impl Into<PathBuf>) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&PathArgs { path: path.into() }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("save_encounter", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn save_encounter_with_callback(path: impl Into<PathBuf>, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(save_encounter(path.into()), callback.into()));
}

pub async fn load_encounter(path: impl Into<PathBuf>) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&PathArgs { path: path.into() }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("load_encounter", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn load_encounter_with_callback(path: impl Into<PathBuf>, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(load_encounter(path.into()), callback.into()));
}

pub async fn new_encounter() -> Result<(), Error> {
    invoke_no_args("new_encounter").await.map_err(js_to_error)?;
    Ok(())
}

pub fn new_encounter_with_callback(callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(new_encounter(), callback.into()));
}

pub async fn open_encounter_dialog() -> Result<Option<PathBuf>, Error> {
    let args = OpenDialogOptions {
        default_path: Some(dirs::home_dir().unwrap_or(PathBuf::from("/home")).to_string_lossy().to_string()),
        directory: false,
        multiple: false,
        recursive: false,
        filters: get_encounter_filters()
    };

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    let result = serde_wasm_bindgen::from_value(open(args_value).await).map_err(Error::SerdeWasmBindgenError)?;

    Ok(result)
}

pub fn open_encounter_dialog_with_callback(callback: impl Into<Callback<Option<PathBuf>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(open_encounter_dialog(), callback.into()));
}

pub async fn save_encounter_dialog() -> Result<Option<PathBuf>, Error> {
    let args = SaveDialogOptions {
        default_path: Some(dirs::home_dir().unwrap_or(PathBuf::from("/home")).to_string_lossy().to_string()),
        filters: get_encounter_filters()
    };

    let args_value = serde_wasm_bindgen::to_value(&args).map_err(Error::SerdeWasmBindgenError)?;
    let result = serde_wasm_bindgen::from_value(save(args_value).await).map_err(Error::SerdeWasmBindgenError)?;

    Ok(result)
}

pub fn save_encounter_dialog_with_callback(callback: impl Into<Callback<Option<PathBuf>>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(save_encounter_dialog(), callback.into()));
}

fn get_encounter_filters() -> Option<Vec<ExtensionFilter>> {
    Some(vec![
        ExtensionFilter::new("Encounter", vec!["enc", "encounter"])
    ])
}

fn js_to_error(value: JsValue) -> Error {
    match serde_wasm_bindgen::from_value::<BackendError>(value) {
        Ok(err) => Error::BackendError(err),
        Err(err) => Error::SerdeWasmBindgenError(err)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], js_name = "invoke", catch)]
    async fn invoke_no_args(cmd: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn save(args: JsValue) -> JsValue;
}