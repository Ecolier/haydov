import {UiNodeAnchorAttributes} from '@ory/client';

interface NodeAnchorProps {
  attributes: UiNodeAnchorAttributes;
}

export const NodeAnchor = ({attributes}: NodeAnchorProps) => {
  return (
    <button
      data-testid={`node/anchor/${attributes.id}`}
      onClick={e => {
        e.stopPropagation();
        e.preventDefault();
        window.location.href = attributes.href;
      }}>
      {attributes.title.text}
    </button>
  );
};
