# fritzbox-prometheus

A Prometheus exporter for Fritzbox AVM Smart Home devices that logs `power`, `voltage`, `energy` and `temperature` to be scraped by a [Prometheus](https://prometheus.io/) instance.

## Run it

```shell
FRITZBOX_USERNAME=<username> FRITZBOX_PASSWORD=<password> cargo run
```

## Docker

```shell
docker run -p 9000:9000 -e FRITZBOX_USERNAME=<username> -e FRITZBOX_PASSWORD=<password> domnikl/fritzbox-prometheus-exporter
```
