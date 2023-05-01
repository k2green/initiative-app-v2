use std::path::PathBuf;

use common_data_lib::{BackendError, creatures::Creature};
use serde::Serialize;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::Callback;

use crate::{Error, emit_callback_if_ok};

#[derive(Debug, Serialize)]
struct AddCreaturesArgs {
    creatures: String
}

#[derive(Debug, Serialize)]
struct RemoveCreatureArgs {
    id: Uuid
}

#[derive(Debug, Serialize)]
struct SetSelectedArgs {
    id: Uuid,
    selected: bool
}

#[derive(Debug, Serialize)]
struct SetInitiativeArgs {
    id: Uuid,
    initiative: isize
}

#[derive(Debug, Serialize)]
struct SetSubOrderArgs {
    id: Uuid,
    sub_order: isize
}

#[derive(Debug, Serialize)]
struct SetAllSelectedArgs {
    selected: bool
}

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

pub async fn add_creatures(creatures: impl Into<String>) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&AddCreaturesArgs { creatures: creatures.into() }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("add_creatures", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn add_creatures_with_callback(creatures: impl Into<String>, callback: impl Into<Callback<()>>) {
    let creatures = creatures.into();
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(add_creatures(creatures), callback.into()));
}

pub async fn remove_creature(id: Uuid) -> Result<Creature, Error> {
    let args = serde_wasm_bindgen::to_value(&RemoveCreatureArgs { id }).map_err(Error::SerdeWasmBindgenError)?;
    let result = invoke("remove_creature", args).await.map_err(js_to_error)?;
    serde_wasm_bindgen::from_value(result).map_err(Error::SerdeWasmBindgenError)
}

pub fn remove_creature_with_callback(id: Uuid, callback: impl Into<Callback<Creature>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(remove_creature(id), callback.into()));
}

pub async fn set_creature_selected(id: Uuid, selected: bool) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SetSelectedArgs { id, selected }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("set_creature_selected", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn set_creature_selected_with_callback(id: Uuid, selected: bool, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(set_creature_selected(id, selected), callback.into()));
}

pub async fn set_creature_initiative(id: Uuid, initiative: isize) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SetInitiativeArgs { id, initiative }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("set_creature_initiative", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn set_creature_initiative_with_callback(id: Uuid, initiative: isize, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(set_creature_initiative(id, initiative), callback.into()));
}

pub async fn set_creature_sub_order(id: Uuid, sub_order: isize) -> Result<(), Error> {
    let args = serde_wasm_bindgen::to_value(&SetSubOrderArgs { id, sub_order }).map_err(Error::SerdeWasmBindgenError)?;
    invoke("set_creature_sub_order", args).await.map_err(js_to_error)?;
    Ok(())
}

pub fn set_creature_sub_order_with_callback(id: Uuid, sub_order: isize, callback: impl Into<Callback<()>>) {
    wasm_bindgen_futures::spawn_local(emit_callback_if_ok(set_creature_sub_order(id, sub_order), callback.into()));
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