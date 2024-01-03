import { configureStore } from "@reduxjs/toolkit";
import gamesReducer from "./games";
import profsReducer from "./profiles";

const store = configureStore({
  reducer: {
    games: gamesReducer,
    profiles: profsReducer,
  },
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;

export default store;
