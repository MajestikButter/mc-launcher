import styled from "styled-components";
import {useState} from "preact/hooks";
import {useAppDispatch, useAppSelector} from "../hooks";
import {selectActiveProfile, selectGameProfiles, updateProfile, setActive} from "../store/profiles";
import {Dropdown} from "./Dropdown";
import {FullProfileInfo, ipcInvoke} from "../ipc";
import {ProfileElement} from "./ProfileElement";
import {EditDialog} from "./EditDialog";
import {Icon} from "./Icon";
import {DirectoryPicker} from "./DirectoryPicker";

export function ProfileDropdown() {
  const activeProf = useAppSelector(selectActiveProfile);
  const game = activeProf?.game;

  const profiles = useAppSelector((state) => selectGameProfiles(state, game));

  const [edit, setEdit] = useState<{ info: FullProfileInfo, name: string } | null>(null);
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
          Icon Path:
          <DirectoryPicker initialDir={edit.info.iconPath} picked={(newPath) => {
            console.log(newPath);
            dispatch(updateProfile({game, name: edit.name, data: {icon: newPath}}));
            ipcInvoke("update_profile", {game, profile: edit.name, data: {iconPath: newPath}});
          }}
          />
          Path:
          <DirectoryPicker initialDir={edit.info.path} picked={(newPath) => {
            ipcInvoke("update_profile", {game, profile: edit.name, data: {path: newPath}})
          }}
          />
        </EditDialog>
      )}
      <ProfileDrop
        selected={[
          activeProf?.icon && <DropIcon src={activeProf?.icon}/>,
          <Title>{activeProf?.name ?? "Unknown"}</Title>,
        ]}
      >
        {profiles.map((prof) => (
          <ProfileElement
            name={prof.name}
            icon={prof.icon}
            onClick={() => {
              dispatch(setActive(prof.name));
              ipcInvoke("select_profile", {game, profile: prof.name});
            }}
            onEdit={async () => {
              const info = await ipcInvoke("get_full_profile", {game, profile: prof.name});
              setEdit({info, name: prof.name});
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
