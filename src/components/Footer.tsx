import { PropsWithChildren } from "preact/compat";
import styled from "styled-components";

interface FooterProperties {}
export function Footer(props: PropsWithChildren<FooterProperties>) {
  return <StyledDiv>{props.children}</StyledDiv>;
}

const StyledDiv = styled.div`
  width: 100%;
  text-align: left;

  color: #0f0f0f;
  background-color: #cccccc;

  @media (prefers-color-scheme: dark) {
    color: #ffffff;
    background-color: #0f0f0fe0;
  }
`;
