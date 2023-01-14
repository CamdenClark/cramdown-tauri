import { tauri } from "@tauri-apps/api";
import type { PageLoad } from "./$types";

export const load = async (_: any) => {
  const decks: string[] = await tauri.invoke("get_decks");
  return {
    decks,
  };
};
