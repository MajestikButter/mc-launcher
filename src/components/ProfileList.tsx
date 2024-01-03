import { useAppSelector } from "../hooks";
import { selectActiveGame } from "../store/games";
import styled from "styled-components";
import { ProfileElement } from "./ProfileElement";
import { selectActiveProfileIdx, selectGameProfiles } from "../store/profiles";

export function ProfileList() {
  const activeGame = useAppSelector(selectActiveGame);
  const profiles = useAppSelector((state) => selectGameProfiles(state, activeGame?.name));
  const active = useAppSelector(selectActiveProfileIdx);

  return (
    <WrapperDiv>
      <Div>
        {profiles.map((game, i) => (
          <ProfileElement name={game.name} icon={game.icon} active={active == i} />
        ))}
      </Div>
    </WrapperDiv>
  );
}

const WrapperDiv = styled.div`
  position: absolute;
  left: 0px;
  bottom: calc(10px + 6vw);
  height: 20vh;
  width: 100%;

  display: table-cell;
  vertical-align: bottom;
`;

const Div = styled.div`
  overflow: auto;
  max-height: 20vh;
`;
