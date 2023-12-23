# UML
Your PlantUML diagrams should follow this naming scheme:
- Component diagram = `component.puml`
- Sequence diagrams
    - Configuration = `sequence-config.puml`
    - First pedal diagram = `sequence-pedal-1.puml`
    - Second pedal diagram = `sequence-pedal-2.puml`
- Class diagrams
    - Database = `class-database.puml`
    - High-level = `class-high-level.puml`
Exported images are named the same as the diagrams (expect for the file extension).
The report below is a Markdown document that covers the team's modeling decisions for each diagram.

## Component Diagram
### *Description* 
Use PlantUML to create 1 component diagram that shows the hardware and software/electronic components for a system 
with 1 plane. In addition, introduce associations with required and provided interfaces between the components. 
Hint: It may be useful to introduce packages to group some components.*

### *Requirements*
* Assembly notation was used (instead of dependency).
* The diagram uses UML correctly.
* Each interface is provided by exactly one component.
* The correct relation types are used.
* The diagram models the system (up to modeling decisions).

### Introduction
Figure 1 shows the Component Diagram of an Interventional X-Ray System that provides real-time visual images based 
on X-Rays. As per assignment's description, it shows the hardware, electronic and software components of 1 X-Ray Plane. 
All hardware components except the Tablet are grouped and aligned at the bottom section of the diagram. This is in 
accordance with the classification of the Outside Perspective. The grouping of the hardware at the bottom section 
provides a clear and instant overview of most hardware the Surgeon is required to physically see, understand, and 
use during the operation. The exclusion of the tablet hardware being located at the top section is due to its link 
with the system configuration branch.

![component.png](component.png)
<p align="center">Figure 1. component.png</p>

### Modelling Decisions
1. As per the hint given, the team followed it by introducing packages to group two types of components. The first
package is the Pedal Board with 3 different pedal options to control the system. This grouping is necessary to indicate 
the options provided for the Surgeon to step on. It is also necessary to show the option to step on 1 or 2 of the 
pedals will convert the physical step(s) into send the respective signal(s) into the Pedal Mapper. The unused pedal is
shown but not connected with the Pedal Step Interface due to it being outside the scope of this assignment.

2. The second grouping of the X-Ray Tube and X-Ray Detector with their respective interface is necessary to represent
the package of the X-Ray Plane. It is logically declared as one item by itself in the hardware components list written 
in the assignment. This package is essential to convey the idea on how the X-Ray Plane operates. When the Table is empty,
the Table allows the X-Ray wave to transverse and pass through from the Tube to the Detector and provides a baseline 
X-Ray Data result. However, when the Patient is placed on the Table, the X-Ray from the X-Ray Tube undergoes attenuation 
through the human body medium, and gets absorbed by the X-Ray Detector. This will also provide vital high-level system 
understanding to the software designer. This is because, while writing the software with no Patient placed on the Table, 
the software designer knows the incoming data collected by the Detector must be consistently correct. Only then can the 
software designer have some level of assurance to begin testing when a sample Patient is placed on the Table to collect 
X-Ray results.

3. The team made a decision for the model to be horizontally aligned. Starting from the left in the top section, the
software designer can understand the need for the Database to load all system configurations and component settings
for the Surgeon to select. After selecting the necessary configurations and settings, they will be loaded to the X-Ray
Controller and Image Processor. Only then the Surgeon can proceed to the Pedal Board area and conduct the surgery while 
stepping on the necessary pedals he needs. Upon stepping on the pedal, it will trigger the logic flow of the X-Ray
system from left to right. The left-to-right logic flow for the configuration branch is modeled to assist the reader 
to understand the Sequence-Configuration Diagram. The left-to-right logic flow for the main branch is modeled to assist
the reader to understand the Sequence-Pedal-1 Diagram and the Sequence-Pedal-2 Diagram.

4. The grouping of the hardware at the bottom section is for personnel surrounding the Surgeon or X-Ray System to be 
aware. They are to not obstruct the Pedal Board, the space between the X-Ray Tube and Detector, and the visual view of 
the Screen during software testing or actual surgery. This is especially so when a high dose of X-Ray from the X-Ray 
Tube can be unknowingly harmful during long periods.

## Sequence Diagram for Configuration
Use PlantUML to create 3 sequence diagrams for the following workflows in a system with 1 plane:

### *Description*
Before a medical procedure:
1. Show the supported medical procedures to the Surgeon.
2. Configure the system for a specific medical procedure that is selected by the Surgeon.

### *Requirements*
* The interactions match your component diagram.
* The use of synchronous and asynchronous events is correct.
* The XrayController receives an alternation of activate and deactivate events.
* The XrayController decouples the receiving of activate and deactivate events from the sending of timed pulses.
* The diagrams model the correct scenarios (up to modeling decisions).

### Introduction
Figure 2 shows the Sequence Configuration Diagram of an Interventional X-Ray System. It illustrates the sequence of 
interactions in an Interventional X-Ray System's configuration process. The primary actor, the Surgeon, initiates the 
sequence by accessing the Tablet to configure system settings. The Tablet acts as the interface between the Surgeon 
and the underlying system components.

