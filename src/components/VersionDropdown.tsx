import styled from "styled-components";
import { useAppSelector } from "../hooks";
import { selectActiveProfile } from "../store/profiles";
import { Dropdown } from "./Dropdown";
import { selectActiveGame } from "../store/games";
import { VersionElement } from "./VersionElement";
import { selectGameVersions } from "../store/versions";

export function VersionDropdown() {
  const activeGame = useAppSelector(selectActiveGame);

  const versions = useAppSelector((state) => selectGameVersions(state, activeGame?.versionType));
  const active = useAppSelector(selectActiveProfile);

  // const dispatch = useAppDispatch();

  return (
    <VersionDrop selected={[<Title>{active?.version ?? "Unknown"}</Title>]}>
      <VersionElement
        name="latest"
        onClick={() => {
          const game = activeGame?.name;
          if (!game) return;
          // dispatch(setActive(game));
          // ipcInvoke("select_profile", { game, profile: prof.name });
        }}
      />
      {versions.map((ver) => (
        <VersionElement
          name={ver.name}
          onClick={() => {
            const game = activeGame?.name;
            if (!game) return;
            // dispatch(setActive(game));
            // ipcInvoke("select_profile", { game, profile: prof.name });
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
