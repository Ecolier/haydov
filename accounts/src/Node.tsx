import {UiNode} from '@ory/client';
import {
  isUiNodeInputAttributes,
} from '@ory/integrations/ui'
import { TextField } from './components/TextField';

type FlowProps = {
  node: UiNode;
}

export default function Node({node}: FlowProps) {
  const {attributes} = node;
  if (isUiNodeInputAttributes(attributes)) {
    switch (attributes.type) {
      case 'text': 
        return <TextField />
    }
  }
}