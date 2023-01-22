import { tauri } from "@tauri-apps/api";

type Card = {
	front: string;
	back: string;
}

export const load = async ({ params }: any) => {
	const { deck } = params;
	const notes: Card[] = await tauri.invoke("list_notes", { deck });

	return {
		params,
		notes
	};
};
