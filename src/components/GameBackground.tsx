import { PropsWithChildren } from "preact/compat";
import { useAppSelector } from "../hooks";
import { selectActiveGame } from "../store/games";
import styled from "styled-components";
import { LoadingImage } from "./LoadingImage";

interface GameBackgroundProperties {
  className?: string;
}
export function GameBackground(props: PropsWithChildren<GameBackgroundProperties>) {
  const back = useAppSelector(selectActiveGame)?.background;
  return (
    <Image className={props.className} src={back}>
      {props.children}
    </Image>
  );
}

const Image = styled(LoadingImage)`
  position: absolute;
  top: 0;
  left: 0;
  height: 100%;
  width: 100%;
  z-index: -1;
  object-fit: cover;
  border: transparent;
`;
