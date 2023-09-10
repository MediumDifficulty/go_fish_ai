export type GameObserver = {
    deck: Deck,
    otherPlayers: Player[],
    ownDeck: KnownDeck | null,
    id: number,
}

export type Deck = {
    cards: Probability[],
    size: number,
}

export type Probability = {
    type: string,
    value: number,
}

export type Player = {
    cards: Probability[],
}

export type KnownDeck = number[]