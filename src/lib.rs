#[derive(Clone)]
pub struct MyAcpiHandler;

impl acpi::AcpiHandler for MyAcpiHandler {
    unsafe fn map_physical_region<T>(&self, _: usize, _: usize) -> acpi::PhysicalMapping<Self, T> {
        unimplemented!()
    }
    fn unmap_physical_region<T>(_: &acpi::PhysicalMapping<Self, T>) {
        unimplemented!()
    }
}
