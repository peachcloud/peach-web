// Monitor data transmission totals, set thresholds and check alert flags

use std::convert::TryInto;

use nest::{Error, Store, Value};
use serde_json::json;

/// Network traffic data values
#[derive(Debug, Serialize)]
pub struct Data {
    pub rx: u64, // total bytes received
    pub tx: u64, // total bytes transmitted
}

impl Data {
    /// Retrieve network traffic data values from the store
    fn get(store: &Store) -> Data {
        // retrieve previous network traffic statistics
        let rx_stored = match store.get(&["net", "traffic", "rx"]) {
            Ok(rx) => rx,
            // return 0 if no value exists
            Err(_) => Value::Uint(u64::MIN),
        };
        let tx_stored = match store.get(&["net", "traffic", "tx"]) {
            Ok(tx) => tx,
            // return 0 if no value exists
            Err(_) => Value::Uint(u64::MIN),
        };

        let mut data = Vec::new();
        // retrieve u64 from Value type
        if let Value::Uint(rx) = rx_stored {
            data.push(rx);
        };
        if let Value::Uint(tx) = tx_stored {
            data.push(tx);
        };

        Data {
            rx: data[0],
            tx: data[1],
        }
    }
}

/// Network traffic notification thresholds and flags (user-defined)
#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Threshold {
    rx_warn: u64,       // received bytes warning threshold
    rx_cut: u64,        // received bytes cutoff threshold
    tx_warn: u64,       // transmitted bytes warning threshold
    tx_cut: u64,        // transmitted bytes cutoff threshold
    rx_warn_flag: bool, // received bytes warning notification flag
    rx_cut_flag: bool,  // received bytes cutoff notification flag
    tx_warn_flag: bool, // transmitted bytes warning notification flag
    tx_cut_flag: bool,  // transmitted bytes cutoff notification flag
}

impl Threshold {
    /// Retrieve notification thresholds and flags from the store
    fn get(store: &Store) -> Threshold {
        let mut threshold = Vec::new();

        let rx_warn_val = store
            .get(&["net", "notify", "rx_warn"])
            .unwrap_or(Value::Uint(0));
        if let Value::Uint(rx) = rx_warn_val {
            threshold.push(rx);
        };

        let rx_cut_val = store
            .get(&["net", "notify", "rx_cut"])
            .unwrap_or(Value::Uint(0));
        if let Value::Uint(rx) = rx_cut_val {
            threshold.push(rx);
        };

        let tx_warn_val = store
            .get(&["net", "notify", "tx_warn"])
            .unwrap_or(Value::Uint(0));
        if let Value::Uint(tx) = tx_warn_val {
            threshold.push(tx);
        };

        let tx_cut_val = store
            .get(&["net", "notify", "tx_cut"])
            .unwrap_or(Value::Uint(0));
        if let Value::Uint(tx) = tx_cut_val {
            threshold.push(tx);
        };

        let mut flag = Vec::new();

        let rx_warn_flag = store
            .get(&["net", "notify", "rx_warn_flag"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(rx) = rx_warn_flag {
            flag.push(rx);
        }

        let rx_cut_flag = store
            .get(&["net", "notify", "rx_cut_flag"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(rx) = rx_cut_flag {
            flag.push(rx);
        }

        let tx_warn_flag = store
            .get(&["net", "notify", "tx_warn_flag"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(tx) = tx_warn_flag {
            flag.push(tx);
        }

        let tx_cut_flag = store
            .get(&["net", "notify", "tx_cut_flag"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(tx) = tx_cut_flag {
            flag.push(tx);
        }

        Threshold {
            rx_warn: threshold[0],
            rx_cut: threshold[1],
            tx_warn: threshold[2],
            tx_cut: threshold[3],
            rx_warn_flag: flag[0],
            rx_cut_flag: flag[1],
            tx_warn_flag: flag[2],
            tx_cut_flag: flag[3],
        }
    }

    /// Store notification flags from user data
    fn set(self, store: &Store) {
        store
            .set(&["net", "notify", "rx_warn"], &Value::Uint(self.rx_warn))
            .unwrap();
        store
            .set(&["net", "notify", "rx_cut"], &Value::Uint(self.rx_cut))
            .unwrap();
        store
            .set(&["net", "notify", "tx_warn"], &Value::Uint(self.tx_warn))
            .unwrap();
        store
            .set(&["net", "notify", "tx_cut"], &Value::Uint(self.tx_cut))
            .unwrap();
        store
            .set(
                &["net", "notify", "rx_warn_flag"],
                &Value::Bool(self.rx_warn_flag),
            )
            .unwrap();
        store
            .set(
                &["net", "notify", "rx_cut_flag"],
                &Value::Bool(self.rx_cut_flag),
            )
            .unwrap();
        store
            .set(
                &["net", "notify", "tx_warn_flag"],
                &Value::Bool(self.tx_warn_flag),
            )
            .unwrap();
        store
            .set(
                &["net", "notify", "tx_cut_flag"],
                &Value::Bool(self.tx_cut_flag),
            )
            .unwrap();
    }
}

/// Warning and cutoff network traffic alert flags (programatically-defined)
#[derive(Debug, Serialize)]
pub struct Alert {
    rx_warn: bool,
    rx_cut: bool,
    tx_warn: bool,
    tx_cut: bool,
}

impl Alert {
    /// Retrieve latest alert flags from the store
    fn get(store: &Store) -> Alert {
        let mut alert = Vec::new();

        let rx_warn_flag = store
            .get(&["net", "alert", "rx_warn"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(rx) = rx_warn_flag {
            alert.push(rx);
        }

        let rx_cut_flag = store
            .get(&["net", "alert", "rx_cut"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(rx) = rx_cut_flag {
            alert.push(rx);
        }

        let tx_warn_flag = store
            .get(&["net", "alert", "tx_warn"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(tx) = tx_warn_flag {
            alert.push(tx);
        }

        let tx_cut_flag = store
            .get(&["net", "alert", "tx_cut"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(tx) = tx_cut_flag {
            alert.push(tx);
        }

        Alert {
            rx_warn: alert[0],
            rx_cut: alert[1],
            tx_warn: alert[2],
            tx_cut: alert[3],
        }
    }
}

fn create_store() -> std::result::Result<Store, Error> {
    // define the path
    let path = xdg::BaseDirectories::new()
        .unwrap()
        .create_data_directory("peachcloud")
        .unwrap();

    // define the schema
    let schema = json!({
        "net": {
            "traffic": "json",
            "alert": "json",
            "notify": "json",
        }
    })
    .try_into()?;

    // create the data store
    let store = Store::new(path, schema);

    Ok(store)
}

pub fn get_alerts() -> std::result::Result<Alert, Error> {
    let store = create_store()?;
    let alerts = Alert::get(&store);

    Ok(alerts)
}

pub fn get_data() -> std::result::Result<Data, Error> {
    let store = create_store()?;
    let data = Data::get(&store);

    Ok(data)
}

pub fn get_thresholds() -> std::result::Result<Threshold, Error> {
    let store = create_store()?;
    let thresholds = Threshold::get(&store);

    Ok(thresholds)
}

pub fn update_store(threshold: Threshold) -> std::result::Result<(), Error> {
    let store = create_store()?;
    Threshold::set(threshold, &store);

    Ok(())
}
