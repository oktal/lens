<h1 align="center">Datalens</h1> <br>
<p align="center">
<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 32 32"><path fill="currentColor" d="m23.793 14.293l1.414 1.414l-6.408 6.409l-3.75-3.25l-4.342 4.341l-1.414-1.414l5.658-5.659l3.75 3.25zM12 11v5l-2 2v-7zm10-6v8l-2 2V5zm-5 3v7l-2-2V8z" class="ouiIcon__fillSecondary"/><path fill="currentColor" d="M17 0c8.284 0 15 6.716 15 15s-6.716 15-15 15c-3.782 0-7.238-1.4-9.876-3.71l-5.417 5.417l-1.414-1.414l5.417-5.417A14.94 14.94 0 0 1 2 15c0-1.05.108-2.074.313-3.062l1.906.672Q4.002 13.774 4 15c0 7.18 5.82 13 13 13s13-5.82 13-13S24.18 2 17 2c-1.002 0-1.978.113-2.915.328l-.75-1.877A15 15 0 0 1 17 0M9.621 1.937l.75 1.877a13.06 13.06 0 0 0-4.82 5.024l-1.907-.673a15.07 15.07 0 0 1 5.977-6.228"/></svg>
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