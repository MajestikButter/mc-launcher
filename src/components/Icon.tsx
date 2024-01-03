import styled from "styled-components";
import { LoadingImage } from "./LoadingImage";

interface IconProperties {
  className?: string;
  src?: string;
}
export function Icon(props: IconProperties) {
  const { className, src } = props;

  return (
    <IconDiv className={className}>
      <IconImg src={src} />
    </IconDiv>
  );
}

const IconImg = styled(LoadingImage)`
  height: 100%;
  width: 100%;
`;
const IconDiv = styled.div`
  display: block;
  aspect-ratio: 1;
`;
