import {selectVersionSwitching} from "../store/settings";
import {useAppDispatch, useAppSelector} from "../hooks";
import {ipcInvoke} from "../ipc";
import {selectActiveGame} from "../store/games";
import styled from "styled-components";
import {useState} from "preact/hooks";
import {updateVersions} from "../store/versions.ts";

export function PlayButton() {
  const game = useAppSelector(selectActiveGame)?.name;
  const withVersion = useAppSelector(selectVersionSwitching);
  const [active, setActive] = useState(false)

  const dispatch = useAppDispatch();

  return (
    <Button
      disabled={active}
      onClick={() => {
        console.log(game, active);
        if (!game || active) return;
        setActive(true);
        ipcInvoke("play_game", {game, withVersion}).finally(async () => {
          setActive(false);
          dispatch(updateVersions(await ipcInvoke("list_versions")));
        });
      }}
    >
      Play
    </Button>
  );
}

const Button = styled.button`
  position: absolute;
  bottom: 40px;
  width: 16vw;
  min-width: 30mm;
  max-width: 70mm;
  aspect-ratio: 3.5;
  font-size: 1.5em;
  font-weight: bolder;
  transform: translate(-50%, 0);
  z-index: 2;

  color: #ffffff;
  background-color: #2bac43;
  border: 2px solid transparent;

  &:enabled:hover {
    border-color: #145425;
  }

  &:enabled:active {
    border-color: #145425;
    background-color: #218634;
    transform: translate(-50%, 2px);
  }

  &:disabled {
    background-color: #366940;
    cursor: not-allowed;
  }

  &:disabled:hover {
    border-color: transparent;
  }
`;
