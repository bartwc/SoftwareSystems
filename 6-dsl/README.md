# DSL

## Importing this project

1. Open eclipse
2. Select this directory as the workspace
3. Click `File > Open Projects From File System ...`
4. Set the Import Source to this directory
5. Press Finish

Project is for Interventional X-Ray System at Philips Healthcare
https://link.springer.com/chapter/10.1007/978-3-642-30729-4_19

## DSL Model - Modelling Language
### *Description*
Use Eclipse Xtext to design a DSL so that a user of your language can choose between different options for the following 
aspects:

- Static configuration:
  - Choose the mapping between pedals and types of X-ray
  - Choose the order of projections when using the selection pedal
- Dynamic logic:
  - Choose the behavior of the selection pedal when other pedals are already pressed
  - Choose the behavior for combinations of X-ray requests with the same dose
  - Choose the behavior for combinations of X-ray requests with a different dose

This means that a user should be able to select whether they whether the selection pedal is ignored when other pedals 
are pressed, or whether some doses override others, or even whether some projections override others or combine (i.e. 
whether frontal + lateral = biplane).

This does not mean that you pick one of these. The user of your language should be able to choose.

### *Hints*
- You are not developing a (full-fledged) programming language.
- First, create example instances of your languages before creating a grammar.
- Think about the users of your language and how you would explain the dynamic logic to them.

### Introduction
<div style='text-align: justify;'>
Code Block below shows the Modelling Language for an Interventional X-Ray System that provides real-time visual images
based on X-Rays.

    grammar xray.XRayDSL with org.eclipse.xtext.common.Terminals
    
    generate xRayDSL "http://www.XRayDSL.xray"
    SystemType: 'Static_Configuration:' configuration = Static_Configuration
    'Dynamic_Logic:' logic = Dynamic_Logic
    ;
    
    Dynamic_Logic:
    ('Combined_Pedal_Behaviour:' combinedpedal = Behaviours)?
    ('Same_Dose_Behaviour:' samedosebehaviour = Behaviours)?
    ('Different_Dose_Behaviour:' differentdosebehaviour = Behaviours)
    ;
    
    
    Static_Configuration: ThreePedals | SixPedals
    ;
    
    ThreePedals:'ThreePedal1:' pedal1 = Pedal
    'ThreePedal2:' pedal2 = Pedal
    'ThreePedal3:' pedal3 = Pedal
    ;
    SixPedals:  'SixPedal1:' pedal1 = Pedal
    'SixPedal2:' pedal2 = Pedal
    'SixPedal3:' pedal3 = Pedal
    'SixPedal4:' pedal4 = Pedal
    'SixPedal5:' pedal5 = Pedal
    'SixPedal6:' pedal6 = Pedal
    ;
    Pedal: 'Dose:' dose = Dose
    ('Projection:' projection = Projection)?
    'Mode:' mode = Mode
    ;
    
    Behaviours: 'Behaviour:' behaviour= Behaviour
    ;
    
    enum Projection:
    ProjFrontal = 'Frontal' |
    ProjLateral = 'Lateral' |
    ProjBiplane = 'Biplane'
    ;
    
    enum Dose:
    DoseLow = 'Low'   |
    DoseHigh = 'High' |
    DoseUnused = 'Unused'
    ;
    
    enum Mode:
    ModeVideo = 'Video' |
    ModeImage = 'Image'
    ;
    
    enum Behaviour:
    HighCombine = 'high_combine' |
    HighOverride = 'high_override' |
    LowCombine = 'low_combine' |
    LowOverride = 'low_override'
    ;
    
    enum Order:
    RoundRobin = 'round_robin'
    ;
### Modelling Decisions

1. Following the description provided for the modelling language, there exists the Static Configuration and the Dynamic
   Logic. Static Configuration will provide two types, one called ThreePedals and the other is called SixPedals. For
   ThreePedals, the team followed the functional examples section stated in the assignment webpage. Based on the default
   Configuration, both Pedal1 and Pedal2 points to Video Mode and are for low and high dose respectively. Pedal3 is
   indicated as unused.

2. For the SixPedals default configuration, Pedal1, Pedal2, and Pedal3 are meant for Low Doses in Frontal, Lateral, Biplane
   planes respectively. Pedal4 is used to toggle projection when no pedals are pressed. Pedal 5 and Pedal 6 are meant
   for High Doses in Video and Image modes respectively. The same explanation to the user will apply when updating the
   user on the special configurations for ThreePedals and SixPedals.

3. Based on the main.rs template provided, it can be known that Pedal1, Pedal2, and Pedal3 have three parameters, namely
   Projection, Dose, and Mode. Pedal4 mainly controls Projection, whereas Pedal5 and Pedal6 control Dose and Mode.

