// Monitor data transmission totals, set thresholds and check alert flags

use std::convert::TryInto;

use nest::{Error, Store, Value};
use rocket::request::FromForm;
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Network traffic data total
#[derive(Debug, Serialize)]
pub struct Data {
    pub total: u64, // total traffic in bytes
}

impl Data {
    /// Retrieve network traffic data values from the store
    fn get(store: &Store) -> Data {
        // retrieve previous network traffic statistics
        let data_stored = match store.get(&["net", "traffic", "total"]) {
            Ok(total) => total,
            // return 0 if no value exists
            Err(_) => Value::Uint(u64::MIN),
        };

        let mut data = Vec::new();
        // retrieve u64 from Value type
        if let Value::Uint(total) = data_stored {
            data.push(total);
        };

        Data { total: data[0] }
    }
}

/// Network traffic notification thresholds and flags (user-defined)
#[derive(Debug, Deserialize, Serialize, FromForm)]
pub struct Threshold {
    warn: u64,       // traffic warning threshold
    cut: u64,        // traffic cutoff threshold
    warn_flag: bool, // traffic warning notification flag
    cut_flag: bool,  // traffic cutoff notification flag
}

impl Threshold {
    /// Retrieve notification thresholds and flags from the store
    fn get(store: &Store) -> Threshold {
        let mut threshold = Vec::new();

        let warn_val = store
            .get(&["net", "notify", "warn"])
            .unwrap_or(Value::Uint(0));
        if let Value::Uint(val) = warn_val {
            threshold.push(val);
        };

        let cut_val = store
            .get(&["net", "notify", "cut"])
            .unwrap_or(Value::Uint(0));
        if let Value::Uint(val) = cut_val {
            threshold.push(val);
        };

        let mut flag = Vec::new();

        let warn_flag = store
            .get(&["net", "notify", "warn_flag"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(state) = warn_flag {
            flag.push(state);
        }

        let cut_flag = store
            .get(&["net", "notify", "cut_flag"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(state) = cut_flag {
            flag.push(state);
        }

        Threshold {
            warn: threshold[0],
            cut: threshold[1],
            warn_flag: flag[0],
            cut_flag: flag[1],
        }
    }

    /// Store notification flags from user data
    fn set(self, store: &Store) {
        store
            .set(&["net", "notify", "warn"], &Value::Uint(self.warn))
            .unwrap();
        store
            .set(&["net", "notify", "cut"], &Value::Uint(self.cut))
            .unwrap();
        store
            .set(
                &["net", "notify", "warn_flag"],
                &Value::Bool(self.warn_flag),
            )
            .unwrap();
        store
            .set(&["net", "notify", "cut_flag"], &Value::Bool(self.cut_flag))
            .unwrap();
    }
}

/// Warning and cutoff network traffic alert flags (programatically-defined)
#[derive(Debug, Serialize)]
pub struct Alert {
    warn: bool,
    cut: bool,
}

impl Alert {
    /// Retrieve latest alert flags from the store
    fn get(store: &Store) -> Alert {
        let mut alert = Vec::new();

        let warn_flag = store
            .get(&["net", "alert", "warn"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(flag) = warn_flag {
            alert.push(flag);
        }

        let cut_flag = store
            .get(&["net", "alert", "cut"])
            .unwrap_or(Value::Bool(false));
        if let Value::Bool(flag) = cut_flag {
            alert.push(flag);
        }

        Alert {
            warn: alert[0],
            cut: alert[1],
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

// set stored traffic total to 0
pub fn reset_data() -> std::result::Result<(), Error> {
    let store = create_store()?;
    store.set(&["net", "traffic", "total"], &Value::Uint(0))?;

    Ok(())
}

pub fn update_store(threshold: Threshold) -> std::result::Result<(), Error> {
    let store = create_store()?;
    Threshold::set(threshold, &store);

    Ok(())
}
