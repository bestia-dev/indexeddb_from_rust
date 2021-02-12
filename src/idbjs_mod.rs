// idbjs_mod

// Imported functions from javascript recognize only JsValue and &str
// I want to isolate this functions because they are used only by the idb_mod

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

// rustfmt 1.4.25 bug removes the word async from extern "C". Skip this.
#[rustfmt::skip]
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/indexeddb_lib.js")]
extern "C" {
    pub(crate) fn check_browser_capability();
    /// open db with name currency_rates and returns the idb.IDBPDatabase as JsValue
    #[wasm_bindgen(catch)]
    pub(crate) async fn init_upgrade_db(db_name: &str,version:u32) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) async fn add_key_value(db_name: &str, store: &str, key: &str, value: &str, ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) async fn put_key_value(db_name: &str, store: &str, key: &str, value: &str, ) -> Result<(), JsValue>;
    pub(crate) async fn get_key_value(db_name: &str, store: &str, key: &str, ) -> JsValue;
    pub(crate) fn transaction(db:&JsValue, store:String) ->JsValue;
    pub(crate) fn create_object_store(db:JsValue,store_name:&str);
    pub(crate) fn get_object_store_from_transaction(tx:&JsValue,store_name:&str) -> JsValue;
    pub(crate) fn transaction_object_store_put(tx_ob_st: JsValue, key:&str, value:&str);
}
