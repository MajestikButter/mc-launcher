import { faEdit } from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";

interface EditButtonProperties {
  onClick: (e: MouseEvent) => void;
}
export function EditButton(props: EditButtonProperties) {
  const { onClick } = props;
  return (
    <FontAwesomeIcon
      icon={faEdit}
      onClick={(e: MouseEvent) => {
        e.preventDefault();
        e.stopPropagation();
        onClick(e);
      }}
    />
  );
}
