import styled from "styled-components";
import { useAppDispatch } from "../hooks";
import { setActive } from "../store/profiles";
import { ipcInvoke } from "../ipc";
import store from "../store";
import { Icon } from "./Icon";

interface ProfileElementProperties {
  icon?: string;
  name: string;
  active: boolean;
}
export function ProfileElement(props: ProfileElementProperties) {
  const { icon, active, name } = props;
  const dispatch = useAppDispatch();
  return (
    <Button
      $active={active}
      onClick={() => {
        const state = store.getState().games;
        const game = state.games[state.active]?.name;
        if (!game) return;
        dispatch(setActive(name));
        ipcInvoke("select_profile", { game, profile: name });
      }}
    >
      <ProfileIcon src={icon} />
      <Title>{name}</Title>
    </Button>
  );
}

const ProfileIcon = styled(Icon)`
  height: 100%;
`;

const Button = styled.button<{ $active: boolean }>`
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
