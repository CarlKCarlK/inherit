use std::collections::HashMap;

/// Simple stand-in traits so this example has no external dependencies.
trait Serialize {}
trait Deserialize {}

#[derive(Debug, Clone)]
struct WifiCredentials {
    ssid: String,
    password: String,
}
impl Serialize for WifiCredentials {}
impl Deserialize for WifiCredentials {}

#[derive(Debug, Clone)]
struct DisplayOnly(String); // Intentionally not serializable.

/// Tiny in-memory "flash block" demo.
///
/// Key point: method-level gating.
/// - The type exists for all use-cases.
/// - Only `save/load<T>` require extra capabilities on `T`.
#[derive(Default)]
struct FlashBlockDemo {
    store: HashMap<String, Vec<u8>>,
}

impl FlashBlockDemo {
    fn new() -> Self {
        Self::default()
    }

    fn clear(&mut self) {
        self.store.clear();
    }

    fn save<T>(&mut self, key: &str, _value: &T)
    where
        T: Serialize + Deserialize,
    {
        // Real code would serialize `value` here.
        self.store.insert(key.to_string(), vec![1, 2, 3]);
    }

    fn load<T>(&self, key: &str) -> Option<T>
    where
        T: Serialize + Deserialize,
    {
        // Real code would deserialize bytes into `T` here.
        if self.store.contains_key(key) {
            None
        } else {
            None
        }
    }
}

fn main() {
    let mut flash = FlashBlockDemo::new();

    let credentials = WifiCredentials {
        ssid: "HomeWiFi".to_string(),
        password: "secret".to_string(),
    };
    println!(
        "ssid={} password_len={}",
        credentials.ssid,
        credentials.password.len()
    );

    // Works: WifiCredentials satisfies Serialize + Deserialize.
    flash.save("wifi", &credentials);
    let _loaded: Option<WifiCredentials> = flash.load("wifi");

    // Uncomment to see method-level gating fail:
    // let label = DisplayOnly("status".to_string());
    // flash.save("label", &label); // error: DisplayOnly doesn't satisfy bounds
    let label = DisplayOnly("status".to_string());
    println!("display-only sample: {}", label.0);

    println!("stored keys: {}", flash.store.len());
    flash.clear();
    println!("stored keys after clear: {}", flash.store.len());
}
