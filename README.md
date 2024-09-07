<h1 align="center">Lens</h1> <br>
<p align="center">
  <img alt="lens" src="./assets/lens.svg" width="64" height="64" />
</p>

<p align="center">
  Lens through your data. Built with <img alt="tauri" src="./assets/tauri.svg" width="24" height="24" /> <a href="https://tauri.app/">Tauri</a>
</p>

<img alt="demo" align="center" src="./assets/demo.gif" /> <br />

Powerful, lightweight application that provides an interactive interface to run SQL queries on structured files, including Parquet, CSV, and JSON formats.
You can use Lens locally or directly access data stored in cloud services like Amazon AWS or Google GCP.
Lens simplifies data analysis by allowing you to query, transform, and visualize data without complex setup or external databases.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [From source](#from-source)
- [License](#license)

## Features
- 📁 **Support for Multiple File Formats:** Query data stored in Parquet, CSV, and JSON files.
- ☁ **Cloud Integration:** Seamlessly connect to cloud storage services like AWS S3 or Google Cloud Storage to run SQL queries on remote datasets.
- 💻 **Local Support:** Run SQL queries on files located on your local machine.
- 🔎 **SQL Interface:** Use familiar SQL syntax to query and transform data.
- 📓 **Query History:** Save and replay previous SQL queries to streamline repetitive tasks.
- 👁 **Data Visualization:** View query results in tabular format
- ↬ **Export Results:** Easily export query results to CSV, JSON, or other formats.


## Installation

### From source

#### Prerequisites

Lens is built with [tauri](https://tauri.app/) using a [Rust](https://www.rust-lang.org/) backend.
To build lens from source, first clone the git repository:

```bash
git clone https://github.com/oktal/lens.git
```

Then, to install all the dependencies required for tauri, you can follow the official [Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) section of the Tauri' documentation

Use [rustup](https://www.rust-lang.org/tools/install) to install Rust on your system.
Also make sure to install [Nodejs](https://nodejs.org/en)


#### Build

Once you followed all the steps described in the previous section, build and start lens through `npm`:

```bash
npm install
npm run tauri dev
```
## License

This project is licensed under the MIT License. See the [LICENSE] file for more details.
