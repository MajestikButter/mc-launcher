import { useAppSelector } from "../hooks";
import { selectActiveGame } from "../store/games";
import { Title } from "./Title";

export function GameName() {
  const active = useAppSelector(selectActiveGame);
  return <Title>{active?.name ?? "Unknown"}</Title>;
}
