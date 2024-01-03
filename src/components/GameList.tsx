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
  flex-grow: 1;
  width: 100%;
  padding-right: 10px;
  overflow-y: auto;
`;
