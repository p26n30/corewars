//! Redcode is the assembly language for MARS
//!
//! An [introduction](http://vyznev.net/corewar/guide.html) to Redcode and Core Wars in general can be found on the Core Wars [homepage](https://corewars.org/information.html)

/// Each Redcode instruction contains the following parts:
///
/// * the OpCode itself
/// * the source address (a.k.a. the A-field)
/// * the destination address (the B-field)
/// * a modifier
pub struct Instruction {
    op_code: Opcode,
    source: Address,
    destination: Address,
    modifier: Modifier,
}

/// Each Instruction can perform on of these operations.
pub enum Opcode {
    /// data; kills the process
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

struct Address {
    value: u16,
    mode: Mode,
}

/// The address mode determines how the address value is used.
pub enum Mode {
    /// signified by `#`.
    Immediate,
    /// signified by `$`, which is optional.
    Direct,
    /// signified by `*`.
    AFieldIndirect,
    /// signified by `@`.
    BFieldIndirect,
    /// signified by `{`.
    AFieldIndirectWithPredecrement,
    /// signified by `<`.
    BFieldIndirectWithPredecrement,
    /// signified by `}`.
    AFieldIndirectWithPostincrement,
    /// signified by `>`.
    BFieldIndirectWithPostincrement,
}

/// A modifier is a suffix that is added to the instruction to specify which parts of the source and the destination it will affect.
pub enum Modifier {
    /// moves the A-field of the source into the A-field of the destination
    A,
    /// moves the B-field of the source into the B-field of the destination
    B,
    /// moves the A-field of the source into the B-field of the destination
    AB,
    /// moves the B-field of the source into the A-field of the destination
    BA,
    /// moves both fields of the source into the same fields in the destination
    F,
    /// moves both fields of the source into the opposite fields in the destination
    X,
    /// moves the whole source instruction into the destination
    I,
}
