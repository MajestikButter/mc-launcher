import { useEffect, useState } from "preact/hooks";

interface LoadingImageProperties {
  className?: string;
  src?: string;
}

const DEFAULT_IMG = "./unknown_icon.png";
export function LoadingImage(props: LoadingImageProperties) {
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

  return <img className={className} src={loadedSrc} />;
}

