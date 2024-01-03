import styled from "styled-components";
import { useEffect, useState } from "preact/hooks";

interface IconProperties {
  src?: string;
  className?: string;
}

// const DEFAULT_IMG = preactLogo;
const DEFAULT_IMG = "./unknown_icon.png";
export function Icon(props: IconProperties) {
  const { className, src } = props;
  const [loadedSrc, setSrc] = useState(DEFAULT_IMG);
  useEffect(() => {
    if (!src) return;
    const img = new Image();
    console.log("loading", src);
    img.onload = () => {
      console.log("loaded", src);
      setSrc(src);
    };
    img.src = src;
    return () => setSrc(DEFAULT_IMG);
  }, [src]);

  console.log("display", loadedSrc);

  return (
    <IconDiv className={className}>
      <IconImg src={loadedSrc} />
    </IconDiv>
  );
}

const IconImg = styled.img`
  height: 100%;
  width: 100%;
`;
const IconDiv = styled.div`
  display: block;
  aspect-ratio: 1;
`;
