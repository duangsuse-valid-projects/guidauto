extern crate uuid;
use uuid::Uuid;

//FIXME
pub mod genguid {
    pub fn getguid() -> String {
        let mut u = uuid::Uuid::new_v4();
        u.setVariant(uuid::UuidVariant::Microsoft);
        u.to_string()
    }
}
