import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { RootState } from ".";

export enum VersionType {
  release = "release",
  preview = "preview",
  custom = "custom",
}

export interface VersionInfo {
  versionType: VersionType;
  name: string;
}

interface VersionsState {
  versions: VersionInfo[];
  latest: { [k in VersionType]: string };
}

const initialState: VersionsState = {
  versions: [],
  latest: {
    [VersionType.release]: "1.20.0.1",
    [VersionType.preview]: "1.20.0.23",
    [VersionType.custom]: "",
  },
};

const versSlice = createSlice({
  name: "versions",
  initialState,
  reducers: {
    updateVersions: (
      state,
      action: PayloadAction<VersionInfo[]>,
    ) => {
      state.versions = action.payload;
    },
  },
});

export const { updateVersions } = versSlice.actions;

export const selectVersions = ({ versions }: RootState) => versions.versions;
export const selectGameVersions = createSelector(
  [
    selectVersions,
    (_state: RootState, versionType: VersionType) => versionType,
  ],
  (vers, versionType) => vers.filter((v) => v.versionType === versionType),
);

export default versSlice.reducer;
