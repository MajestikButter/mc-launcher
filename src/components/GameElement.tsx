import styled from "styled-components";
import { ipcInvoke } from "../ipc";
import { useAppDispatch } from "../hooks";
import { setActive } from "../store/games";
import { Icon } from "./Icon";

interface GameElementProperties {
  icon?: string;
  name: string;
  active: boolean;
}
export function GameElement(props: GameElementProperties) {
  const { icon, active, name } = props;
  const dispatch = useAppDispatch();
  return (
    <Button
      $active={active}
      onClick={() => {
        dispatch(setActive(name));
        ipcInvoke("request_profiles_update", { name });
      }}
    >
      <GameIcon src={icon} />
      <Title>{name}</Title>
    </Button>
  );
}

const GameIcon = styled(Icon)`
  width: 30%;
`;

const Button = styled.button<{ $active: boolean }>`
  display: flex;
  width: 100%;
  overflow: hidden;
  padding: 0;
  margin-bottom: 3px;
  border-style: solid;
  transition: 100ms linear;

  transform: ${({ $active }) => ($active ? `translate(10px, 0)` : ``)};
`;
const Title = styled.text`
  display: inline;
  height: 20%;
  flex-grow: 1;
  font-size: 2vw;
  font-weight: bold;
  text-overflow: ellipsis;
  overflow: hidden;
`;
