import "./styles.css";
import {render} from "preact";
import {Provider} from "react-redux";

import {App} from "./App.tsx";
import {ipcInvoke} from "./ipc";
import store from "./store";
import {updateProfiles} from "./store/profiles";
import {updateGames} from "./store/games";
import {updateVersions} from "./store/versions";
import {updateSettings} from "./store/settings.ts";
import {attachConsole} from "tauri-plugin-log-api";
import {event} from "@tauri-apps/api";

const detach = await attachConsole();

const games = await ipcInvoke("list_games");
store.dispatch(updateGames(games));
const game = games[0];
const name = game?.name;
if (name) {
  const vers = await ipcInvoke("list_versions");
  store.dispatch(updateVersions(vers));
  const profs = await ipcInvoke("list_game_profiles", {name});
  store.dispatch(updateProfiles(profs));
  const settings = await ipcInvoke("get_settings");
  store.dispatch(updateSettings(settings));
}

render(
  <Provider store={store}><App/></Provider>,
  document.getElementById("root")!
);

event.listen("tauri://close-requested", detach);
