pub type Character = [[bool; 8]; 16];

//       TODO_(completed):
//       this is one way to get a simple font in your program.
//       finish this if you want to use it properly. Also add
//       functions to screen.rs to draw a `Character`
pub const ZERO: Character = [
    [false, true, true, true, true, true, true, false],
    [true, true, true, true, true, true, true, true],
    [true, true, true, false, false, true, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, true, false, false, true, true, true],
    [true, true, true, true, true, true, true, true],
    [false, true, true, true, true, true, true, false],
];

pub const UN: Character = [
    [false, false, false, true, true, false, false, false],
    [false, false, true, true, true, false, false, false],
    [false, true, true, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, false, true, true, false, false, false],
    [false, false, true, true, true, true, false, false],
    [false, false, true, true, true, true, false, false],
];

pub const DEUX: Character = [
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
];

pub const TROIS: Character = [
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
];

pub const QUATRE: Character = [
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, true],
    [false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
];

pub const CINQ: Character = [
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
];

pub const SIX: Character = [
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [true, true, false, false, false, false, false, false],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
];

pub const SEPT: Character = [
    [true, true, true, true, true, true, true, true],
    [true, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false],
    [false, false, false, false, false, true, true, false]
];

pub const HUIT: Character = [
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
];

pub const NEUF: Character = [
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [true, true, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true, true, true, true, true, true, false],
    [false, true, true, true, true, true, true, false],
];

pub const NUMBERS: [Character; 10] = [
    ZERO,
    // TODO_(completed)
    UN,     // ONE
    DEUX,   // TWO
    TROIS,  // THREE
    QUATRE, // FOUR
    CINQ,   // FIVE
    SIX,    // SIX
    SEPT,   // SEVEN
    HUIT,   // EIGHT
    NEUF,   // NINE
];
