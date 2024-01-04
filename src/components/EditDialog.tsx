import { PropsWithChildren, createPortal } from "preact/compat";
import styled from "styled-components";

interface DialogProperties {}
export function EditDialog(props: PropsWithChildren<DialogProperties>) {
  const { children } = props;
  const root = document.getElementById("root")!;
  return createPortal(
    <Dialog
      open
      onClick={(e: MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();
      }}
    >
      {children}
    </Dialog>,
    root
  );
}

const Dialog = styled.dialog`
  position: fixed;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);

  ::backdrop {
    background-image: linear-gradient(45deg, magenta, rebeccapurple, dodgerblue, green);
    opacity: 0.75;
  }
`;
