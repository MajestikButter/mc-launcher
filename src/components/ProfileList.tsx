import styled from "styled-components";
import { useAppDispatch, useAppSelector } from "../hooks";
import { setActive } from "../store/profiles";
import { selectActiveGame } from "../store/games";
import { ipcInvoke } from "../ipc";
import { selectActiveProfileIdx, selectGameProfiles } from "../store/profiles";
import { ProfileElement } from "./ProfileElement";
import { useState } from "preact/hooks";
import { EditDialog } from "./EditDialog";

interface ProfileListProperties {
  onClick: () => void;
}

export function ProfileList(props: ProfileListProperties) {
  const { onClick } = props;

  const activeGame = useAppSelector(selectActiveGame);
  const profiles = useAppSelector((state) => selectGameProfiles(state, activeGame?.name));
  const active = useAppSelector(selectActiveProfileIdx);

  const [edit, setEdit] = useState(false);
  const dispatch = useAppDispatch();

  return (
    <WrapperDiv>
      <Div>
        {profiles.map((prof, i) => (
          <ProfileElement
            name={prof.name}
            icon={prof.icon}
            active={active == i}
            onClick={() => {
              const game = activeGame?.name;
              if (!game) return;
              dispatch(setActive(game));
              ipcInvoke("select_profile", { game, profile: prof.name });
              onClick();
            }}
            onEdit={() => {
              onClick();
              setEdit(true);
            }}
          />
        ))}
        {edit && (
          <EditDialog>
            <button onClick={() => setEdit(false)}>Close Dialog</button>
          </EditDialog>
        )}
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
