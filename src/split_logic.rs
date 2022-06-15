use crate::{player_manager::{Hand, Player}, card_manager::Shoe};

pub fn split_hands(hands: &Hand, shoe: &mut Shoe) -> Vec<Hand> {
        let mut new_hands = vec![];
        new_hands.push(hands.clone());

        if let Some(card) = new_hands[0].hand.pop() {
            new_hands.push(Hand { hand: vec![card] })
        }
            for i in 0..2 {
                new_hands[i].hand.push(shoe.draw_card());
            }
        new_hands
}

pub fn change_coords_of_split_cards(player: &mut Player) {
    let hands = &mut player.hands;
    let point = player.split_coords_point;

        let coords = vec![
            (point.0 - 100, point.1), (point.0 + 150, point.1), (point.0 - 100, point.1 - 200), (point.0 + 150, point.1 - 200)];

        for i in 0..hands.len() {
                hands[i].hand[0].coords = coords[i];
                hands[i].hand[1].coords = coords[i];
                hands[i].hand[1].coords.0 += 20;
                hands[i].hand[1].coords.1 -= 20;
        }
}