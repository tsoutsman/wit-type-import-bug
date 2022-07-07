wit_bindgen_rust::export!("../export.wit");

struct Export;

impl export::Export for Export {
    fn init(_pci_device: export::PciDevice) {}
}
