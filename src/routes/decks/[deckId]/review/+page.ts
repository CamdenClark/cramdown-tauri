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
	console.log(params);
	const { deckId } = params;
	const cards: Card[] = await tauri.invoke("list_cards_to_review", { deck: deckId });

	console.log(cards);

	return {
		cards
	};
};
