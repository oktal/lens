<h1 align="center">Datalens</h1> <br>
<p align="center">
  <img alt="lens" src="./assets/lens.svg" width="64" height="64" />
</p>

<p align="center">
  Lens through your data. Built with <img alt="tauri" src="./assets/tauri.svg" width="24" height="24" /> <a href="https://tauri.app/">Tauri</a>
</p>

<img alt="demo" align="center" src="./assets/demo.gif" />

## Table of Contents

- [Installation](#installation)
  - [From source](#from-source)
- [Features](#features)

## Installation

### From source

#### Prerequisites

Datalens is built with [tauri](https://tauri.app/) using a [Rust](https://www.rust-lang.org/) backend.
To build datalens from source, first clone the git repository:

```bash
git clone https://github.com/oktal/datalens.git
```

Then, to install all the dependencies required for tauri, you can follow the official [Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) section of the Tauri' documentation

Use [rustup](https://www.rust-lang.org/tools/install) to install Rust on your system.
Also make sure to install [Nodejs](https://nodejs.org/en)


#### Build

Once you followed all the steps described in the previous section, build and start datalens through `npm`:

```bash
npm install
npm run tauri dev
```

## Features

* Register location for your data through data sources
  * Amazon S3
    * Login through SSO
  * Google GCS
  * Local directory is also supported out of the box
* Create named entities
  * Database
  * Schema
  * Table
* Create partitioned tables
* Query through SQL
  * Data streaming so that you don't have to wait and data is fetched in batch asynchronously
  * Pause queries