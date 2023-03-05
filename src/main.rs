use std::error;

use fritzbox_prometheus::FritzboxPrometheus;
use metrics::gauge;
use metrics_exporter_prometheus::PrometheusBuilder;

fn main() -> Result<(), Box<(dyn error::Error + 'static)>> {
    let username =
        std::env::var("FRITZBOX_USERNAME").expect("FRITZBOX_USERNAME env variable missing");
    let password =
        std::env::var("FRITZBOX_PASSWORD").expect("FRITZBOX_PASSWORD env variable missing");

    let mut fritzbox_prometheus = FritzboxPrometheus::new(username, password);

    let builder = PrometheusBuilder::new();
    builder
        .install()
        .expect("Failed to install recorder/exporter");

    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        let metrics = fritzbox_prometheus.extract_metrics().unwrap();

        for (key, value) in &metrics {
            let labels = [("name", key.to_string())];
            gauge!("fritzbox_actor_energy", value.energy as f64, &labels);
            gauge!("fritzbox_actor_power", value.power as f64, &labels);
            gauge!("fritzbox_actor_voltage", value.voltage as f64, &labels);
            gauge!(
                "fritzbox_actor_temperature",
                value.temperature as f64,
                &labels
            );
        }

        println!("{:?}", metrics);
    })
    .join()
    .unwrap();

    Ok(())
}
