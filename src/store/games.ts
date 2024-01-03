import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import type { RootState } from ".";

export interface GameInfo {
  name: string;
  icon: string;
  background: string;
}

interface GamesState {
  active: number;
  games: GameInfo[];
}

const initialState: GamesState = {
  active: 0,
  games: [],
};

const findGameIdx = (state: GamesState, name: string) => {
  const prev = state.active;
  const idx = state.games.findIndex((v) => v.name === name);
  if (idx !== -1) return idx;
  return Math.min(state.games.length - 1, Math.max(0, prev));
};

const gamesSlice = createSlice({
  name: "games",
  initialState,
  reducers: {
    setActive: (state, action: PayloadAction<string>) => {
      state.active = findGameIdx(state, action.payload);
    },
    updateGames: (state, action: PayloadAction<GameInfo[]>) => {
      const prevName = state.games[state.active]?.name;
      state.games = action.payload;
      state.active = findGameIdx(state, prevName);
    },
  },
});

export const { setActive, updateGames } = gamesSlice.actions;

export const selectGames = ({ games }: RootState) => games.games;
export const selectActiveGameIdx = ({ games }: RootState) => games.active;
export const selectActiveGame = ({ games }: RootState) =>
  games.games[games.active];

export default gamesSlice.reducer;
