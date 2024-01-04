import { useState } from "preact/hooks";
import styled from "styled-components";
import { EditDialog } from "./EditDialog";

export function SettingsButton() {
  const [edit, setEdit] = useState(false);

  return (
    <Button onClick={() => setEdit(true)}>
      Settings
      {edit && (
        <EditDialog>
          <button onClick={() => setEdit(false)}>Close Dialog</button>
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
