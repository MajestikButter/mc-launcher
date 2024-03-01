import { configureStore } from "@reduxjs/toolkit";
import gamesReducer from "./games";
import profsReducer from "./profiles";
import setsReducer from "./settings";
import versReducer from "./versions";

const store = configureStore({
  reducer: {
    games: gamesReducer,
    profiles: profsReducer,
    settings: setsReducer,
    versions: versReducer,
  },
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;

export default store;
