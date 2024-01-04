import { useAppDispatch, useAppSelector } from "../hooks";
import { selectActiveGameIdx, selectGames, setActive } from "../store/games";
import styled from "styled-components";
import { GameElement } from "./GameElement";
import { useState } from "preact/hooks";
import { ipcInvoke } from "../ipc";
import { EditDialog } from "./EditDialog";

export function GameList() {
  const games = useAppSelector(selectGames);
  const active = useAppSelector(selectActiveGameIdx);

  const [edit, setEdit] = useState(false);
  const dispatch = useAppDispatch();

  return (
    <Div>
      {games.map((game, i) => (
        <GameElement
          name={game.name}
          icon={game.icon}
          active={active == i}
          onClick={() => {
            dispatch(setActive(game.name));
            ipcInvoke("request_profiles_update", { name: game.name });
          }}
          onEdit={() => {
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
  );
}

const Div = styled.div`
  flex-grow: 1;
  width: 100%;
  padding-right: 10px;
  overflow-y: auto;
`;
