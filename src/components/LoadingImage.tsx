import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "preact/hooks";
import styled from "styled-components";

interface LoadingImageProperties {
  className?: string;
  src?: string;
}

const getFileSrc = async (path: string) => {
  path = await resolveResource("./resources/" + path);
  return convertFileSrc(path);
};

const DEFAULT_IMG = await getFileSrc("./assets/unknown_icon.png");
export function LoadingImage(props: LoadingImageProperties) {
  const { className, src } = props;
  const [loadedSrc, setSrc] = useState(DEFAULT_IMG);

  useEffect(() => {
    if (!src) return;
    getFileSrc(src ?? DEFAULT_IMG).then((srcUrl) => {
      const img = new Image();
      console.log("loading", src);
      img.onload = () => {
        console.log("loaded", src);
        setSrc(srcUrl);
      };
      img.src = srcUrl;
    });
    return () => setSrc(DEFAULT_IMG);
  }, [src]);

  return <Img className={className} src={loadedSrc} />;
}

const Img = styled.img`
  user-select: none;
`;
