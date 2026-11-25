import { invoke } from "@tauri-apps/api/core";

export interface AppStatus {
  hasApiKey: boolean;
}

export interface LazyMeBridge {
  getAppStatus: () => Promise<AppStatus>;
}

/** Default production IPC bridge backed by Tauri commands. */
export const tauriBridge: LazyMeBridge = {
  getAppStatus: () => invoke<AppStatus>("get_app_status"),
};
