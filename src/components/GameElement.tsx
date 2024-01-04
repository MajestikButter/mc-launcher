import styled from "styled-components";
import { Icon } from "./Icon";
import { EditButton } from "./EditButton";

interface GameElementProperties {
  icon?: string;
  name: string;
  active: boolean;
  onClick: () => void;
  onEdit: () => void;
}
export function GameElement(props: GameElementProperties) {
  const { icon, active, name, onClick, onEdit } = props;

  return (
    <Button
      $active={active}
      onClick={onClick}
    >
      <GameIcon src={icon} />
      <Title>{name}</Title>
      <EditButton onClick={onEdit} />
    </Button>
  );
}

const GameIcon = styled(Icon)`
  width: 30%;
`;

const Button = styled.button<{ $active: boolean }>`
  display: flex;
  width: 100%;
  aspect-ratio: 3.33;
  overflow: hidden;
  padding: 0;
  margin-bottom: 3px;
  border-style: solid;
  transition: 100ms linear;

  transform: ${({ $active }) => ($active ? `translate(10px, 0)` : ``)};
`;
const Title = styled.text`
  display: inline;
  height: 50%;
  flex-grow: 1;
  font-size: 7mm;
  font-weight: bold;
  text-overflow: ellipsis;
  overflow: hidden;
`;
