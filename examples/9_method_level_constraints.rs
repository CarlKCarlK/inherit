use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct WifiCredentials {
    ssid: String,
    password: String,
}

/// Key point: method-level gating.
/// - The type exists for all use-cases.
/// - Only `save/load<T>` require extra capabilities on `T`.
#[derive(Default)]
struct FlashBlock {
    store: HashMap<String, Vec<u8>>,
}

impl FlashBlock {
    fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.store.clear();
    }

    fn save<T>(&mut self, key: &str, value: &T) -> Result<(), postcard::Error>
    where
        T: Serialize + DeserializeOwned,
    {
        let bytes = postcard::to_stdvec(value)?;
        self.store.insert(key.to_string(), bytes);
        Ok(())
    }

    fn load<T>(&self, key: &str) -> Option<T>
    where
        T: Serialize + DeserializeOwned,
    {
        let bytes = self.store.get(key)?;
        postcard::from_bytes(bytes).ok()
    }
}

// TECHNIQUE NAME: method-level gating.

fn main() -> Result<(), postcard::Error> {
    let mut flash = FlashBlock::new();

    let credentials = WifiCredentials {
        ssid: "HomeWiFi".to_string(),
        password: "secret".to_string(),
    };

    // Works: WifiCredentials satisfies Serialize + Deserialize.
    flash.save("wifi", &credentials)?;
    let loaded: Option<WifiCredentials> = flash.load("wifi");
    let loaded = loaded.unwrap();
    assert_eq!(&loaded.ssid, "HomeWiFi");
    assert_eq!(&loaded.password, "secret");

    flash.clear();
    assert!(flash.load::<WifiCredentials>("wifi").is_none());

    // but ... you may not want this behavior.
    flash.save("number", &42u8)?;
    let loaded: Option<String> = flash.load("number");
    assert!(loaded.is_none());

    Ok(())
}