4. Dynamic Logic will have three categories. The first will be Same Dose Behaviour. The second will be Different Dose
   Behaviour. The third will be Combined Pedal Behaviour. In the Behaviour, there are 4 enumerators. For example,
   HighOverride means when Low Dose Pedals are already pressed i.e. Pedal 1, Pedal2, and Pedal3, as long as Pedal5 and
   Pedal6 are pressed, it can let Dose switch from Low to High in the SixPedals Default Configuration. As for HighCombine,
   it means that when there is High Dose for Frontal and Lateral Planes, it will combine to output High Dose for Biplane.
   The same reasoning goes for LowOverride and LowCombine.

5. Lastly, Order is to indicate the RoundRobin for the Projection to toggle in the SixPedal Configurations.

## DSL Model - Modelling Validation
### *Description*
Add model validation for early feedback to users of your language.

For example, your grammar might allow something for which it would be impossible to generate the correct code. Your 
validator should report this to the user and prevent code generation.

### Introduction

![statechart_1-Plane.png](statechart_1-Plane.png)
<p align="center">Figure 1. Statechart_1-Plane.png</p>

### Modelling Decisions
1. As per the hints given, the team set the goal for the FSM - 1-Plane System to be insightful and not complicated.
   Hence, the team ensured that the number of states is minimal, yet sufficient.

## DSL Model - Code Generation
### *Description*
There is a simple Rust X-ray simulator crate that you should use. See the example at the bottom of this page for an 
implementation that demonstrates how to use the simulator and where to add your own pedal and X-ray action logic.

Create a code generator that generates such Rust files from your DSL. Your DSL should generate a Rust project which 
includes a Cargo.toml and a main.rs file located in src/.

Note: You are only required to do code generation for single-plane systems.
Your DSL is still required to support two-plane systems in the modelling language and validation, but you do not 
have to generate code for two-plane systems.

### *Hints*
- The amount of work for creating the code generator depends very much on the choices you have made for your grammar. 
  Make sure your code generator can at least generate two different regimes for the dynamic logic, and mention if 
  certain language features are not supported.
- First manually develop a piece of Rust code with your preferred code structure, and then copy this into a code 
  generator template and introduce the variation points based on your grammar.
- Validate your generated code by running it with the provided simple Rust-based simulator.

### Introduction
The team followed the hint and used the template main.rs file for modification purposes. Along with the Cargo.toml file,
the team developed a code that implemented the SixPedals Default Configuration. The team ensured the requirement that
"While using low-dose streaming video, surgeons need to be able to temporarily switch to high-dose streaming video." The
team also followed the logic for the SixPedals Default Configuration i.e. High Dose Overrides Low Dose. The team also
followed the requirement where selection pedal toggles only when no pedal is pressed. By following the hint, the team
managed to have a good understanding of the X-Ray Simulator that was provided.

The team then generated a main.rs code for the Single Plane Configurations before using it to develop the code for
XRayDSLGenerator.xtend and RUSTGenerator.xtend code shown below.

"XRayDSLGenerator.xtend"
    /*
    * generated by Xtext 2.33.0
      */
      package xray.generator
    
    import org.eclipse.emf.ecore.resource.Resource
    import org.eclipse.xtext.generator.AbstractGenerator
    import org.eclipse.xtext.generator.IFileSystemAccess2
    import org.eclipse.xtext.generator.IGeneratorContext
    
    /**
     * Generates code from your model files on save.
       *
         * See https://www.eclipse.org/Xtext/documentation/303_runtime_concepts.html#code-generation
         */
    class XRayDSLGenerator extends AbstractGenerator {
    
        override void doGenerate(Resource resource, IFileSystemAccess2 fsa, IGeneratorContext context) {
            val root = resource.allContents.head as Planning;
            if (root !== null) {
                var path = "generated/" + resource.getURI().lastSegment + "/"
                fsa.generateFile(path+"main.rs", RUSTGenerator.toRUST(root))
            if (root !== null) {
                var path = "generated/" + resource.getURI().lastSegment + "/"
                fsa.generateFile(path+"Cargo.toml", RUSTGenerator.toToml(root))
        }
    //		fsa.generateFile('greetings.txt', 'People to greet: ' +
    //			resource.allContents
    //				.filter(Greeting)
    //				.map[name]
    //				.join(', '))
        }
    }
"RUSTGenerator.xtend"

### Modelling Decisions

1.

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Part 2 Assignment 3 — DSL](https://cese.pages.ewi.tudelft.nl/software-systems/part-2/assignments/dsl.html)
</div>