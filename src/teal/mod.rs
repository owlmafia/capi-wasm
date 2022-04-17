pub mod customer_escrow;
pub mod dao_app_approval;
pub mod dao_app_clear;
pub mod investing_escrow;
pub mod locking_escrow;
pub mod update_teal;

// use core::flows::create_dao::create_dao::{Escrows, Programs};
// use core::teal::{TealSource, TealSourceTemplate};

// pub fn programs() -> Programs {
//     Programs {
//         central_app_approval: TealSourceTemplate(app_central_approval::SRC.as_bytes().to_vec()),
//         central_app_clear: TealSource(app_central_clear::SRC.as_bytes().to_vec()),
//         escrows: Escrows {
//             customer_escrow: TealSourceTemplate(customer_escrow::SRC.as_bytes().to_vec()),
//             invest_escrow: TealSourceTemplate(investing_escrow::SRC.as_bytes().to_vec()),
//             locking_escrow: TealSourceTemplate(locking_escrow::SRC.as_bytes().to_vec()),
//         },
//     }
// }
