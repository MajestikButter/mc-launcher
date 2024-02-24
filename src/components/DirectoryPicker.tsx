import {useState} from "preact/hooks";
import {ipcInvoke} from "../ipc";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faFolder} from "@fortawesome/free-solid-svg-icons";

interface DirectoryPickerProperties {
  initialDir: string;
  picked?: (newPath: string) => void;
}

export function DirectoryPicker(props: DirectoryPickerProperties) {
  const {initialDir, picked} = props;
  const [dir, setDir] = useState(initialDir);

  let timeout: NodeJS.Timeout | undefined;
  return (
    <div>
      <input value={dir} onChange={(ev) => {
        const input = ev.srcElement as HTMLInputElement;
        if (timeout) clearTimeout(timeout);
        timeout = setTimeout(() => {
          timeout = undefined;
          if (picked) picked(input.value);
          setDir(input.value);
        }, 1000)
      }}
      />
      <button
        onClick={async () => {
          const newDir = await ipcInvoke("select_dir", {path: dir});
          if (timeout) clearTimeout(timeout);
          if (picked) picked(newDir);
          setDir(newDir);
        }}
      >
        <FontAwesomeIcon icon={faFolder}/>
      </button>
    </div>
  );
}
