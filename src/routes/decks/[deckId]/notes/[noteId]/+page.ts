import { tauri } from "@tauri-apps/api";

type Note = {
	front: string;
	back: string;
}

export const load = async ({ params }: any) => {
	const { deckId, noteId } = params;
	const note: Note = await tauri.invoke("read_note", { deckId, noteId });

	return {
		params,
		note
	};
};
