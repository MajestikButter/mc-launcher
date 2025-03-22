import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { RootState } from ".";
import { ipcInvoke, Settings } from "../ipc";

interface VersionsState {
  settings: Settings;
}

const initialState: VersionsState = {
  settings: {
    keepOpen: true,
    versionSwitching: true,
    profilesFolder: "%appdata%/com.majestik.mc-launcher/profiles",
    versionsFolder: "%appdata%/com.majestik.mc-launcher/versions",
    versionListEndpoint:
      "https://raw.githubusercontent.com/ddf8196/mc-w10-versiondb-auto-update/refs/heads/master/versions.json.min",
  },
};

const setsSlice = createSlice({
  name: "settings",
  initialState,
  reducers: {
    updateSettings: (state, action: PayloadAction<Partial<Settings>>) => {
      state.settings = { ...state.settings, ...action.payload };
      ipcInvoke("set_settings", { settings: state.settings });
    },
  },
});

export const { updateSettings } = setsSlice.actions;

export const selectSettings = ({ settings }: RootState) => settings.settings;
export const selectVersionSwitching = ({ settings }: RootState) => settings.settings.versionSwitching;

export default setsSlice.reducer;
