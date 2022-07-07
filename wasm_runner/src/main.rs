use wasmtime::component::{ Component, ComponentType, Linker, Lower};
use wasmtime::{Config, Engine, Store };

#[derive(ComponentType, Lower)]
#[component(record)]
pub struct PciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
}

fn main() {
    let engine = Engine::new(Config::default().wasm_component_model(true))
        .expect("Couldn't construct engine");
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    let component=
        Component::new(&engine, include_bytes!("../../wit_component.wasm")).unwrap();
    let instance = linker.instantiate(&mut store, &component).unwrap();

    instance
        .get_typed_func::<(PciDevice,), (), _>(&mut store, "init")
        .unwrap()
        .call(
            &mut store,
            (PciDevice {
                vendor_id: 10,
                device_id: 10
            },),
        )
        .unwrap();
}
