import { render } from "preact";
import App from "./App";
import "./styles.css";
import { Provider } from "react-redux";
import store from "./store";
import "./event";
import { ipcInvoke } from "./ipc";
import { sleep } from "./utils";

await sleep(50);
const game = (await ipcInvoke("request_games_update"))[0];
await sleep(50);
if (game) {
  await ipcInvoke("request_profiles_update", { name: game.name });
  await sleep(50);
}

render(
  <Provider store={store}>
    <App />
  </Provider>,
  document.getElementById("root")!
);
