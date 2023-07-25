import LoginFlow from "./LoginFlow";

type Props = {
  searchParams: {
    return_to?: string;
    flow?: string;
    refresh?: string;
    aal?: string;
  };
};

export default function Login({searchParams}: Props) {
  return <LoginFlow
    returnTo={searchParams.return_to}
    flowId={searchParams.flow}
    refresh={searchParams.refresh}
    aal={searchParams.aal}
  ></LoginFlow>
}