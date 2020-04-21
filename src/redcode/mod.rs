//! Redcode is the assembly language for MARS
//!
//! An [introduction](http://vyznev.net/corewar/guide.html) to Redcode and Core Wars in general can be found on the Core Wars [homepage](https://corewars.org/information.html)

/// Each Redcode instruction contains three parts: the OpCode itself, the source address (a.k.a. the A-field) and the destination address (the B-field).
pub enum Instruction {
    /// data kills the process
    DAT,
    /// move; copies data from one address to another
    MOV,
    /// add; adds one number to another
    ADD,
    /// subtract; subtracts one number from another
    SUB,
    /// multiply; multiplies one number with another
    MUL,
    /// divide; divides one number with another
    DIV,
    /// modulus; divides one number with another and gives the remainder
    MOD,
    /// jump; continues execution from another address
    JMP,
    /// jump if zero; tests a number and jumps to an address if it's 0
    JMZ,
    /// jump if not zero; tests a number and jumps if it isn't 0
    JMN,
    /// decrement and jump if not zero; decrements a number by one, and jumps unless the result is 0
    DJN,
    /// split; start a second process at another address
    SPL,
    /// compare; same as `SEQ`
    CMP,
    /// skip if equal; compares two instructions, and skips the next instruction if they are equal
    SEQ,
    /// skip if not equall; compares two intructions, and skips the next instruction if they aren't equal
    SNE,
    /// skip if lower than; compares two values, and skips the next instruction if the first is lower than the second
    SLT,
    /// load from p-space; loads a number from the private storage space
    LDP,
    /// save to p-space; saves a number to private storage space
    STP,
    /// no operation; does nothing
    NOP,
}
