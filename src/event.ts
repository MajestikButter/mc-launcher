import { event } from "@tauri-apps/api";
import { type GameInfo, updateGames } from "./store/games";
import store from "./store";
import { updateProfiles } from "./store/profiles";

interface EventsRouter {
  "update-games": GameInfo[];
  "update-profiles": { game: string; profiles: []; selected: string };
}

function listenTo<K extends keyof EventsRouter>(
  name: K,
  listener: (d: EventsRouter[K]) => void,
) {
  const eventName = "mc-launcher://" + name;
  console.log("Listening for", eventName);
  event.listen<EventsRouter[K]>(eventName, (e) => {
    console.log(`Received ${eventName}`, e.payload);
    listener(e.payload);
  });
}

listenTo("update-games", (games) => store.dispatch(updateGames(games)));
listenTo("update-profiles", (data) => store.dispatch(updateProfiles(data)));
