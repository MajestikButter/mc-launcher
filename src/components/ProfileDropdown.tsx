import styled from "styled-components";
import { useState } from "preact/hooks";
import { useAppDispatch, useAppSelector } from "../hooks";
import { selectActiveProfile, selectGameProfiles } from "../store/profiles";
import { Dropdown } from "./Dropdown";
import { setActive } from "../store/games";
import { FullProfileInfo, ipcInvoke } from "../ipc";
import { ProfileElement } from "./ProfileElement";
import { EditDialog } from "./EditDialog";
import { Icon } from "./Icon";
import { DirectoryPicker } from "./DirectoryPicker";

export function ProfileDropdown() {
  const activeProf = useAppSelector(selectActiveProfile);
  const game = activeProf?.game;

  const profiles = useAppSelector((state) => selectGameProfiles(state, game));

  const [edit, setEdit] = useState<FullProfileInfo | null>(null);
  const dispatch = useAppDispatch();

  return (
    <>
      {edit && (
        <EditDialog
          title="Edit Profile"
          onConfirm={() => {
            setEdit(null);
          }}
        >
          Path: <DirectoryPicker initialDir={edit.path} />
        </EditDialog>
      )}
      <ProfileDrop
        selected={[
          activeProf?.icon && <DropIcon src={activeProf?.icon} />,
          <Title>{activeProf?.name ?? "Unknown"}</Title>,
        ]}
      >
        {profiles.map((prof) => (
          <ProfileElement
            name={prof.name}
            icon={prof.icon}
            onClick={() => {
              dispatch(setActive(game));
              ipcInvoke("select_profile", { game, profile: prof.name });
            }}
            onEdit={async () => {
              const info = await ipcInvoke("get_full_profile", { game, profile: prof.name });
              setEdit(info);
            }}
          />
        ))}
      </ProfileDrop>
    </>
  );
}

const Title = styled.text`
  flex-grow: 1;
  font-weight: bold;
  margin: auto;
`;

const DropIcon = styled(Icon)`
  height: 100%;
`;

const ProfileDrop = styled(Dropdown)`
  bottom: 20px;
  left: 10px;
`;
