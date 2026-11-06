// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod doctor;
pub mod runner;
pub mod step;

#[allow(unused_imports)]
pub use doctor::{run_doctor, print_doctor_report, DoctorReport};
pub use runner::execute_installation;
pub use step::StepStatus;
