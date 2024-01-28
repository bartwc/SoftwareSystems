// Import types from the simulation library.
use tudelft_xray_sim::*;
// Import enum variants to make this example a bit easier to read.
use Dose::*;
use Mode::*;
use Projection::*;

use log::info;
use tudelft_xray_sim::ThreePedals::{Pedal1, Pedal2, Pedal3};
use crate::PedalUsage::{Unused, Used};
use crate::UserPreference::{EarlyOverride, HighOverride, LowOverride};

const DESIGN_CHOICE: UserPreference = LowOverride;
const PEDAL1_USAGE: PedalUsage = Used;
const PEDAL2_USAGE: PedalUsage = Used;
const PEDAL3_USAGE: PedalUsage = Unused;

fn main() {
    // Initialize logger.
    simple_logger::init().unwrap();
    // Run simulation with your own implementation of the control logic.
    //run_double_plane_sim(Logic::default());
    run_single_plane_sim(Logic::default());
}

/// Example control logic for a two plane system.
/// The pedal mapping is based on the example mapping given in the DSL assignment.
#[derive(Default)]
struct Logic {
    selected_dose: Dose,
    selected_mode: Mode,
    high_on: bool,
    low_on: bool,
    // you can have whatever other information that you want here
}

#[derive(PartialEq)]
enum UserPreference{
    HighOverride,
    LowOverride,
    EarlyOverride,
}
#[derive(PartialEq)]
enum PedalUsage{
    Used,
    Unused,
}
impl PedalMapper for Logic {
    /// We use an associated type to determine which pedal enum is used.
    /// Single-plane systems use the `ThreePedals` enum, while
    /// two-plane systems use `SixPedals`.
    /// Whether you used the correct type is checked at compile time.
    type Pedals = ThreePedals;

    fn on_press(&self, pedal: Self::Pedals) -> Option<Request> {
        use ThreePedals::*;
        match pedal {
            // 3 pedals for low dose X-ray streaming video (one for each projection)
            Pedal1 => {
                if PEDAL1_USAGE == Used {
                    Some(Request::start(Frontal, Low, Video))
                } else {
                    None
                }
            },
            Pedal2 => {
                if PEDAL2_USAGE == Used {
                    Some(Request::start(Frontal, High, Video))
                } else {
                    None
                }
            },
            Pedal3 => {
                if PEDAL3_USAGE == Used {
                    Some(Request::start(Frontal, Low, Video))
                } else {
                    None
                }
            }
        }
    }

    fn on_release(&self, pedal: Self::Pedals) -> Option<Request> {
        use ThreePedals::*;
        match pedal {
            Pedal1 => {
                if PEDAL1_USAGE == Used {
                    Some(Request::stop(Frontal, Low, Video))
                } else {
                    None
                }
            },
            Pedal2 => {
                if PEDAL2_USAGE == Used {
                    Some(Request::stop(Frontal, High, Video))
                } else {
                    None
                }
            },
            Pedal3 => {
                if PEDAL3_USAGE == Used {
                    Some(Request::stop(Frontal, Low, Video))
                } else {
                    None
                }
            }
        }
    }
}

impl ActionLogic<false> for Logic {
    /// Naive implementation of request handling which does not handle
    /// multiple pedals being pressed at once.
    fn handle_request(&mut self, request: Request, controller: &mut Controller<false>) {
        // This is how you can get access to the planes in case you want to inspect their status.

        // Custom logging of requests.
        info!("Processing request: {request:?}");

        // In your own code (as well as the code that you generate),
        // you should do checks before using the controller to
        // start and stop X-rays.
        match request {
            Request::Start {
                projection,
                dose,
                mode,
            } => {
                if dose == High {
                    if DESIGN_CHOICE == HighOverride {
                        self.selected_dose = dose;
                        self.selected_mode = mode;
                        self.high_on = true;
                        if self.low_on == false {
                            controller.activate_frontal(dose, mode);
                        } else {
                            controller.deactivate_xray();
                            controller.activate_frontal(dose, mode);
                        }
                    } else if DESIGN_CHOICE == LowOverride || DESIGN_CHOICE == EarlyOverride {
                        self.high_on = true;
                        if self.low_on == false {
                            self.selected_dose = dose;
                            self.selected_mode = mode;
                            controller.activate_frontal(dose, mode);
                        }
                    }
                } else if dose == Low {
                    if DESIGN_CHOICE == LowOverride {
                        self.selected_dose = dose;
                        self.selected_mode = mode;
                        self.low_on = true;
                        if self.high_on == false {
                            controller.activate_frontal(dose, mode);
                        } else {
                            controller.deactivate_xray();
                            controller.activate_frontal(dose, mode);
                        }
                    } else if DESIGN_CHOICE == HighOverride || DESIGN_CHOICE == EarlyOverride {
                        self.low_on = true;
                        if self.high_on == false {
                            self.selected_dose = dose;
                            self.selected_mode = mode;
                            controller.activate_frontal(dose, mode);
                        }
                    }
                }
            }
            Request::Stop {
                projection,
                dose,
                mode, } => {
                if dose == High {
                    self.high_on = false;
                    if self.low_on {
                        controller.deactivate_xray();
                        controller.activate_frontal(Low, Video);
                    } else {
                        controller.deactivate_xray();
                    }
                } else {
                    self.low_on = false;
                    if self.high_on {
                        controller.deactivate_xray();
                        controller.activate_frontal(High, Video);
                    } else {
                        controller.deactivate_xray();
                    }
                }
            }
            _ => {}
        }
    }
}