import "./App.css";
import { Footer } from "./components/Footer";
import styled from "styled-components";
import { GameList } from "./components/GameList";
import { PageArea } from "./components/PageArea";
import { GameName } from "./components/GameName";
import { ProfileDropdown } from "./components/ProfileDropdown";
import { PlayButton } from "./components/PlayButton";
import { GameBackground } from "./components/GameBackground";
import { LeftPanel } from "./components/LeftPanel";
import { SettingsButton } from "./components/SettingsButton";

function App() {
  return (
    <StyledDiv>
      <StyledContentDiv>
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
      </StyledContentDiv>
      <Footer>Created by MajestikButter</Footer>
    </StyledDiv>
  );
}

const StyledDiv = styled.div`
  display: flex;
  height: 100%;
  width: 100%;
  margin: 0;
  flex-direction: column;
  justify-content: center;
  text-align: center;
`;

const StyledContentDiv = styled.div`
  display: flex;
  flex-grow: 1;
`;

export default App;
