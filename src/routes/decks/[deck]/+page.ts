import { tauri } from "@tauri-apps/api";

type Card = {
	front: string;
	back: string;
}

export const load = async ({ params }: any) => {
	const { deck } = params;
	const cards: Card[] = await tauri.invoke("list_cards", { deck });

	return {
		params,
		cards
	};
};
