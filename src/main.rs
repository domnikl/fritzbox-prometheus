use std::collections::HashMap;
use std::error;

use fritzapi::AVMDevice;
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;

#[derive(Debug)]
struct Metrics {
    energy: f32,
    power: f32,
    voltage: f32,
    temperature: f32,
}

impl Metrics {
    fn new(energy: f32, power: f32, voltage: f32, temperature: f32) -> Metrics {
        Metrics {
            energy,
            power,
            voltage,
            temperature,
        }
    }
}

fn fetch_device_metrics(sid: &str, device: &AVMDevice) -> Result<Metrics, Box<dyn error::Error>> {
    let stats = device.fetch_device_stats(sid)?;
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

fn extract_metrics(sid: &str) -> Result<HashMap<String, Metrics>, Box<dyn error::Error>> {
    let binding = fritzapi::list_devices(&sid)?;
    let metrics = binding.iter().map(|device: &AVMDevice| {
        (
            device.name().to_owned(),
            fetch_device_metrics(sid, device).unwrap(),
        )
    });

    Ok(HashMap::from_iter(metrics))
}

fn main() -> Result<(), Box<(dyn error::Error + 'static)>> {
    let user = std::env::var("FRITZBOX_USERNAME").expect("FRITZBOX_USERNAME env variable missing");
    let password =
        std::env::var("FRITZBOX_PASSWORD").expect("FRITZBOX_PASSWORD env variable missing");

    let sid = fritzapi::get_sid(&user, &password).unwrap();

    let builder = PrometheusBuilder::new();
    builder
        .install()
        .expect("Failed to install recorder/exporter");

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let metrics = extract_metrics(&sid).unwrap();

        for (key, value) in &metrics {
            let labels = [("name", format!("{}", key))];
            gauge!("gaia_energy", value.energy as f64, &labels);
            gauge!("gaia_power", value.power as f64, &labels);
            gauge!("gaia_voltage", value.voltage as f64, &labels);
            gauge!("gaia_temperature", value.temperature as f64, &labels);
        }

        println!("{:?}", metrics);
    })
    .join()
    .unwrap();

    Ok(())
}
