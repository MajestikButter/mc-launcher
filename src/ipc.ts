import {invoke} from "@tauri-apps/api";
import {GameInfo} from "./store/games";
import {ProfileInfo} from "./store/profiles";
import {VersionInfo, VersionType} from "./store/versions";

export interface ListGameProfiles {
  profiles: ProfileInfo[];
  selected: string;
  game: string;
}

export interface FullGameInfo {
  iconPath: string;
  backgroundPath: string;
  destination: string;
  launchScript: string;
  useVersion: VersionType;
}

export interface FullProfileInfo {
  iconPath: string;
  path: string;
  subfolders: Record<string, string>;
  version: string;
}

export interface Settings {
  keepOpen: boolean;
  versionSwitching: boolean;
  profilesFolder: string;
  versionsFolder: string;
  versionListEndpoint: string;
}

interface InvokeRouter {
  select_profile: {
    params: { game: string; profile: string };
    return: void;
  };
  get_full_profile: {
    params: { game: string; profile: string };
    return: FullProfileInfo;
  };
  update_profile: {
    params: { game: string; profile: string; data: Partial<FullProfileInfo> };
    return: void;
  };
  play_game: {
    params: { game: string; withVersion: boolean };
    return: void;
  };
  list_games: {
    params: void;
    return: GameInfo[];
  };
  get_full_game: {
    params: { game: string };
    return: FullGameInfo;
  };
  update_game: {
    params: { game: string; data: Partial<FullGameInfo> };
    return: void;
  };
  list_versions: {
    params: void;
    return: VersionInfo[];
  };
  list_game_profiles: {
    params: { name: string };
    return: ListGameProfiles;
  };
  select_dir: {
    params: { path: string };
    return: string;
  };
  get_settings: {
    params: void;
    return: Settings;
  };
  set_settings: {
    params: { settings: Settings };
    return: void;
  };
  import_version: {
    params: void;
    return: void;
  }
}

type OptionalInvoke = {
  [k in keyof InvokeRouter]: InvokeRouter[k]["params"] extends undefined | void
    ? k
    : never;
}[keyof InvokeRouter];
type RequiredInvoke = keyof Omit<InvokeRouter, OptionalInvoke>;

export async function ipcInvoke<T extends RequiredInvoke>(
  method: T,
  params: InvokeRouter[T]["params"],
): Promise<InvokeRouter[T]["return"]>;
export async function ipcInvoke<T extends OptionalInvoke>(
  method: T,
  params?: InvokeRouter[T]["params"],
): Promise<InvokeRouter[T]["return"]>;
export async function ipcInvoke<T extends keyof InvokeRouter>(
  method: T,
  params?: InvokeRouter[T]["params"],
): Promise<InvokeRouter[T]["return"]> {
  if (typeof method !== "string") return;
  console.log("Invoking", method, "with", params);
  const response: any = await invoke(method, {...params});
  if (response.error != null) {
    console.error("ipcInvoke error", response);
    throw new Error(response.error);
  } else {
    return response.result.data;
  }
}
