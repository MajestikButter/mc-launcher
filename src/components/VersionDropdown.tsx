import styled from "styled-components";
import {useAppDispatch, useAppSelector} from "../hooks";
import {selectActiveProfile, updateProfile} from "../store/profiles";
import {Dropdown} from "./Dropdown";
import {selectActiveGame} from "../store/games";
import {VersionElement} from "./VersionElement";
import {selectGameVersions, updateVersions, VersionType} from "../store/versions";
import {selectVersionSwitching} from "../store/settings";
import {ipcInvoke} from "../ipc";

export function VersionDropdown() {
  const activeGame = useAppSelector(selectActiveGame);

  const useVersion = useAppSelector(selectVersionSwitching);
  if (!useVersion || activeGame.versionType === VersionType.none) return <div/>;

  const versions = useAppSelector((state) => selectGameVersions(state, activeGame?.versionType));
  const active = useAppSelector(selectActiveProfile);


  const hasLatest = activeGame.versionType === VersionType.release || activeGame.versionType === VersionType.preview;

  const dispatch = useAppDispatch();
  let activeVersion = active?.version;
  const hasVersion = !!versions.find((v) => v.name === activeVersion);

  if (activeVersion === "latest" ? !hasLatest : !hasVersion) {
    activeVersion = "unknown version";
  }

  return (
    <VersionDrop selected={[<Title>{activeVersion}</Title>]}>
      {hasLatest && (<VersionElement
        name="latest"
        onClick={() => {
          const game = activeGame?.name;
          if (!game) return;
          ipcInvoke("update_profile", {game, profile: active.name, data: {version: "latest"}});
          dispatch(updateProfile({game, name: active.name, data: {version: "latest"}}));
        }}
      />)}

      {activeGame.versionType === 3 && (
        <button onClick={async () => {
          await ipcInvoke("import_version");
          dispatch(updateVersions(await ipcInvoke("list_versions")));
        }}
        >Import Version</button>
      )}

      {versions.map((ver) => (
        <VersionElement
          name={ver.name}
          installed={ver.installed}
          onClick={() => {
            const game = activeGame?.name;
            if (!game) return;
            ipcInvoke("update_profile", {game, profile: active.name, data: {version: ver.name}});
            dispatch(updateProfile({game, name: active.name, data: {version: ver.name}}));
          }}
        />
      ))}
    </VersionDrop>
  );
}

const Title = styled.text`
  flex-grow: 1;
  font-weight: bold;
  margin: auto;
`;

const VersionDrop = styled(Dropdown)`
  bottom: 20px;
  right: 10px;
`;
