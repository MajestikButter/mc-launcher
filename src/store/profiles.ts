import { createSelector, createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { RootState } from ".";

export interface ProfileInfo {
  game: string;
  name: string;
  icon: string;
}

interface GamesState {
  active: number;
  profiles: ProfileInfo[];
}

const initialState: GamesState = {
  active: 0,
  profiles: [],
};

const findProfIdx = (state: GamesState, game: string, profile: string) => {
  const prev = state.active;
  const idx = state.profiles.findIndex((v) => v.name === profile && v.game === game);
  if (idx !== -1) return idx;
  return Math.min(state.profiles.length - 1, Math.max(0, prev));
};

const profsSlice = createSlice({
  name: "profiles",
  initialState,
  reducers: {
    setActive: (state, action: PayloadAction<string>) => {
      const game = state.profiles[state.active]?.game;
      state.active = findProfIdx(state, game, action.payload);
    },
    updateProfiles: (
      state,
      action: PayloadAction<
        { game: string; profiles: ProfileInfo[]; selected: string }
      >,
    ) => {
      const profs = [];
      for (const p of state.profiles) {
        if (p.game === action.payload.game) continue;
        profs.push(p);
      }
      profs.push(...action.payload.profiles);
      state.profiles = profs;
      state.active = findProfIdx(state, action.payload.game, action.payload.selected);
    },
  },
});

export const { setActive, updateProfiles } = profsSlice.actions;

export const selectProfiles = ({ profiles }: RootState) => profiles.profiles;

export const selectGameProfiles = createSelector(
  [selectProfiles, (_state: RootState, game: string) => game],
  (profs, game) => profs.filter((v) => v.game === game),
);
export const selectActiveProfileIdx = ({ profiles }: RootState) =>
  profiles.active;
export const selectActiveProfile = ({ profiles }: RootState) =>
  profiles.profiles[profiles.active];

export default profsSlice.reducer;
