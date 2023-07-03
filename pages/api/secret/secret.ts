import { invokeFetcher } from "../invoke/useInvoke";
import { secretObject } from "../models/secret.object";

export const get_secrets = async (token: string): Promise<secretObject[] | undefined> => {
    try {
      const data: unknown = await invokeFetcher(["get_secrets", { token }]);
      const jobject = JSON.parse(data as string);
      console.log("Get secrets", jobject);
        return jobject as secretObject[];
    } catch (err) {
      console.error("Get secrets failed", err);
      return undefined;
    }
  };

  export const create_secret = async (name: string, content: string, token: string): Promise<void> => {
    try {
      await invokeFetcher(["create_secret", { name, content, token }]);
    } catch (err) {
      console.error("Create secret failed", err);
    }
  }

  export const delete_secret = async (id: number): Promise<void> => {
    try {
      await invokeFetcher(["delete_secret", { id }]);
    } catch (err) {
      console.error("Delete secret failed", err);
    }
  }
