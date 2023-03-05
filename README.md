# fritzbox-prometheus

[![build](https://github.com/domnikl/fritzbox-prometheus/workflows/build%20main/badge.svg)](https://github.com/domnikl/fritzbox-prometheus/actions)
[![Docker Pulls](https://img.shields.io/docker/pulls/domnikl/fritzbox-prometheus-exporter)](https://hub.docker.com/repository/docker/domnikl/fritzbox-prometheus-exporter)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

A Prometheus exporter for Fritzbox AVM Smart Home devices that logs `power`, `voltage`, `energy` and `temperature` to be scraped by a [Prometheus](https://prometheus.io/) instance.

## Run it

```shell
FRITZBOX_USERNAME=<username> FRITZBOX_PASSWORD=<password> cargo run
```

## Docker

```shell
docker run -p 9000:9000 -e FRITZBOX_USERNAME=<username> -e FRITZBOX_PASSWORD=<password> domnikl/fritzbox-prometheus-exporter
```
