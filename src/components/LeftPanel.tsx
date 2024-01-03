import styled from "styled-components";
import { PropsWithChildren } from "preact/compat";

interface LeftPanelProperties {}
export function LeftPanel(props: PropsWithChildren<LeftPanelProperties>) {
  const { children } = props;
  return <Div>{children}</Div>;
}

const Div = styled.div`
  display: flex;
  flex-direction: column;
  justify-content: center;
  height: calc(100% - 7px - 5vh - 7px);
  width: 17vw;
  min-width: 50mm;
  max-width: 80mm;
  padding: calc(5vh + 7px) 9px 7px 9px;
  
  z-index: 1;

  box-shadow: 0 10px 10px rgba(0, 0, 0, 0.2);
  color: #0f0f0f;
  background-color: #ffffff;

  @media (prefers-color-scheme: dark) {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
`;
