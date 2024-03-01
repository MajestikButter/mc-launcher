import styled from "styled-components";
import { Icon } from "./Icon";
import { EditButton } from "./EditButton";

interface ProfileElementProperties {
  icon?: string;
  name: string;
  onClick: () => void;
  onEdit: () => void;
}
export function ProfileElement(props: ProfileElementProperties) {
  const { icon, name, onClick, onEdit } = props;
  return (
    <Button
      onClick={onClick}
    >
      <ProfileIcon src={icon} />
      <Title>{name}</Title>
      <EditButton onClick={() => onEdit()} />
    </Button>
  );
}

const ProfileIcon = styled(Icon)`
  height: 100%;
`;

const Button = styled.button`
  display: flex;
  height: 4vw;
  width: 100%;
  overflow: hidden;
  padding: 0;
  margin-bottom: 3px;
  border-style: solid;
`;
const Title = styled.text`
  display: inline;
  flex-grow: 1;
  font-size: 1.5vw;
  text-overflow: ellipsis;
  overflow: hidden;
`;
