import {useState} from "preact/hooks";
import styled from "styled-components";
import {EditDialog} from "./EditDialog";
import {useAppDispatch, useAppSelector} from "../hooks";
import {selectSettings, updateSettings} from "../store/settings";
import {DirectoryPicker} from "./DirectoryPicker.tsx";

export function SettingsButton() {
  const [edit, setEdit] = useState(false);
  const settings = useAppSelector(selectSettings);
  const dispatch = useAppDispatch();

  return (
    <Button onClick={() => setEdit(true)}>
      Settings
      {edit && (
        <EditDialog
          title="Settings"
          onConfirm={() => {
            setEdit(false);
          }}
        >
          <input
            type="checkbox"
            checked={settings.keepOpen}
            onClick={() => dispatch(updateSettings({keepOpen: !settings.keepOpen}))}
          />{" "}
          Keep Open
          <br/>
          <input
            type="checkbox"
            checked={settings.versionSwitching}
            onClick={() => dispatch(updateSettings({versionSwitching: !settings.versionSwitching}))}
          />{" "}
          Version Switching
          <br/>
          Profiles Folder:
          <DirectoryPicker initialDir={settings.profilesFolder} picked={(newPath) => {
            dispatch(updateSettings({profilesFolder: newPath}))
          }}
          />
          Versions Folder:
          <DirectoryPicker initialDir={settings.versionsFolder} picked={(newPath) => {
            dispatch(updateSettings({versionsFolder: newPath}))
          }}
          />
          <br/>
          Version List Endpoint:
          <input
            type="text"
            value={settings.versionListEndpoint}
            onChange={(e) => dispatch(updateSettings({versionListEndpoint: (e.target! as HTMLInputElement).value}))}
          />
        </EditDialog>
      )}
    </Button>
  );
}

const Button = styled.button`
  width: 100%;
  height: 8%;
  font-size: 1.5em;
  font-weight: bold;
`;
