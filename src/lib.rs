#![no_std]
use factory_tamagotchi_io::{FactoryAction, FactoryEvent, TamagotchiId};
use gstd::{msg, prelude::*, prog::ProgramGenerator, ActorId, CodeId};
use tamagotchi_io::{TmgAction, TmgEvent};

const GAS_FOR_CREATION: u64 = 100_000_000_000;

#[derive(Default)]
pub struct TamagotchiFactory {
    pub tamagotchi_number: TamagotchiId,
    pub id_to_address: BTreeMap<TamagotchiId, ActorId>,
    pub tamagotchi_code_id: CodeId,
}
static mut TAMAGOTCHI_FACTORY: Option<TamagotchiFactory> = None;

#[no_mangle]
unsafe extern "C" fn init() {
    let tamagotchi_code_id: CodeId =
        msg::load().expect("Unable to decode CodeId of the Escrow program");
    gstd::debug!("tamagotchi_code_id = {:?}", tamagotchi_code_id);

    let escrow_factory = TamagotchiFactory {
        tamagotchi_code_id,
        ..Default::default()
    };
    TAMAGOTCHI_FACTORY = Some(escrow_factory);
}

impl TamagotchiFactory {
    async fn create_tamagotchi(&mut self, name: String) {
        gstd::debug!("tamagotchi_name = {:?}", &name);
        let (address, _) = ProgramGenerator::create_program_with_gas_for_reply(
            self.tamagotchi_code_id,
            name.encode(),
            GAS_FOR_CREATION,
            0,
        )
        .expect("Error during Tamagotchi program initialization")
        .await
        .expect("Program was not initialized");

        self.tamagotchi_number = self.tamagotchi_number.saturating_add(1);
        self.id_to_address.insert(self.tamagotchi_number, address);
        msg::reply(
            FactoryEvent::TamagotchiCreated {
                tamagotchi_id: self.tamagotchi_number,
                tamagotchi_address: address,
            },
            0,
        )
        .expect("Error during a reply `FactoryEvent::ProgramCreated`");
    }

    fn get_tamagotchi_address(&self, escrow_id: TamagotchiId) -> ActorId {
        *self
            .id_to_address
            .get(&escrow_id)
            .expect("The escrow with indicated id does not exist")
    }
}

async fn send_message(tamagotchi_address: &ActorId, tamagotchi_payload: TmgAction) {
    msg::send_for_reply_as::<_, TmgEvent>(*tamagotchi_address, tamagotchi_payload, msg::value())
        .expect("Error during a sending message to a Escrow program")
        .await
        .expect("Unable to decode EscrowEvent");
}

#[gstd::async_main]
async fn main() {
    let action: FactoryAction = msg::load().expect("Unable to decode `FactoryAction`");
    let factory = unsafe { TAMAGOTCHI_FACTORY.get_or_insert(Default::default()) };
    match action {
        FactoryAction::CreateTamagotchi { name } => factory.create_tamagotchi(name).await,
    }
}
