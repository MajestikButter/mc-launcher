import styled from "styled-components";
import { useAppDispatch, useAppSelector } from "../hooks";
import { selectActiveProfile, updateProfile } from "../store/profiles";
import { Dropdown } from "./Dropdown";
import { selectActiveGame } from "../store/games";
import { VersionElement } from "./VersionElement";
import { selectGameVersions } from "../store/versions";
import { selectVersionSwitching } from "../store/settings";
import { ipcInvoke } from "../ipc";

export function VersionDropdown() {
  const activeGame = useAppSelector(selectActiveGame);

  const useVersion = useAppSelector(selectVersionSwitching);
  const versions = useAppSelector((state) => selectGameVersions(state, activeGame?.versionType));
  const active = useAppSelector(selectActiveProfile);

  if (!useVersion || !versions.length) return <div />;

  const dispatch = useAppDispatch();

  return (
    <VersionDrop selected={[<Title>{active?.version ?? "Unknown"}</Title>]}>
      <VersionElement
        name="latest"
        onClick={() => {
          const game = activeGame?.name;
          if (!game) return;
          ipcInvoke("select_profile_version", { game, profile: active.name, version: "latest" });
          dispatch(updateProfile({ game, name: active.name, data: { version: "latest" } }));
        }}
      />
      {versions.map((ver) => (
        <VersionElement
          name={ver.name}
          installed={ver.installed}
          onClick={() => {
            const game = activeGame?.name;
            if (!game) return;
            ipcInvoke("select_profile_version", { game, profile: active.name, version: ver.name });
            dispatch(updateProfile({ game, name: active.name, data: { version: ver.name } }));
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
