/*
 * generated by Xtext 2.33.0
 */
package xray.generator

import xray.xRayDSL.System
import xray.xRayDSL.ThreePedals
import xray.xRayDSL.SixPedals

abstract class RUSTGenerator {

def static CharSequence toToml()'''
[package]
name = "example"
version = "0.1.0"
edition = "2021"

[dependencies]
# this is the simulation library
tudelft-xray-sim = "1.0.0"

# logging libraries
simple_logger = "4.0.0"
log = "0.4"
'''

def static CharSequence toRUST(xray.xRayDSL.System root)'''«»
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

const DESIGN_CHOICE: UserPreference = «root.logic.differentdosebehaviour.behaviour»;
const PEDAL1_USAGE: PedalUsage = «root.configuration.pedal1.usage»;
const PEDAL2_USAGE: PedalUsage = «root.configuration.pedal2.usage»;
const PEDAL3_USAGE: PedalUsage = «root.configuration.pedal3.usage»;


fn main() {
    // Initialize logger.
    simple_logger::init().unwrap();
    // Run simulation with your own implementation of the control logic.
    run_single_plane_sim(Logic::default())
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
enum UserPreference {
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
    type Pedals = «type2Code(root.configuration)»;

    fn on_press(&self, pedal: Self::Pedals) -> Option<Request> {
        use «type2Code(root.configuration)»::*;
        match pedal {
            // 3 pedals for low dose X-ray streaming video (one for each projection)
            Pedal1 => {
                if PEDAL1_USAGE == Used {
                    Some(Request::start(«root.configuration.pedal1.projection»,«root.configuration.pedal1.dose»,«root.configuration.pedal1.mode»))
                } else {
                    None
                }
            },
            Pedal2 => {
                if PEDAL2_USAGE == Used {
                    Some(Request::start(«root.configuration.pedal2.projection»,«root.configuration.pedal2.dose»,«root.configuration.pedal2.mode»))
                } else {
                    None
                }
            },
            Pedal3 => {
                if PEDAL3_USAGE == Used {
                    Some(Request::start(«root.configuration.pedal3.projection»,«root.configuration.pedal3.dose»,«root.configuration.pedal3.mode»))
                } else {
                    None
                }
            }
        }
    }

    fn on_release(&self, pedal: Self::Pedals) -> Option<Request> {
        use «type2Code(root.configuration)»::*;
        match pedal {
            Pedal1 => {
                if PEDAL1_USAGE == Used {
                    Some(Request::stop(«root.configuration.pedal1.projection»,«root.configuration.pedal1.dose»,«root.configuration.pedal1.mode»))
                } else {
                    None
                }
            },
            Pedal2 => {
                if PEDAL2_USAGE == Used {
                    Some(Request::stop(«root.configuration.pedal2.projection»,«root.configuration.pedal2.dose»,«root.configuration.pedal2.mode»))
                } else {
                    None
                }
            },
            Pedal3 => {
                if PEDAL3_USAGE == Used {
                    Some(Request::stop(«root.configuration.pedal3.projection»,«root.configuration.pedal3.dose»,«root.configuration.pedal3.mode»))
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
        '''

        def static dispatch type2Code(ThreePedals pedal)'''
            ThreePedals'''

        def static dispatch type2Code(SixPedals pedal)'''
            SixPedals'''
}

