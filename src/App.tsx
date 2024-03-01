import {LeftPanel} from "./components/LeftPanel.tsx";
import {GameList} from "./components/GameList.tsx";
import {SettingsButton} from "./components/SettingsButton.tsx";
import {PageArea} from "./components/PageArea.tsx";
import {GameName} from "./components/GameName.tsx";
import {GameBackground} from "./components/GameBackground.tsx";
import {ProfileDropdown} from "./components/ProfileDropdown.tsx";
import {PlayButton} from "./components/PlayButton.tsx";
import {VersionDropdown} from "./components/VersionDropdown.tsx";
import {Window} from "./components/Window.tsx";

export function App() {
  return (
    <Window>
      <LeftPanel>
        <GameList/>
        <SettingsButton/>
      </LeftPanel>
      <PageArea>
        <GameName/>
        <GameBackground/>
        <ProfileDropdown/>
        <PlayButton/>
        <VersionDropdown/>
      </PageArea>
    </Window>
  )
}