The Surgeon accesses the Tablet, navigating to the settings page. The Tablet, in turn, retrieves and displays all 
system configurations and component settings from the database. Upon configuration completion, the Surgeon receives 
confirmation and returns to the settings page. 

The selected system configurations and medical procedure details are communicated from the Tablet to the System 
Configurator. The System Configurator initiates configuration commands to the X-Ray Controller, which orchestrates 
commands to the X-Ray Tube, X-Ray Detector, and Image Processor. Each component responds with the state of activation 
or deactivation. The System Configurator also interacts with the Image Processor for configuration commands and 
status updates. The sequence diagram provides a comprehensive overview of the configuration workflow, showcasing 
the seamless communication between the Surgeon, Tablet, System Configurator, and the underlying X-Ray System components.

![sequence-config.png](sequence-config.png)
<p align="center">Figure 2. sequence-config.png</p>

### Modelling Decisions
1. The Surgeon actor remains in an activated state throughout the entire surgery operation. This corresponds to the 
Surgeon's continuous engagement in the operation.
2. The Tablet remains activated as long as key components (System Configurator, X-Ray Controller, X-Ray Tube, X-Ray 
Detector, and Image Processor) are active. The activation of the Tablet follows the First-In, Last-Out concept, serving 
as a safety feature. The brightly-lit screen indicates the ongoing operation of the X-Ray System, enhancing awareness 
alertness.
3. The process of obtaining and selecting settings for the medical procedure is designed to be quick and accurate. This 
decision aims to guide the software designer in creating a user-friendly interface for the Surgeon, ensuring no mistakes 
in configuration before the surgery. The emphasis is on preventing technical difficulties, allowing the Surgeon to focus
on patient care without distraction.
4. System Configurator, X-Ray Controller, X-Ray Tube, X-Ray Detector, and Image Processor provide feedback signals 
after Activation/Deactivation Commands. This decision facilitates immediate detection of failures, prompting timely 
repairs or inspections during regular maintenance. It aims to prevent breakdowns during critical surgery operations.
5. Messages exchanged between System Configurator, X-Ray Controller, X-Ray Tube, X-Ray Detector, and Image Processor 
are asynchronous. This decision acknowledges that the sequence of actions for these components may not be predetermined, 
allowing for flexibility in their interactions.

## Sequence Diagram for Pedal Scenario 1
### *Description*
During a medical procedure:
1. Pressing the pedal for low-dose video.
2. Releasing the pedal for low-dose video.

You must include the X-ray tube, X-ray detector, ImageProcessor, and Screen components in this particular scenario. 
Include pedal actions and the displaying images on the screen.

### Introduction
Figure 3 shows the Sequence Pedal Scenario 1 Diagram of an Interventional X-Ray System. It illustrates the sequence 
of interactions in an Interventional X-Ray System's process when the pedal for low-dose X-Ray is stepped on. This stage 
is based after configuration of the X-Ray system. This diagram captures the dynamic flow of events initiated by the 
Surgeon's action of stepping on the pedal, leading to the activation of the X-Ray components, continuous emission of 
low-dose X-Ray, and the subsequent deactivation process. 

![sequence-pedal-1.png](sequence-pedal-1.png)
<p align="center">Figure 3. sequence-pedal-1.png</p>

### Modelling Decisions
1. Although the Screen does not follow the First-In concept as the Tablet, it follows the Last-Out concept. This is a 
safety feature that ensures X-Ray Tube and X-Ray Detector are safely deactivated before the Screen is deactivated. As
long as the Screen is not deactivated, it can display pixels of the Readable Image or Return Activation State message  
to indicate that the X-Ray Tube and X-Ray Detector are still live.
2. The continuous emission of low-dose X-Ray happens because of a loop that executes multiple times as the pedal is 
continuously pressed and not released. The surgeon needs to take note of the relatively short lifeline of the Table 
as it indicates that the Patient is undergoing continuous X-Ray radiation in the loop when the pedal has not been
released by the surgeon.

## Sequence Diagram for Pedal Scenario 2
### *Description*
During a medical procedure:
1. Pressing the pedal for low-dose video.
2. Releasing the pedal for low-dose video.

You must include the X-ray tube, X-ray detector, ImageProcessor, and Screen components in this particular scenario.
Include pedal actions and the displaying images on the screen.

### Introduction
Figure 4 shows the Sequence Pedal Scenario 2 Diagram of an Interventional X-Ray System. It illustrates the sequence
of interactions in an Interventional X-Ray System's process when the pedal for low-dose X-Ray and high-dose X-Ray are
stepped on. XXX - More Information Needed

![sequence-pedal-2.png](sequence-pedal-2.png)
<p align="center">Figure 4. sequence-pedal-2.png</p>

### Modelling Decisions
1. XXX - More Information Needed

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Part 2 Assignment 1 - UML](https://cese.pages.ewi.tudelft.nl/software-systems/part-2/assignments/uml.html)
