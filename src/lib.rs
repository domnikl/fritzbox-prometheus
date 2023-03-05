use std::{collections::HashMap, error};

use fritzapi::AVMDevice;

#[derive(Debug)]
pub struct FritzboxPrometheus {
    username: String,
    password: String,
    sid: Option<String>,
}

impl FritzboxPrometheus {
    pub fn new(username: String, password: String) -> FritzboxPrometheus {
        FritzboxPrometheus {
            username,
            password,
            sid: None,
        }
    }

    pub fn extract_metrics(&mut self) -> Result<HashMap<String, Metrics>, Box<dyn error::Error>> {
        let sid = match &self.sid {
            None => fritzapi::get_sid(&self.username, &self.password)?,
            Some(s) => s.to_string(),
        };

        self.sid = Some(sid);

        let binding = fritzapi::list_devices(self.sid.as_ref().unwrap())?;
        let metrics = binding.iter().map(|device: &AVMDevice| {
            (
                device.name().to_owned(),
                self.fetch_device_metrics(device).unwrap(),
            )
        });

        Ok(HashMap::from_iter(metrics))
    }

    fn fetch_device_metrics(&self, device: &AVMDevice) -> Result<Metrics, Box<dyn error::Error>> {
        let stats = device.fetch_device_stats(self.sid.as_ref().unwrap())?;
        let mut temperature = 0.0;
        let mut energy = 0.0;
        let mut power = 0.0;
        let mut voltage = 0.0;

        for stat in stats {
            let values = stat.values.first().unwrap();
            let measurement = values.values.first().unwrap();

            match stat.kind {
                fritzapi::DeviceStatsKind::Temperature => temperature = *measurement,
                fritzapi::DeviceStatsKind::Energy => energy = *measurement,
                fritzapi::DeviceStatsKind::Power => power = *measurement,
                fritzapi::DeviceStatsKind::Voltage => voltage = *measurement,
            }
        }

        Ok(Metrics::new(energy, power, voltage, temperature))
    }
}

#[derive(Debug)]
pub struct Metrics {
    pub energy: f32,
    pub power: f32,
    pub voltage: f32,
    pub temperature: f32,
}

impl Metrics {
    pub fn new(energy: f32, power: f32, voltage: f32, temperature: f32) -> Metrics {
        Metrics {
            energy,
            power,
            voltage,
            temperature,
        }
    }
}
