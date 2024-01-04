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
import "./event";
import { ipcInvoke } from "./ipc";
import { sleep } from "./utils";

await sleep(50);
const game = (await ipcInvoke("request_games_update"))[0];
await sleep(50);
if (game) {
  await ipcInvoke("request_profiles_update", { name: game.name });
  await sleep(50);
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
      </PageArea>
    </Window>
  </Provider>,
  document.getElementById("root")!
);
