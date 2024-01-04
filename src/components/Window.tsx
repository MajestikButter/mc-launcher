import { Footer } from "./Footer";
import styled from "styled-components";
import { PropsWithChildren } from "preact/compat";

interface WindowProperties {}
export function Window(props: PropsWithChildren<WindowProperties>) {
  return (
    <StyledDiv>
      <StyledContentDiv>{props.children}</StyledContentDiv>
      <Footer>Created by MajestikButter</Footer>
    </StyledDiv>
  );
}

const StyledDiv = styled.div`
  display: flex;
  height: 100%;
  width: 100%;
  margin: 0;
  flex-direction: column;
  justify-content: center;
  text-align: center;
`;

const StyledContentDiv = styled.div`
  display: flex;
  flex-grow: 1;
`;
