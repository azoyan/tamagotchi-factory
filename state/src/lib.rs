#![no_std]

use gmeta::metawasm;
use gstd::{prelude::*, ActorId};
use tamagotchi_io::*;

#[metawasm]
pub mod metafns {
   pub type State = Escrow;

  

   pub fn tamagotchi_state(state: State) -> TamagotchiState {
       state.state
   }
}