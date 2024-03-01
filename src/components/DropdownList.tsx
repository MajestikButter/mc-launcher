import styled from "styled-components";
import { PropsWithChildren } from "preact/compat";

interface DropdownListProperties {}
export function DropdownList(props: PropsWithChildren<DropdownListProperties>) {
  const { children } = props;
  return (
    <WrapperDiv>
      <Div>{children}</Div>
    </WrapperDiv>
  );
}

const WrapperDiv = styled.div`
  position: absolute;
  left: 0px;
  bottom: calc(10px + 6vw);
  height: 20vh;
  width: 100%;

  display: table-cell;
  vertical-align: bottom;
`;

const Div = styled.div`
  overflow: auto;
  max-height: 20vh;
`;
