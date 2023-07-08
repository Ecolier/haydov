import { Configuration, FrontendApi } from "@ory/client"
import { edgeConfig } from "@ory/integrations/next"
import axios from "@/axios";

const frontendApi = new FrontendApi(new Configuration(edgeConfig), undefined, axios);

export default frontendApi;