import { useAppSelector } from "../hooks";
import { selectActiveGameIdx, selectGames } from "../store/games";
import styled from "styled-components";
import { GameElement } from "./GameElement";

export function GameList() {
  const games = useAppSelector(selectGames);
  const active = useAppSelector(selectActiveGameIdx);

  return (
    <Div>
      {games.map((game, i) => (
        <GameElement name={game.name} icon={game.icon} active={active == i} />
      ))}
    </Div>
  );
}

const Div = styled.div`
  height: 100%;
  width: 22vw;
  padding-left: 9px;
  padding-right: 9px;
  padding-top: 7px;

  box-shadow: 0 10px 10px rgba(0, 0, 0, 0.2);
  color: #0f0f0f;
  background-color: #ffffff;

  @media (prefers-color-scheme: dark) {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
`;
