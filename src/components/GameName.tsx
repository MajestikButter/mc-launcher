import { useAppSelector } from "../hooks";
import { selectActiveGame } from "../store/games";
import styled from "styled-components";

export function GameName() {
  const active = useAppSelector(selectActiveGame);
  return <Title>{active?.name ?? "Unknown"}</Title>;
}

const Title = styled.h1`
  margin: auto;
  height: 5vh;
  padding: 0.1em;

  font-size: 3vw;
  font-family: inherit;

  user-select: none;

  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);

  @media (prefers-color-scheme: dark) {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
`;
