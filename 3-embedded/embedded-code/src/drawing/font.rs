pub type Character = [[bool; 8]; 16];

//       TODO_(completed):
//       this is one way to get a simple font in your program.
//       finish this if you want to use it properly. Also add
//       functions to screen.rs to draw a `Character`
pub const ZERO: Character = [
    [false, true, true,  true,  true,  true,  true, false],
    [true,  true, true,  true,  true,  true,  true, true],
    [true,  true, true,  false, false, true,  true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, true,  false, false, true,  true, true],
    [true,  true, true,  true,  true,  true,  true, true],
    [false, true, true,  true,  true,  true,  true, false],
];

pub const UN: Character = [ // ONE
    [false, false, false, true, true, false, false, false],
    [false, false, true,  true, true, false, false, false],
    [false, true,  true,  true, true, false, false, false],
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
    [false, false, true,  true, true, true,  false, false],
    [false, false, true,  true, true, true,  false, false],
];

pub const DEUX: Character = [ // TWO
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, true,  true,  true,  true,  true,  true,  false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, true,  true,  true,  true,  true,  true,  false],
];

pub const TROIS: Character = [ // THREE
    [false, true,  true,  true,  true,  true,  true, false],
    [false, true,  true,  true,  true,  true,  true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true,  true,  true,  true,  true,  true, false],
    [false, true,  true,  true,  true,  true,  true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true,  true,  true,  true,  true,  true, false],
    [false, true,  true,  true,  true,  true,  true, false],
];

pub const QUATRE: Character = [ // FOUR
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [false, true,  true,  true,  true,  true,  true, true],
    [false, true,  true,  true,  true,  true,  true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
];

pub const CINQ: Character = [ // FIVE
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, true,  true,  true,  true,  true,  true,  false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [true,  true,  false, false, false, false, false, false],
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, false, false, false, false, false, true,  true],
    [false, true,  true,  true,  true,  true,  true,  false],
    [false, true,  true,  true,  true,  true,  true,  false],
];

pub const SIX: Character = [ // SIX
    [false, true, true,  true,  true,  true,  true,  false],
    [false, true, true,  true,  true,  true,  true,  false],
    [true,  true, false, false, false, false, false, false],
    [true,  true, false, false, false, false, false, false],
    [true,  true, false, false, false, false, false, false],
    [true,  true, false, false, false, false, false, false],
    [false, true, true,  true,  true,  true,  true,  false],
    [false, true, true,  true,  true,  true,  true,  false],
    [true,  true, false, false, false, false, true,  true],
    [true,  true, false, false, false, false, true,  true],
    [true,  true, false, false, false, false, true,  true],
    [true,  true, false, false, false, false, true,  true],
    [true,  true, false, false, false, false, true,  true],
    [true,  true, false, false, false, false, true,  true],
    [false, true, true,  true,  true,  true,  true,  false],
    [false, true, true,  true,  true,  true,  true,  false],
];

pub const SEPT: Character = [ // SEVEN
    [true,  true,  true,  true,  true,  true,  true, true],
    [true,  true,  true,  true,  true,  true,  true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false],
    [false, false, false, false, false, true,  true, false]
];

pub const HUIT: Character = [ // EIGHT
    [false, true, true,  true,  true,  true,  true, false],
    [false, true, true,  true,  true,  true,  true, false],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [false, true, true,  true,  true,  true,  true, false],
    [false, true, true,  true,  true,  true,  true, false],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [true,  true, false, false, false, false, true, true],
    [false, true, true,  true,  true,  true,  true, false],
    [false, true, true,  true,  true,  true,  true, false],
];

pub const NEUF: Character = [ // NINE
    [false, true,  true,  true,  true,  true, true, false],
    [false, true,  true,  true,  true,  true, true, false],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [true,  true,  false, false, false, false, true, true],
    [false, true,  true,  true,  true,  true, true, false],
    [false, true,  true,  true,  true,  true, true, false],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, false, false, false, false, false, true, true],
    [false, true,  true,  true,  true,  true,  true, false],
    [false, true,  true,  true,  true,  true,  true, false],
];

pub const COMMA: Character = [ // COMMA
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false, false],
    [false, false, false, false, false, true,  false, false],
    [false, false, true,  true,  true,  true,  false, false],
    [false, false, true,  true,  true,  true,  false, false],
    [false, false, true,  true,  true,  true,  false, false],
    [false, false, true,  true,  true,  true,  false, false],
    [false, true,  true,  true,  true,  true,  false, false],
    [false, true,  true,  true,  true,  false, false, false],
    [true,  true,  true,  true,  false, true,  false, false]
];

pub const NUMBERS: [Character; 11] = [
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
    COMMA, // COMMA
];
