# LogStatter

<p align="center">
  <img src="/assets/logstatter.png" alt="LogStatter Logo">
</p>

![build](https://github.com/malvads/logstatter/actions/workflows/ci.yml/badge.svg?event=push)


## Overview

LogStatter is a high-performance log monitoring application for logstash written in Rust. is designed to provide lightning-fast real-time pipeline anaylsis for Logstash, with output capabilities to Kafka.

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