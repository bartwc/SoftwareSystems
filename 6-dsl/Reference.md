# DSL

## Submission
The 6-dsl directory contains a template Xtext project and an examples/ folder.

Your repo should include the following:

1. Examples of your grammar and generated code in the examples/ folder. The example scenarios are listed below.
2. Your modelling grammar in XRayDSL.xtext
3. Your validation in XRayDSLValidator.xtend
4. Your code generation in XRayDSLGenerator.xtend

Templates for these files will be generated once you run Generate Xtext Artifacts.

Project is for Interventional X-Ray System at Philips Healthcare
https://link.springer.com/chapter/10.1007/978-3-642-30729-4_19

## Functional Examples
<div style='text-align: justify;'>
You should provide the following examples, written in your DSL. For the one plane examples also provide the generated 
code.

In total, you should provide 2 one plane examples (with generated code) and 2 two plane examples. The examples should 
encode the following cases:

1. One plane, default configuration
   - Pedal mapping:
         1. low dose
         2. high dose
         3. unused
   - High dose should override low dose
2. One plane, special configuration
   - Pedal mapping:
         1. unused
         2. low dose
         3. high dose
   - Earliest pressed pedal has priority.
3. Two plane, default configuration 
   - Pedal mapping:
   1. low dose, frontal projection
   2. low dose, lateral projection
   3. low dose, biplane projection
   4. high dose on the selected projection
   5. toggle selected projection (round robin)
   6. unused
   - High dose overrides low dose
   - Selection pedal should toggle between frontal, lateral and biplane projection in that order
   - Selection pedal only toggles when no pedal is pressed
4. Two plane, special configuration
   - Pedal mapping:
   1. unused
   2. high dose, frontal projection
   3. high dose, lateral projection
   4. low dose on the selected projection
   5. toggle selected projection (round robin)
   6. high dose, biplane projection
   - Low dose overrides everything except high dose biplane
   - Selection pedal should toggle between frontal, biplane and lateral projection in that order
   - Selection pedal should only toggle when low dose pedal is pressed
If your DSL cannot model some of the examples then provide an alternative example of your DSL with an explanation of 
how you change the static configuration and the dynamic logic. In total you should still have 2 one plane examples 
(with generated code), and 2 two plane examples.

## Example Use of Simulator
Consult the documentation of the package for additional help. If something is unclear, let us know in an email or in 
person during the lab sessions so that we can clear it up.

    In Cargo.toml:
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

In src/main.rs:

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

## Authors
[@Teaching Team - CESE4015 Software Systems]()

## Acknowledgments
* [Part 2 Assignment 3 — DSL](https://cese.pages.ewi.tudelft.nl/software-systems/part-2/assignments/dsl.html)
</div>