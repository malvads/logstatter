# LogStatter

<p align="center">
  <img src="/assets/logstatter.png" alt="LogStatter Logo">
</p>

![build](https://github.com/redBorder/logstatter/actions/workflows/buldrpm.yml/badge.svg?event=pull_request)
![build](https://github.com/redBorder/logstatter/actions/workflows/macos.yml/badge.svg?event=push)
![build](https://github.com/redBorder/logstatter/actions/workflows/linux.yml/badge.svg?event=push)
[![AGPL License](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://opensource.org/licenses/AGPL-3.0)

## Overview

LogStatter is a high-performance monitoring application for logstash written in Rust. is designed to provide lightning-fast real-time process & pipeline anaylsis for Logstash, with output capabilities to Kafka

## Getting Started

1. Clone this repository:

   ```bash
   git clone https://github.com/malvads/logstatter.git
   ```

2. Install dependencies:

   ```bash
   cd logstatter
   cargo build
   ```

3. Run the app

   ```bash
   /path/to/bin/logstatter -c /path/to/config.yml
   ```

## Authors

Miguel Álvarez <malvarez@redborder.com>
