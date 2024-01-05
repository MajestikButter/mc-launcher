import { useState } from "preact/hooks";
import { ipcInvoke } from "../ipc";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFolder } from "@fortawesome/free-solid-svg-icons";

interface DirectoryPickerProperties {
  initialDir: string;
}
export function DirectoryPicker(props: DirectoryPickerProperties) {
  const { initialDir } = props;
  const [dir, setDir] = useState(initialDir);
  return (
    <div>
      <input value={dir} />
      <button
        onClick={async () => {
          const newDir = await ipcInvoke("select_dir", { path: dir });
          setDir(newDir);
        }}
      >
        <FontAwesomeIcon icon={faFolder} />
      </button>
    </div>
  );
}
