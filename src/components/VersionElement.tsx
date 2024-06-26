import styled from "styled-components";

interface VersionElementProperties {
  name: string;
  installed?: boolean;
  onClick: () => void;
}
export function VersionElement(props: VersionElementProperties) {
  const { name, installed, onClick } = props;
  return (
    <Button onClick={onClick}>
      <Title>{name}<br/>{installed && " [Installed]"}</Title>
    </Button>
  );
}

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
