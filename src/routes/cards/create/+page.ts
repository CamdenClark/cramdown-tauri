import { tauri } from "@tauri-apps/api";

export const load = async ({ params }: any) => {
  const decks: string[] = await tauri.invoke("get_decks");
  return {
    decks,
    params,
  };
};
