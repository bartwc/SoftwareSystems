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
and the controller.activateLowVideo will be raised to the value of request.startLowVideo. The ActionLogic block will
initiate a request.startHighVideo when the High-Dose X-Ray pedal is pressed.

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
2. Since it was mentioned in the assignment description that values of the events can be ignored, there needs to be a
   solution for "HighDose" state to either transit to "LowDose" and "ActionLogic" without using the same event transition.
   After consulting with the lecturer, it is permitted for request.startLowVideo to transit from "HighDose" state to
   "LowDose" state while request.stopHighVideo transits "HighDose" state to "ActionLogic" state.

## FSM Model - 2-Plane System
### *Description*
2. FSM model for a 2-plane system.
   * In this model, the value of the events needs to be considered.
   * Use the interface that was given above. You do not need explicit toggle selection requests.

### Introduction
Figure 2 shows the FSM Model for a 2-Plane Interventional X-Ray System that provides real-time visual images based
on X-Rays. As per assignment's description, Figure 2 aims to provide an insightful overview of the states that exist
within the 2-Plane System.

As compared to the 1-Plane System, there are three pedals for Low-Dose X-Ray streaming video. Each pedal represents each
projection, which are "Frontal", "Lateral", and "Biplane" respectively. For readability purposes, "Frontal", "Lateral",
and "Biplane" are labelled with "p1", "p2", and "p3" respectively in the interface request list.

As compared to the 1-Plane System, there are five pedals, 3 "LowDose" and 2 "HighDose", which relate to the 3 Low-Dose 
X-Ray pedals - p1, p2, p3, and 2 High-Dose X-Ray pedals - one for high dose projection and the other for high dose 
streaming video respectively. When the Low-Dose X-Ray pedal is pressed, The ActionLogic block will initiate a 
request.startLowVideo and the controller.activateLowVideo will be raised to the value of request.startLowVideo. The 
ActionLogic block will initiate a request.startHighVideo when the High-Dose X-Ray pedal is pressed.

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
   This was tested more extensively as there exists more states as compared to FSM - 1-Plane System.
2. Given the requirement that the value of the events needs to be considered in FSM - 2-Plane System, the
   "lowDosePedal" variable was created to indicate whether no pedal ("0"), pedal 1 ("1"), pedal 2 ("2"), or pedal 3 ('3") 
   was chosen. Once again, "1", "2", "3" relates to the respective planes "Frontal", "Lateral", and "Biplane". This enables 
   a feature where each of the Low-Dose states can transit to the "HighDose" state. When "stopHighVideo" request is sent, 
   it can transit back to the correct "Low-Dose" state based on the "lowDosePedal" integer value that was last
   recorded.
3. "HighDose" state can also transit either to "LowDose" and "ActionLogic" while ensuring the event transitions are not
   the same. After consulting with the lecturer, it is permitted for request.startLowVideo to transit from "HighDose"
   state to "LowDose" state when "lowDosePedal" integer is not "0" while request.stopHighVideo transits "HighDose" state 
   to "ActionLogic" state when "lowDosePedal" integer is "0".

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Part 2 Assignment 2 — FSM](https://cese.pages.ewi.tudelft.nl/software-systems/part-2/assignments/fsm.html)
</div>