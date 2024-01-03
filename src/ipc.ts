import { invoke } from "@tauri-apps/api";
import { GameInfo } from "./store/games";
import { ProfileInfo } from "./store/profiles";

interface BasicRouter {
  [k: string]: {
    params: any;
    return: any;
  };
}

interface InvokeRouter extends BasicRouter {
  select_profile: {
    params: { game: string; profile: string };
    return: void;
  };
  request_games_update: {
    params: void;
    return: GameInfo[];
  };
  request_profiles_update: {
    params: { name: string };
    return: { game: string; profiles: ProfileInfo[]; selected: string };
  };
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
  const response: any = await invoke(method, { ...params });
  if (response.error != null) {
    console.error("ipcInvoke error", response);
    throw new Error(response.error);
  } else {
    return response.result.data;
  }
}
