import styled from "styled-components";
import { useState } from "preact/hooks";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChevronDown, faChevronUp } from "@fortawesome/free-solid-svg-icons";
import { useAppSelector } from "../hooks";
import { selectActiveProfile } from "../store/profiles";
import { ProfileList } from "./ProfileList";
import { Icon } from "./Icon";

export function ProfileDropdown() {
  const [open, setOpen] = useState(false);
  const active = useAppSelector(selectActiveProfile);

  return (
    <Button onClick={() => setOpen(!open)}>
      <ProfileDiv>
        {active?.icon && <DropIcon src={active?.icon} />}
        <Title>{active?.name ?? "Unknown"}</Title>
      </ProfileDiv>
      <FontAwesomeIcon icon={open ? faChevronDown : faChevronUp}></FontAwesomeIcon>
      {open && <ProfileList onClick={() => setOpen(false)} />}
    </Button>
  );
}

const Title = styled.text`
  flex-grow: 1;
  font-weight: bold;
  margin: auto;
`;

const DropIcon = styled(Icon)`
  height: 100%;
`;

const ProfileDiv = styled.div`
  display: flex;
  height: 100%;
  flex-grow: 1;
  border-radius: 8px;
  overflow: hidden;
`;

const Button = styled.button`
  display: flex;
  position: absolute;
  padding: 0;
  aspect-ratio: 3.5;
  width: 20vw;
  min-width: 40mm;
  max-width: 90mm;
  bottom: 20px;
  left: 10px;
`;
