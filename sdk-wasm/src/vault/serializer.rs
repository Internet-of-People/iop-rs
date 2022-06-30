//! Cannot use typetags in wasm_bindgen, because it depends on module constructors. So as a workaround
//! until the <https://github.com/mmastrac/rust-ctor/issues/14> issue is resolved.

use super::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "pluginName")]
enum VaultPluginSerializer {
    Hydra(hd_hydra::Plugin),
    Morpheus(hd_morpheus::Plugin),
}

impl From<VaultPluginSerializer> for Box<dyn VaultPlugin> {
    fn from(plugin: VaultPluginSerializer) -> Box<dyn VaultPlugin> {
        use VaultPluginSerializer::*;
        match plugin {
            Hydra(x) => Box::new(x),
            Morpheus(x) => Box::new(x),
        }
    }
}

/// See module level documentation why this is here.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultSerializer {
    encrypted_seed: String,
    plugins: Vec<VaultPluginSerializer>,
}

impl From<VaultSerializer> for Vault {
    fn from(mut ser: VaultSerializer) -> Self {
        let plugins = ser.plugins.drain(..).map(|p| p.into()).collect::<Vec<_>>();
        Self::new(ser.encrypted_seed, plugins, false)
    }
}
