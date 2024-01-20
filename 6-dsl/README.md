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
Figure 1 shows the FSM Model for a 1-Plane Interventional X-Ray System that provides real-time visual images based 
on X-Rays. As per assignment's description, Figure 1 aims to provide an insightful overview of the states that exist
within the 1-Plane System.

![statechart_1-Plane.png](statechart_1-Plane.png)
<p align="center">Figure 1. Statechart_1-Plane.png</p>

### Modelling Decisions

1. As per the hints given, the team set the goal for the FSM - 1-Plane System to be insightful and not complicated.
   Hence, the team ensured that the number of states is minimal, yet sufficient.

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

## Authors
[@Zhengtao Huang (5833469, zhengtaohuang)]()<br>
[@Barry Tee Wei Cong (5662834, btee)]()

## Acknowledgments
* [Part 2 Assignment 3 — DSL](https://cese.pages.ewi.tudelft.nl/software-systems/part-2/assignments/dsl.html)
</div>