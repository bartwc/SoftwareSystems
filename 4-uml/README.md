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
### Description 
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
Figure 1 shows the component diagram of an Interventional X-Ray System that provides real-time visual images based 
on X-Rays. As per assignment's description, it shows the hardware, electronic and software components of 1 X-Ray Plane. 
All hardware components except the Tablet are grouped and aligned at the bottom section of the diagram. This is in 
accordance with the classification of the Outside Perspective. The grouping of the hardware at the bottom section 
provides a clear and instant overview of most hardware the surgeon is required to physically see, understand, and 
use during the operation. The exclusion of the tablet hardware being located at the top section is due to its link 
with the system configuration branch.

![component.png](component.png)
<p align="center">Figure 1 - component.png</p>

### Modelling Decisions
1. As per the hint given, the team followed it by introducing packages to group two types of components. The first
package is the Pedal Board with 3 different pedal options to control the system. This grouping is necessary to indicate 
the options provided for the surgeon to step on. It is also necessary to show the option to step on 1 or 2 of the 
pedals will convert the physical step(s) into send the respective signal(s) into the Pedal Mapper. The unused pedal is
shown but not connected with the Pedal Step Interface due to it being outside the scope of this assignment.

2. The second grouping of the X-Ray Tube and X-Ray Detector with their respective interface is necessary to represent
the package of the X-Ray Plane. It is logically declared as one item by itself in the hardware components list written 
in the assignment. This package is essential to convey the idea on how the X-Ray Plane operates. When the Table is empty,
the Table allows the X-Ray wave to transverse and pass through from the Tube to the Detector and provides a baseline 
X-Ray Data result. However, when the patient is placed on the Table, the X-Ray from the X-Ray Tube undergoes attenuation 
through the human body medium, and gets absorbed by the X-Ray Detector. This will also provide vital high-level system 
understanding to the software designer. This is because, while writing the software with no patient placed on the Table, 
the software designer knows the incoming data collected by the Detector must be consistently correct. Only then can the 
software designer have some level of assurance to begin testing when a sample patient is placed on the Table to collect 
X-Ray results.

3. The team made a decision for the model to be horizontally aligned. Starting from the left in the top section, the
software designer can understand the need for the Database to load all system configurations and component settings
for the surgeon to select. After selecting the necessary configurations and settings, they will be loaded to the X-Ray
Controller and Image Processor. Only then the surgeon can proceed to the Pedal Board area and conduct the surgery while 
stepping on the necessary pedals he needs. Upon stepping on the pedal, it will trigger the logic flow of the X-Ray
system from left to right. The left-to-right logic flow for the configuration branch is modeled to assist the reader 
to understand the Sequence-Configuration Diagram. The left-to-right logic flow for the main branch is modeled to assist
the reader to understand the Sequence-Pedal-1 Diagram and the Sequence-Pedal-2 Diagram.

4. The grouping of the hardware at the bottom section is for personnel surrounding the surgeon or X-Ray System to be 
aware. They are to not obstruct the Pedal Board, the space between the X-Ray Tube and Detector, and the visual view of 
the Screen during software testing or actual surgery. This is especially so when a high dose of X-Ray can be harmful 
unknowingly during long periods.

![sequence-config.png](sequence-config.png)

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Assignment 3 - Embedded Systems](https://cese.pages.ewi.tudelft.nl/software-systems/part-1/assignments/es.html)
