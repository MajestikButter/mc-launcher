import {useAppDispatch, useAppSelector} from "../hooks";
import {selectActiveGameIdx, selectGames, setActive, updateGame} from "../store/games";
import styled from "styled-components";
import {GameElement} from "./GameElement";
import {useState} from "preact/hooks";
import {FullGameInfo, ipcInvoke} from "../ipc";
import {EditDialog} from "./EditDialog";
import {updateProfiles} from "../store/profiles";
import {DirectoryPicker} from "./DirectoryPicker.tsx";
import {VersionType} from "../store/versions.ts";

export function GameList() {
  const games = useAppSelector(selectGames);
  const active = useAppSelector(selectActiveGameIdx);

  const [edit, setEdit] = useState<{ info: FullGameInfo, name: string } | null>(null);
  const dispatch = useAppDispatch();

  return (
    <Div>
      {games.map((game, i) => (
        <GameElement
          name={game.name}
          icon={game.icon}
          active={active == i}
          onClick={async () => {
            dispatch(setActive(game.name));
            const data = await ipcInvoke("list_game_profiles", {name: game.name});
            dispatch(updateProfiles(data));
          }}
          onEdit={async () => {
            const info = await ipcInvoke("get_full_game", {game: game.name});
            setEdit({info, name: game.name});
          }}
        />
      ))}
      {edit && (
        <EditDialog title="Edit Game" onConfirm={() => setEdit(null)}>
          Icon Path:
          <DirectoryPicker initialDir={edit.info.iconPath} picked={(newPath) => {
            dispatch(updateGame({name: edit.name, data: {icon: newPath}}));
            ipcInvoke("update_game", {game: edit.name, data: {iconPath: newPath}})
          }}
          />
          Background Path:
          <DirectoryPicker initialDir={edit.info.backgroundPath} picked={(newPath) => {
            dispatch(updateGame({name: edit.name, data: {background: newPath}}));
            ipcInvoke("update_game", {game: edit.name, data: {backgroundPath: newPath}})
          }}
          />
          Destination:
          <DirectoryPicker initialDir={edit.info.destination} picked={(newPath) => {
            ipcInvoke("update_game", {game: edit.name, data: {destination: newPath}})
          }}
          />
          Launch Script:
          <br/>
          <input value={edit.info.launchScript} onChange={(ev) => {
            const el = ev.srcElement as HTMLInputElement;
            ipcInvoke("update_game", {game: edit.name, data: {launchScript: el.value}})
          }}
          />
          <br/>
          Version Type:
          <br/>
          <select onChange={(ev) => {
            const el = ev.srcElement as HTMLSelectElement;
            const version = parseInt(el.value) as VersionType;
            dispatch(updateGame({name: edit.name, data: {versionType: version}}));
            ipcInvoke("update_game", {game: edit.name, data: {useVersion: version}})
          }}
          >
            {versionTexts.map(([key, text]) => (
                <option selected={VersionType[key] === edit.info.useVersion} value={VersionType[key]}>{text}</option>
              )
            )}
          </select>
        </EditDialog>
      )}
    </Div>
  );
}

const versionTexts = [
  ["none", "None"],
  ["release", "Release"],
  ["preview", "Preview"],
  ["custom", "Custom"]
] as const;

const Div = styled.div`
  flex-grow: 1;
  width: 100%;
  padding-right: 10px;
  overflow-y: auto;
`;
