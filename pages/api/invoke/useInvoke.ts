import { invoke } from "@tauri-apps/api/tauri";

export const invokeFetcher = async <TArgs extends Record<string, any>, TResult>(
  key
): Promise<TResult> => {
  const [command, args] = key;
  console.log(`[cmd]:${command} [args]:${args}, [key]:${key}`);
  return invoke<TResult>(command, args);
};
