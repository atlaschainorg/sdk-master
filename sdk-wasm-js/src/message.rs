use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub(crate) inner: atlas_message::Message,
}

crate::conversion::impl_inner_conversion!(Message, atlas_message::Message);
