import { tauri } from "@tauri-apps/api";

type Card = {
	due: Date,
	ease: number,
	interval: number,
	note_id: string,
	deck_id: string,
	cardNum: number,
	state: number
}


export const load = async ({ params }: any) => {
	const { deck } = params;
	const notes: Card[] = await tauri.invoke("list_notes", { deck });

	return {
		params,
		notes
	};
};
