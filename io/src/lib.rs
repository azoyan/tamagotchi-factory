#![no_std]

use gmeta::{InOut, Metadata};
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

pub type TamagotchiId = u64;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = InOut<(), ()>;
    type Handle = InOut<FactoryAction, FactoryEvent>;
    type Reply = InOut<(), ()>;
    type Others = InOut<(), ()>;
    type Signal = ();
    type State = ();
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FactoryAction {
    CreateTamagotchi { name: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum FactoryEvent {
    TamagotchiCreated {
        tamagotchi_id: TamagotchiId,
        tamagotchi_address: ActorId,
    },
}
