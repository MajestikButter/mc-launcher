import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { RootState } from ".";

export enum VersionType {
  none = -1,
  release = 0,
  preview = 2,
  custom = 3,
}

export interface VersionInfo {
  versionType: VersionType;
  name: string;
  installed: boolean;
}

interface VersionsState {
  versions: VersionInfo[];
}

const initialState: VersionsState = {
  versions: [],
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
