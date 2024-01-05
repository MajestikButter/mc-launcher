import { useAppSelector } from "../hooks";
import { ipcInvoke } from "../ipc";
import { selectActiveGame } from "../store/games";
import styled from "styled-components";

export function PlayButton() {
  const game = useAppSelector(selectActiveGame)?.name;
  return (
    <Button
      onClick={() => {
        if (game) ipcInvoke("play_game", { game });
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

  &:hover {
    border-color: #145425;
  }
  &:active {
    border-color: #145425;
    background-color: #218634;
    transform: translate(-50%, 2px);
  }
`;
