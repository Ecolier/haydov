import { getNodeLabel } from "@ory/integrations/ui"
import { NodeInputProps } from "./helpers"

export function NodeInputCheckbox<T>({
  node,
  attributes,
  setValue,
  disabled,
}: NodeInputProps) {
  // Render a checkbox.s
  return (
    <>
      <input type="checkbox"
        name={attributes.name}
        defaultChecked={attributes.value}
        onChange={(e) => setValue(e.target.checked)}
        disabled={attributes.disabled || disabled}
      />
    </>
  )
}