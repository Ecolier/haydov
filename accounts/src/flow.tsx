import { LoginFlow, RegistrationFlow, SettingsFlow, VerificationFlow, RecoveryFlow } from "@ory/client";
import { getNodeId } from "@ory/integrations/ui";
import Node from "./Node";

type FlowType = 
| LoginFlow
| RegistrationFlow
| SettingsFlow
| VerificationFlow
| RecoveryFlow;

type FlowProps = {
  flow: FlowType
};

export default function Flow({ flow }: FlowProps) {
  return (
    <form action={flow.ui.action} method={flow.ui.method}>
      {flow.ui.nodes.map((node, index) => {
        const id = getNodeId(node);
        return (
          <Node 
            key={`${id}-${index}`}
            node={node}
          />
        )
      })}
    </form>
  )
}