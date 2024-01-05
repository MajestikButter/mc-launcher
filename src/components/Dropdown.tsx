import styled from "styled-components";
import { ComponentChild } from "preact";
import { PropsWithChildren } from "preact/compat";
import { useState } from "preact/hooks";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faChevronDown, faChevronUp } from "@fortawesome/free-solid-svg-icons";
import { DropdownList } from "./DropdownList";

interface DropdownProperties {
  className?: string;
  selected: ComponentChild;
}
export function Dropdown(props: PropsWithChildren<DropdownProperties>) {
  const { className, selected, children } = props;
  const [open, setOpen] = useState(false);

  return (
    <Button className={className} onClick={() => setOpen(!open)}>
      <DisplayDiv>{selected}</DisplayDiv>
      <FontAwesomeIcon icon={open ? faChevronDown : faChevronUp}></FontAwesomeIcon>
      {open && <DropdownList>{children}</DropdownList>}
    </Button>
  );
}

const DisplayDiv = styled.div`
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
`;
