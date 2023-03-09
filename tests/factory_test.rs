use factory_tamagotchi_io::{FactoryAction, FactoryEvent};
use gtest::{Program, System};

#[test]
fn init_tamagotchi_factory() {
    let system = System::new();
    let tamagotchi_code_id = system.submit_code(
        "../hello-world-gear-academy/target/wasm32-unknown-unknown/release/hello_world.opt.wasm",
    );
    println!("{:?}", tamagotchi_code_id.into_bytes());
    let tamagotchi_factory = Program::current(&system);
    let _res = tamagotchi_factory.send(100, tamagotchi_code_id);

    let payload = FactoryAction::CreateTamagotchi {
        name: "Vasya".to_string()
    };
    let res = tamagotchi_factory.send(100, payload);

    assert!(!res.main_failed());
    assert!(res.log().is_empty());
}
