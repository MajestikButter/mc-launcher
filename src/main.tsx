import "./styles.css";
import { render } from "preact";
import { Provider } from "react-redux";

import { Window } from "./components/Window";
import { LeftPanel } from "./components/LeftPanel";
import { GameList } from "./components/GameList";
import { SettingsButton } from "./components/SettingsButton";
import { PageArea } from "./components/PageArea";
import { GameName } from "./components/GameName";
import { GameBackground } from "./components/GameBackground";
import { ProfileDropdown } from "./components/ProfileDropdown";
import { PlayButton } from "./components/PlayButton";

import store from "./store";
import { ipcInvoke } from "./ipc";
import { updateProfiles } from "./store/profiles";
import { updateGames } from "./store/games";
import { updateVersions } from "./store/versions";
import { VersionDropdown } from "./components/VersionDropdown";

const games = await ipcInvoke("list_games");
store.dispatch(updateGames(games));
const game = games[0];
const name = game?.name;
if (name) {
  const vers = await ipcInvoke("list_versions");
  store.dispatch(updateVersions(vers));
  const profs = await ipcInvoke("list_game_profiles", { name });
  store.dispatch(updateProfiles(profs));
}

render(
  <Provider store={store}>
    <Window>
      <LeftPanel>
        <GameList />
        <SettingsButton />
      </LeftPanel>
      <PageArea>
        <GameName />
        <GameBackground />
        <ProfileDropdown />
        <PlayButton />
        <VersionDropdown />
      </PageArea>
    </Window>
  </Provider>,
  document.getElementById("root")!
);
