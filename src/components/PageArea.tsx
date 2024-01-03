import { PropsWithChildren } from "preact/compat";
import styled from "styled-components";

interface PageAreaProperties {}
export function PageArea(props: PropsWithChildren<PageAreaProperties>) {
  return <Div>{props.children}</Div>;
}

const Div = styled.div`
  position: relative;
  flex-grow: 1;
  height: 100%;
`;
