// Import types from the simulation library.
use tudelft_xray_sim::*;
// Import enum variants to make this example a bit easier to read.
use Dose::*;
use Mode::*;
use Projection::*;

use log::info;

fn main() {
    // Initialize logger.
    simple_logger::init().unwrap();
    // Run simulation with your own implementation of the control logic.
    run_double_plane_sim(Logic::default());
}

/// Example control logic for a two plane system.
/// The pedal mapping is based on the example mapping given in the DSL assignment.
#[derive(Default)]
struct Logic {
    /// keep track of the selected projection
    selected: Projection,
    // you can have whatever other information that you want here
}

impl PedalMapper for Logic {
    /// We use an associated type to determine which pedal enum is used.
    /// Single-plane systems use the `ThreePedals` enum, while
    /// two-plane systems use `SixPedals`.
    /// Whether you used the correct type is checked at compile time.
    type Pedals = SixPedals;

    fn on_press(&self, pedal: Self::Pedals) -> Option<Request> {
        use SixPedals::*;
        Some(match pedal {
            // 3 pedals for low dose X-ray streaming video (one for each projection)
            Pedal1 => Request::start(Frontal, Low, Video),
            Pedal2 => Request::start(Lateral, Low, Video),
            Pedal3 => Request::start(Biplane, Low, Video),
            // 1 pedal to select the high dose projection in a round-robin fashion
            Pedal4 => Request::toggle_selected_projection(),
            // 1 pedal for high dose X-ray streaming video
            Pedal5 => Request::start_selected_projection(High, Video),
            // 1 pedal for high dose X-ray single image
            Pedal6 => Request::start_selected_projection(High, Image),
        })
    }

    fn on_release(&self, pedal: Self::Pedals) -> Option<Request> {
        use SixPedals::*;
        Some(match pedal {
            Pedal1 => Request::stop(Frontal, Low, Video),
            Pedal2 => Request::stop(Lateral, Low, Video),
            Pedal3 => Request::stop(Biplane, Low, Video),
            Pedal4 => return None, // nothing happens when we release pedal 3
            Pedal5 => Request::stop_selected_projection(High, Video),
            Pedal6 => Request::stop_selected_projection(High, Image),
        })
    }
}

impl ActionLogic<true> for Logic {
    /// Naive implementation of request handling which does not handle
    /// multiple pedals being pressed at once.
    fn handle_request(&mut self, request: Request, controller: &mut Controller<true>) {
        // This is how you can get access to the planes in case you want to inspect their status.
        let _frontal = controller.frontal();
        let _lateral = controller.lateral();

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
                controller.activate_xray(projection, dose, mode);
            }

            Request::Stop { .. } => controller.deactivate_xray(),

            Request::ToggleSelectedProjection => {
                self.selected = match self.selected {
                    Frontal => Lateral,
                    Lateral => Biplane,
                    Biplane => Frontal,
                };
                info!("Selected: {:?}", self.selected);
            }

            Request::StartSelectedProjection { dose, mode } => {
                controller.activate_xray(self.selected, dose, mode)
            }

            Request::StopSelectedProjection { .. } => controller.deactivate_xray(),
        }
    }
}