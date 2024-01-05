import { PropsWithChildren, createPortal } from "preact/compat";
import styled from "styled-components";
import { Title } from "./Title";

interface DialogProperties {
  title: string;
  onConfirm?: () => void;
}
export function EditDialog(props: PropsWithChildren<DialogProperties>) {
  const { title, onConfirm, children } = props;
  const root = document.getElementById("window")!;
  return createPortal(
    <Overlay>
      <Dialog>
        <Title>{title}</Title>
        <DialogContent>{children}</DialogContent>
        {onConfirm && <button onClick={() => onConfirm()}>Confirm</button>}
      </Dialog>
    </Overlay>,
    root
  );
}

const Overlay = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  z-index: 100;
  user-select: none;

  background: none;
`;

const Dialog = styled.div`
  display: flex;
  justify-content: 1;
  flex-direction: column;
  position: absolute;
  top: 50vh;
  left: 50vw;
  transform: translate(-50%, -50%);
  height: 400px;
  width: 600px;

  overflow: hidden;
  border-radius: 8px;

  box-shadow: 0 10px 10px rgba(0, 0, 0, 0.2);
  color: #0f0f0f;
  background-color: #c4c4c4;

  @media (prefers-color-scheme: dark) {
    color: #ffffff;
    background-color: #232323;
  }
`;

const DialogContent = styled.div`
  margin-top: 10px;
  flex-grow: 1;
`;
