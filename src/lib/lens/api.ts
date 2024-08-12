import { invoke } from "@tauri-apps/api";
import type { DatasourceConfig } from "./types";

export async function createDatasource(config: DatasourceConfig) {
  const res = await invoke('create_datasource', { config })
  return res;
}
