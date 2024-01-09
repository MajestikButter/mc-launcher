import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { RootState } from ".";
import { Settings } from "../ipc";

interface VersionsState {
  settings: Settings;
}

const initialState: VersionsState = {
  settings: {
    keepOpen: true,
    versionSwitching: true,
  },
};

const setsSlice = createSlice({
  name: "settings",
  initialState,
  reducers: {
    updateSettings: (
      state,
      action: PayloadAction<Partial<Settings>>,
    ) => {
      state.settings = { ...state.settings, ...action.payload };
    },
  },
});

export const { updateSettings } = setsSlice.actions;

export const selectSettings = ({ settings }: RootState) => settings.settings;
export const selectVersionSwitching = ({ settings }: RootState) =>
  settings.settings.versionSwitching;

export default setsSlice.reducer;
