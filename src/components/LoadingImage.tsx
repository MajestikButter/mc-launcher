import { resolveResource } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "preact/hooks";

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
      console.log("loading", srcUrl);
      img.onload = () => {
        console.log("loaded", srcUrl);
        setSrc(srcUrl);
      };
      img.src = srcUrl;
    });
    return () => setSrc(DEFAULT_IMG);
  }, [src]);

  return <img className={className} src={loadedSrc} />;
}
