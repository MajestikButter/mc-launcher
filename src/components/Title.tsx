import { PropsWithChildren } from "preact/compat";
import styled from "styled-components";

interface TitleProperties {}
export function Title(props: PropsWithChildren<TitleProperties>) {
  const { children } = props;
  return <Header>{children}</Header>;
}

const Header = styled.h1`
  margin: auto;
  height: 5vh;
  width: calc(100% - 0.2em);
  padding: 0.1em;

  font-size: 3vw;
  font-family: inherit;

  user-select: none;

  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);

  @media (prefers-color-scheme: dark) {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
`;
