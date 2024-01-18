# FSM (Finite-State Machines)

## How to Import the *itemis CREATE* Projects

1. When starting *itemis CREATE*, select this repository as the workspace directory.
2. Once YAKINDU has launched, Click on `File > Import`.
3. Select `General > Existing Projects Into Workspace` and click `Next`.
4. Choose `Select root directory`.
5. Browse to this directory (the one which has subdirectories `one_plane/` and `two_plane/`).
6. Make sure both projects are selected (checkboxes are checked).
7. Click on `Finish`.
8. You should now be able to open the `Statechart.ysc` files.
   - If you get a pop-up about whether to Copy or Link the files, choose to link.

Project is for Interventional X-Ray System at Philips Healthcare
https://link.springer.com/chapter/10.1007/978-3-642-30729-4_19

### Hints
- Make sure your diagram gives an insightful overview of the system.
- Start by modelling the situation in which, at most, one pedal is pressed at the same time before modelling combinations
of pedal presses.
- The assignment leaves some degrees of freedom in the logic to be modelled. If your model tends to get very complicated,
then please revisit the modelling decisions that you have made.

## FSM Model - 1-Plane System
### *Description*
1. FSM model for a 1-plane system.
   * In this model, the values of the events can be ignored.

### *Requirements*
For each itemis model, hand in the following items:
1. itemis CREATE project (as it is in the repository)
2. Screenshot of graphical statechart
3. Textual modelling decisions (see the important remark in the Introduction)

Upload your models to the 5-fsm directory. Both models have their own subdirectory. Instructions for how to import
the itemis CREATE projects can be found in the README.md file in the repository.

### Introduction
<div style='text-align: justify;'>
Figure 1 shows the FSM Model for a 1-Plane Interventional X-Ray System that provides real-time visual images based 
on X-Rays. As per assignment's description, Figure 1 aims to provide an insightful overview of the states that exist
within the 1-Plane System. 

There are two states, "LowDose" and "HighDose", which relate to the low dose of X-Ray pedal, and a high dose of X-Ray
pedal respectively. When the Low-Dose X-Ray pedal is pressed, The ActionLogic block will initiate a request.startLowVideo
and the controller.activateLowVideo will be raised to the value of request.startLowVideo. Likewise, the ActionLogic 
block will initiate a request.startHighVideo when the High-Dose X-Ray pedal is pressed.

The "LowDose" state can also progress to the "HighDose" state to address the requirement, "While using low-dose streaming
video; surgeons need to be able to temporarily switch to high-dose streaming video, without releasing the low-dose
streaming video pedal."

![statechart_1-Plane.png](statechart_1-Plane.png)
<p align="center">Figure 1. Statechart_1-Plane.png</p>

### Modelling Decisions

1. As per the hints given, the team set the goal for the FSM - 1-Plane System to be insightful and not complicated.
   Hence, the team ensured that the number of states is minimal, yet sufficient. This also took into account the
   requirement that the "LowDose" state can still progress to the "HighDose" state. Transitions from one state to another
   are also carefully designed to ensure transition between states is smooth with no deadlock in a certain state. The
   state machine loop is also designed to be complete for the path to start and return to the start state(ActionLogic). 
   This meant that there were safety design considerations to ensure X-Ray Controller can be deactivated when the operation 
   has finally ended.
2. Since it was mentioned in the assignment description that values of the events can be ignored, there needs to be a
   solution for "HighDose" state to either transit to "LowDose" and "ActionLogic" without using the same event transition.
   After consulting with the lecturer, it is permitted for request.startLowVideo to transit from "HighDose" state to
   "LowDose" state while request.stopHighVideo transits "HighDose" state to "ActionLogic" state.
3. The Transition from "HighDose" state to "ActionLogic" state is to enable a safety feature where the Surgeon can 
   deactivate the controller quickly in case of any emergency while using the X-Ray Controller in "HighDose" state.

## FSM Model - 2-Plane System
### *Description*
2. FSM model for a 2-plane system.
   * In this model, the value of the events needs to be considered.
   * Use the interface that was given above. You do not need explicit toggle selection requests.

### Introduction
Figure 2 shows the FSM Model for a 2-Plane Interventional X-Ray System that provides real-time visual images based
on X-Rays. As per assignment's description, Figure 2 aims to provide an insightful overview of the states that exist
within the 2-Plane System.

As compared to the 1-Plane System, the value of the events is considered and introduced in the 2-Plane System.
Having confirmed with the teaching team, the values "0", "1", and "2" correspond to "Frontal", "Lateral", and "Biplane" 
respectively for both the Low-Dose X-Ray projection and the High-Dose X-Ray projection.

The "LowDose" state can also progress to the "HighDose" state to address the requirement, "While using low-dose streaming
video; surgeons need to be able to temporarily switch to high-dose streaming video, without releasing the low-dose
streaming video pedal."

![statechart_2-Plane.png](statechart_2-Plane.png)
<p align="center">Figure 1. statechart_2-Plane.png</p>

### Modelling Decisions

1. As per the hints given, the team set the goal for the FSM - 2-Plane System to be insightful and not complicated.
   Hence, the team ensured that the number of states is minimal, yet sufficient. This also took into account the
   requirement that the "LowDose" state can still progress to the "HighDose" state. Transitions from one state to another
   are also carefully designed to ensure that transition between states is smooth with no deadlock in a certain state. The
   state machine loop is also designed to be complete for the path to start and return to the start state(ActionLogic).
   This meant that there were safety design considerations to ensure X-Ray Controller can be deactivated when the operation
   has finally ended. FSM - 2-Plane System was tested more extensively as there exist values which were absent in the 
   FSM - 1-Plane System.
2. Given the requirement that the value of the events needs to be considered in FSM - 2-Plane System, the
   variables commented out are indicators for pedal 1 (integer "0"), pedal 2 (integer "1"), or pedal 3 (integer "2") was 
   chosen. Once again, pedal 1, pedal 2, pedal 3 relates to the respective planes "Frontal", "Lateral", and "Biplane" 
   for the "LowDose" projections.
3. In the assignment instructions for the 2-Plane System, the teaching team indicated that "You can assume that the 
   startLowVideo and stopLowVideo requests alternate for each specific projection, similarly for the startHighVideo and 
   stopHighVideo". This was clarified extensively with the TA who informed the team that it is indeed not a toggle back 
   and forth from "LowDose" state to "ActionLogic" state just to alternate projection. Rather, it is a key piece of 
   information to check what happens to the designed state diagram logic flow when various "startLowVideo" events are
   sent continuously with different values. With this key piece of information, the following points below would indicate
   the design flow to achieve the requirement.
4. It must be noted in the design that when "startLowVideo" request is sent, a value larger than "2" cannot be sent as 
   it is an illogical value that does not map to any projection. When "activateLowVideo" integer is set after reaching
   the "LowDose" sate, it can return to "ActionLogic" state with deactivation of the controller, without resetting the 
   value of "activateLowVideo". However, if the Surgeon does not want to send a "stopLowVideo" request, then he can send 
   a "startHighVideo" request, but the value of "startHighVideo" request must be the same value as "activateLowVideo" 
   controller. This represents the Surgeon made a concerted move to increase the X-Ray dose from "LowDose" to "HighDose"
   for the particular plane.
5. In any case, when the Surgeon wants to change the plane during "HighDose", the Surgeon can send a "startLowVideo" 
   request with the newly chosen number. This means the value "activateHighVideo" would not reset while transiting to
   "LowDose" state with the newly chosen plane. The Surgeon can then send a "StartHighVideo" request to increase the
   X-Ray dosage for the newly chosen plane.
6. "HighDose" state can also transit either to "LowDose" and "ActionLogic" while ensuring the event transitions are not
   the same. After consulting with the lecturer, it is permitted for request.startLowVideo to transit from "HighDose"
   state to "LowDose" state while request.stopHighVideo transits "HighDose" state to "ActionLogic" state. 
7. The Transition from "HighDose" state to "ActionLogic" state is to enable a safety feature where the Surgeon can
   deactivate the controller quickly in case of any emergency while using the X-Ray Controller in "HighDose" state.

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Part 2 Assignment 2 — FSM](https://cese.pages.ewi.tudelft.nl/software-systems/part-2/assignments/fsm.html)
</div>