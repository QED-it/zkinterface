// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(unused_imports, dead_code)]
pub mod zkinterface {

  use std::mem;
  use std::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Message {
  NONE = 0,
  Circuit = 1,
  ConstraintSystem = 2,
  Witness = 3,
  Command = 4,

}

pub const ENUM_MIN_MESSAGE: u8 = 0;
pub const ENUM_MAX_MESSAGE: u8 = 4;

impl<'a> flatbuffers::Follow<'a> for Message {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Message {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Message;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Message;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Message {
    type Output = Message;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Message>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_MESSAGE:[Message; 5] = [
  Message::NONE,
  Message::Circuit,
  Message::ConstraintSystem,
  Message::Witness,
  Message::Command
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_MESSAGE:[&'static str; 5] = [
    "NONE",
    "Circuit",
    "ConstraintSystem",
    "Witness",
    "Command"
];

pub fn enum_name_message(e: Message) -> &'static str {
  let index = e as u8;
  ENUM_NAMES_MESSAGE[index as usize]
}

pub struct MessageUnionTableOffset {}
#[allow(non_camel_case_types)]
#[repr(i8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ConstraintType {
  R1CS = 0,
  arithmetic = 1,

}

pub const ENUM_MIN_CONSTRAINT_TYPE: i8 = 0;
pub const ENUM_MAX_CONSTRAINT_TYPE: i8 = 1;

impl<'a> flatbuffers::Follow<'a> for ConstraintType {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for ConstraintType {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = i8::to_le(self as i8);
    let p = &n as *const i8 as *const ConstraintType;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = i8::from_le(self as i8);
    let p = &n as *const i8 as *const ConstraintType;
    unsafe { *p }
  }
}

impl flatbuffers::Push for ConstraintType {
    type Output = ConstraintType;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<ConstraintType>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CONSTRAINT_TYPE:[ConstraintType; 2] = [
  ConstraintType::R1CS,
  ConstraintType::arithmetic
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_CONSTRAINT_TYPE:[&'static str; 2] = [
    "R1CS",
    "arithmetic"
];

pub fn enum_name_constraint_type(e: ConstraintType) -> &'static str {
  let index = e as i8;
  ENUM_NAMES_CONSTRAINT_TYPE[index as usize]
}

pub enum CircuitOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// A description of a circuit or sub-circuit.
/// This can be a complete circuit ready for proving,
/// or a part of a circuit being built.
pub struct Circuit<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Circuit<'a> {
    type Inner = Circuit<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Circuit<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Circuit {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CircuitArgs<'args>) -> flatbuffers::WIPOffset<Circuit<'bldr>> {
      let mut builder = CircuitBuilder::new(_fbb);
      builder.add_free_variable_id(args.free_variable_id);
      if let Some(x) = args.configuration { builder.add_configuration(x); }
      if let Some(x) = args.field_maximum { builder.add_field_maximum(x); }
      if let Some(x) = args.connections { builder.add_connections(x); }
      builder.finish()
    }

    pub const VT_CONNECTIONS: flatbuffers::VOffsetT = 4;
    pub const VT_FREE_VARIABLE_ID: flatbuffers::VOffsetT = 6;
    pub const VT_FIELD_MAXIMUM: flatbuffers::VOffsetT = 8;
    pub const VT_CONFIGURATION: flatbuffers::VOffsetT = 10;

  /// Variables to use as connections to the sub-circuit.
  ///
  /// - Variables to use as input connections to the gadget.
  /// - Or variables to use as output connections from the gadget.
  /// - Variables are allocated by the sender of this message.
  /// - The same structure must be provided for R1CS and witness generations.
  /// - If using `Command.witness_generation`, variables must be assigned values.
  #[inline]
  pub fn connections(&self) -> Option<Variables<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Variables<'a>>>(Circuit::VT_CONNECTIONS, None)
  }
  /// A variable ID greater than all IDs allocated by the sender of this message.
  /// The recipient of this message can allocate new IDs >= free_variable_id.
  #[inline]
  pub fn free_variable_id(&self) -> u64 {
    self._tab.get::<u64>(Circuit::VT_FREE_VARIABLE_ID, Some(0)).unwrap()
  }
  /// The largest element of the finite field used by the current system.
  /// A canonical little-endian representation of the field order minus one.
  /// See `Variables.values` below.
  #[inline]
  pub fn field_maximum(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Circuit::VT_FIELD_MAXIMUM, None).map(|v| v.safe_slice())
  }
  /// Optional: Any custom parameter that may influence the circuit construction.
  ///
  /// Example: function_name, if a gadget supports multiple function variants.
  /// Example: the depth of a Merkle tree.
  /// Counter-example: a Merkle path is not config and belongs in `connections.info`.
  #[inline]
  pub fn configuration(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>(Circuit::VT_CONFIGURATION, None)
  }
}

pub struct CircuitArgs<'a> {
    pub connections: Option<flatbuffers::WIPOffset<Variables<'a >>>,
    pub free_variable_id: u64,
    pub field_maximum: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
    pub configuration: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<KeyValue<'a >>>>>,
}
impl<'a> Default for CircuitArgs<'a> {
    #[inline]
    fn default() -> Self {
        CircuitArgs {
            connections: None,
            free_variable_id: 0,
            field_maximum: None,
            configuration: None,
        }
    }
}
pub struct CircuitBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CircuitBuilder<'a, 'b> {
  #[inline]
  pub fn add_connections(&mut self, connections: flatbuffers::WIPOffset<Variables<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Variables>>(Circuit::VT_CONNECTIONS, connections);
  }
  #[inline]
  pub fn add_free_variable_id(&mut self, free_variable_id: u64) {
    self.fbb_.push_slot::<u64>(Circuit::VT_FREE_VARIABLE_ID, free_variable_id, 0);
  }
  #[inline]
  pub fn add_field_maximum(&mut self, field_maximum: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Circuit::VT_FIELD_MAXIMUM, field_maximum);
  }
  #[inline]
  pub fn add_configuration(&mut self, configuration: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Circuit::VT_CONFIGURATION, configuration);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CircuitBuilder<'a, 'b> {
    let start = _fbb.start_table();
    CircuitBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Circuit<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum ConstraintSystemOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// ConstraintSystem represents constraints to be added to the constraint system.
///
/// Multiple such messages are equivalent to the concatenation of `constraints` arrays.
pub struct ConstraintSystem<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for ConstraintSystem<'a> {
    type Inner = ConstraintSystem<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> ConstraintSystem<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        ConstraintSystem {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args ConstraintSystemArgs<'args>) -> flatbuffers::WIPOffset<ConstraintSystem<'bldr>> {
      let mut builder = ConstraintSystemBuilder::new(_fbb);
      if let Some(x) = args.info { builder.add_info(x); }
      if let Some(x) = args.constraints { builder.add_constraints(x); }
      builder.add_constraint_type(args.constraint_type);
      builder.finish()
    }

    pub const VT_CONSTRAINTS: flatbuffers::VOffsetT = 4;
    pub const VT_CONSTRAINT_TYPE: flatbuffers::VOffsetT = 6;
    pub const VT_INFO: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn constraints(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<BilinearConstraint<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<BilinearConstraint<'a>>>>>(ConstraintSystem::VT_CONSTRAINTS, None)
  }
  /// Whether this is an R1CS or fan-in-2 arithmetic circuit.
  /// A special case is a boolean circuit with XOR and AND gates,
  /// then constraint_type == arithmetic and circuit.field_maximum == 1.
  #[inline]
  pub fn constraint_type(&self) -> ConstraintType {
    self._tab.get::<ConstraintType>(ConstraintSystem::VT_CONSTRAINT_TYPE, Some(ConstraintType::R1CS)).unwrap()
  }
  /// Optional: Any complementary info that may be useful.
  ///
  /// Example: human-readable descriptions.
  /// Example: custom hints to an optimizer or analyzer.
  #[inline]
  pub fn info(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>(ConstraintSystem::VT_INFO, None)
  }
}

pub struct ConstraintSystemArgs<'a> {
    pub constraints: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<BilinearConstraint<'a >>>>>,
    pub constraint_type: ConstraintType,
    pub info: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<KeyValue<'a >>>>>,
}
impl<'a> Default for ConstraintSystemArgs<'a> {
    #[inline]
    fn default() -> Self {
        ConstraintSystemArgs {
            constraints: None,
            constraint_type: ConstraintType::R1CS,
            info: None,
        }
    }
}
pub struct ConstraintSystemBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> ConstraintSystemBuilder<'a, 'b> {
  #[inline]
  pub fn add_constraints(&mut self, constraints: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<BilinearConstraint<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ConstraintSystem::VT_CONSTRAINTS, constraints);
  }
  #[inline]
  pub fn add_constraint_type(&mut self, constraint_type: ConstraintType) {
    self.fbb_.push_slot::<ConstraintType>(ConstraintSystem::VT_CONSTRAINT_TYPE, constraint_type, ConstraintType::R1CS);
  }
  #[inline]
  pub fn add_info(&mut self, info: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(ConstraintSystem::VT_INFO, info);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> ConstraintSystemBuilder<'a, 'b> {
    let start = _fbb.start_table();
    ConstraintSystemBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<ConstraintSystem<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum WitnessOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// Witness represents an assignment of values to variables.
///
/// - Does not include variables already given in `Circuit.connections`.
/// - Does not include the constant one variable.
/// - Multiple such messages are equivalent to the concatenation of `Variables` arrays.
pub struct Witness<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Witness<'a> {
    type Inner = Witness<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Witness<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Witness {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args WitnessArgs<'args>) -> flatbuffers::WIPOffset<Witness<'bldr>> {
      let mut builder = WitnessBuilder::new(_fbb);
      if let Some(x) = args.assigned_variables { builder.add_assigned_variables(x); }
      builder.finish()
    }

    pub const VT_ASSIGNED_VARIABLES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn assigned_variables(&self) -> Option<Variables<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Variables<'a>>>(Witness::VT_ASSIGNED_VARIABLES, None)
  }
}

pub struct WitnessArgs<'a> {
    pub assigned_variables: Option<flatbuffers::WIPOffset<Variables<'a >>>,
}
impl<'a> Default for WitnessArgs<'a> {
    #[inline]
    fn default() -> Self {
        WitnessArgs {
            assigned_variables: None,
        }
    }
}
pub struct WitnessBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> WitnessBuilder<'a, 'b> {
  #[inline]
  pub fn add_assigned_variables(&mut self, assigned_variables: flatbuffers::WIPOffset<Variables<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Variables>>(Witness::VT_ASSIGNED_VARIABLES, assigned_variables);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> WitnessBuilder<'a, 'b> {
    let start = _fbb.start_table();
    WitnessBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Witness<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum CommandOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// Optional: Command messages can be used to request actions from the receiver. This makes it
/// possible to write code that works in different environments. Commands and parameters
/// can be passed over the same byte stream as other messages; if so Command must be the first
/// message. This reduces the need for environment-specific methods (it can replace CLI --flags, etc).
pub struct Command<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Command<'a> {
    type Inner = Command<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Command<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Command {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args CommandArgs<'args>) -> flatbuffers::WIPOffset<Command<'bldr>> {
      let mut builder = CommandBuilder::new(_fbb);
      if let Some(x) = args.parameters { builder.add_parameters(x); }
      builder.add_witness_generation(args.witness_generation);
      builder.add_constraints_generation(args.constraints_generation);
      builder.finish()
    }

    pub const VT_CONSTRAINTS_GENERATION: flatbuffers::VOffsetT = 4;
    pub const VT_WITNESS_GENERATION: flatbuffers::VOffsetT = 6;
    pub const VT_PARAMETERS: flatbuffers::VOffsetT = 8;

  /// For gadget flows.
  /// Request the generation of a constraint system (or part thereof).
  /// If true, this must be followed by a Circuit.
  /// The response must be another Circuit message with a greater `free_variable_id`
  /// followed by one or more ConstraintSystem messages.
  #[inline]
  pub fn constraints_generation(&self) -> bool {
    self._tab.get::<bool>(Command::VT_CONSTRAINTS_GENERATION, Some(false)).unwrap()
  }
  /// For gadget flows.
  /// Request the generation of a witness (or part thereof).
  /// If true, this must be followed by a Circuit, and the `connections`
  /// variables must contain input values.
  /// The response must be another Circuit message, with a greater `free_variable_id`,
  /// with output values in `connections` variables, followed by one or more `Witness` messages.
  #[inline]
  pub fn witness_generation(&self) -> bool {
    self._tab.get::<bool>(Command::VT_WITNESS_GENERATION, Some(false)).unwrap()
  }
  /// Optional: Any complementary parameter that may be useful.
  #[inline]
  pub fn parameters(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>(Command::VT_PARAMETERS, None)
  }
}

pub struct CommandArgs<'a> {
    pub constraints_generation: bool,
    pub witness_generation: bool,
    pub parameters: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<KeyValue<'a >>>>>,
}
impl<'a> Default for CommandArgs<'a> {
    #[inline]
    fn default() -> Self {
        CommandArgs {
            constraints_generation: false,
            witness_generation: false,
            parameters: None,
        }
    }
}
pub struct CommandBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> CommandBuilder<'a, 'b> {
  #[inline]
  pub fn add_constraints_generation(&mut self, constraints_generation: bool) {
    self.fbb_.push_slot::<bool>(Command::VT_CONSTRAINTS_GENERATION, constraints_generation, false);
  }
  #[inline]
  pub fn add_witness_generation(&mut self, witness_generation: bool) {
    self.fbb_.push_slot::<bool>(Command::VT_WITNESS_GENERATION, witness_generation, false);
  }
  #[inline]
  pub fn add_parameters(&mut self, parameters: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Command::VT_PARAMETERS, parameters);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> CommandBuilder<'a, 'b> {
    let start = _fbb.start_table();
    CommandBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Command<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum BilinearConstraintOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// A single R1CS constraint between variables.
///
/// - Represents the linear combinations of variables A, B, C such that:
///       (A) * (B) = (C)
/// - A linear combination is given as a sequence of (variable ID, coefficient).
pub struct BilinearConstraint<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for BilinearConstraint<'a> {
    type Inner = BilinearConstraint<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> BilinearConstraint<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        BilinearConstraint {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args BilinearConstraintArgs<'args>) -> flatbuffers::WIPOffset<BilinearConstraint<'bldr>> {
      let mut builder = BilinearConstraintBuilder::new(_fbb);
      if let Some(x) = args.linear_combination_c { builder.add_linear_combination_c(x); }
      if let Some(x) = args.linear_combination_b { builder.add_linear_combination_b(x); }
      if let Some(x) = args.linear_combination_a { builder.add_linear_combination_a(x); }
      builder.finish()
    }

    pub const VT_LINEAR_COMBINATION_A: flatbuffers::VOffsetT = 4;
    pub const VT_LINEAR_COMBINATION_B: flatbuffers::VOffsetT = 6;
    pub const VT_LINEAR_COMBINATION_C: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn linear_combination_a(&self) -> Option<Variables<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Variables<'a>>>(BilinearConstraint::VT_LINEAR_COMBINATION_A, None)
  }
  #[inline]
  pub fn linear_combination_b(&self) -> Option<Variables<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Variables<'a>>>(BilinearConstraint::VT_LINEAR_COMBINATION_B, None)
  }
  #[inline]
  pub fn linear_combination_c(&self) -> Option<Variables<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<Variables<'a>>>(BilinearConstraint::VT_LINEAR_COMBINATION_C, None)
  }
}

pub struct BilinearConstraintArgs<'a> {
    pub linear_combination_a: Option<flatbuffers::WIPOffset<Variables<'a >>>,
    pub linear_combination_b: Option<flatbuffers::WIPOffset<Variables<'a >>>,
    pub linear_combination_c: Option<flatbuffers::WIPOffset<Variables<'a >>>,
}
impl<'a> Default for BilinearConstraintArgs<'a> {
    #[inline]
    fn default() -> Self {
        BilinearConstraintArgs {
            linear_combination_a: None,
            linear_combination_b: None,
            linear_combination_c: None,
        }
    }
}
pub struct BilinearConstraintBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> BilinearConstraintBuilder<'a, 'b> {
  #[inline]
  pub fn add_linear_combination_a(&mut self, linear_combination_a: flatbuffers::WIPOffset<Variables<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Variables>>(BilinearConstraint::VT_LINEAR_COMBINATION_A, linear_combination_a);
  }
  #[inline]
  pub fn add_linear_combination_b(&mut self, linear_combination_b: flatbuffers::WIPOffset<Variables<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Variables>>(BilinearConstraint::VT_LINEAR_COMBINATION_B, linear_combination_b);
  }
  #[inline]
  pub fn add_linear_combination_c(&mut self, linear_combination_c: flatbuffers::WIPOffset<Variables<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Variables>>(BilinearConstraint::VT_LINEAR_COMBINATION_C, linear_combination_c);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> BilinearConstraintBuilder<'a, 'b> {
    let start = _fbb.start_table();
    BilinearConstraintBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<BilinearConstraint<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum VariablesOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// A description of multiple variables.
///
/// - Each variable is identified by a numerical ID.
/// - Each variable can be assigned a concrete value.
/// - In `Circuit.connections`, the IDs indicate which variables are
///   meant to be shared as inputs or outputs of a sub-circuit.
/// - During witness generation, the values form the assignment to the variables.
/// - In `BilinearConstraint` linear combinations, the values are the coefficients
///   applied to variables in a linear combination.
pub struct Variables<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Variables<'a> {
    type Inner = Variables<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Variables<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Variables {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args VariablesArgs<'args>) -> flatbuffers::WIPOffset<Variables<'bldr>> {
      let mut builder = VariablesBuilder::new(_fbb);
      if let Some(x) = args.info { builder.add_info(x); }
      if let Some(x) = args.values { builder.add_values(x); }
      if let Some(x) = args.variable_ids { builder.add_variable_ids(x); }
      builder.finish()
    }

    pub const VT_VARIABLE_IDS: flatbuffers::VOffsetT = 4;
    pub const VT_VALUES: flatbuffers::VOffsetT = 6;
    pub const VT_INFO: flatbuffers::VOffsetT = 8;

  /// The IDs of the variables.
  ///
  /// - IDs must be unique within a constraint system.
  /// - The ID 0 always represents the constant variable one.
  #[inline]
  pub fn variable_ids(&self) -> Option<flatbuffers::Vector<'a, u64>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u64>>>(Variables::VT_VARIABLE_IDS, None)
  }
  /// Optional: values assigned to variables.
  ///
  /// - Values are finite field elements as defined by `circuit.field_maximum`.
  /// - Elements are represented in canonical little-endian form.
  /// - Elements appear in the same order as variable_ids.
  /// - Multiple elements are concatenated in a single byte array.
  /// - The element representation may be truncated and its size shorter
  ///   than `circuit.field_maximum`. Truncated bytes are treated as zeros.
  /// - The size of an element representation is determined by:
  ///
  ///     element size = values.length / variable_ids.length
  #[inline]
  pub fn values(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(Variables::VT_VALUES, None).map(|v| v.safe_slice())
  }
  /// Optional: Any complementary info that may be useful to the recipient.
  ///
  /// Example: human-readable names.
  /// Example: custom variable typing information (`is_bit`, ...).
  /// Example: a Merkle authentication path in some custom format.
  #[inline]
  pub fn info(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<KeyValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<KeyValue<'a>>>>>(Variables::VT_INFO, None)
  }
}

pub struct VariablesArgs<'a> {
    pub variable_ids: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u64>>>,
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
    pub info: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<KeyValue<'a >>>>>,
}
impl<'a> Default for VariablesArgs<'a> {
    #[inline]
    fn default() -> Self {
        VariablesArgs {
            variable_ids: None,
            values: None,
            info: None,
        }
    }
}
pub struct VariablesBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> VariablesBuilder<'a, 'b> {
  #[inline]
  pub fn add_variable_ids(&mut self, variable_ids: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u64>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Variables::VT_VARIABLE_IDS, variable_ids);
  }
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Variables::VT_VALUES, values);
  }
  #[inline]
  pub fn add_info(&mut self, info: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<KeyValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Variables::VT_INFO, info);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> VariablesBuilder<'a, 'b> {
    let start = _fbb.start_table();
    VariablesBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Variables<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum KeyValueOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

/// Generic key-value for custom attributes.
/// The key must be a string.
/// The value can be one of several types.
pub struct KeyValue<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for KeyValue<'a> {
    type Inner = KeyValue<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> KeyValue<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        KeyValue {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args KeyValueArgs<'args>) -> flatbuffers::WIPOffset<KeyValue<'bldr>> {
      let mut builder = KeyValueBuilder::new(_fbb);
      builder.add_number(args.number);
      if let Some(x) = args.text { builder.add_text(x); }
      if let Some(x) = args.data { builder.add_data(x); }
      if let Some(x) = args.key { builder.add_key(x); }
      builder.finish()
    }

    pub const VT_KEY: flatbuffers::VOffsetT = 4;
    pub const VT_DATA: flatbuffers::VOffsetT = 6;
    pub const VT_TEXT: flatbuffers::VOffsetT = 8;
    pub const VT_NUMBER: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn key(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(KeyValue::VT_KEY, None)
  }
  #[inline]
  pub fn data(&self) -> Option<&'a [u8]> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, u8>>>(KeyValue::VT_DATA, None).map(|v| v.safe_slice())
  }
  #[inline]
  pub fn text(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(KeyValue::VT_TEXT, None)
  }
  #[inline]
  pub fn number(&self) -> i64 {
    self._tab.get::<i64>(KeyValue::VT_NUMBER, Some(0)).unwrap()
  }
}

pub struct KeyValueArgs<'a> {
    pub key: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub data: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a ,  u8>>>,
    pub text: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub number: i64,
}
impl<'a> Default for KeyValueArgs<'a> {
    #[inline]
    fn default() -> Self {
        KeyValueArgs {
            key: None,
            data: None,
            text: None,
            number: 0,
        }
    }
}
pub struct KeyValueBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> KeyValueBuilder<'a, 'b> {
  #[inline]
  pub fn add_key(&mut self, key: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_KEY, key);
  }
  #[inline]
  pub fn add_data(&mut self, data: flatbuffers::WIPOffset<flatbuffers::Vector<'b , u8>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_DATA, data);
  }
  #[inline]
  pub fn add_text(&mut self, text: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(KeyValue::VT_TEXT, text);
  }
  #[inline]
  pub fn add_number(&mut self, number: i64) {
    self.fbb_.push_slot::<i64>(KeyValue::VT_NUMBER, number, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> KeyValueBuilder<'a, 'b> {
    let start = _fbb.start_table();
    KeyValueBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<KeyValue<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum RootOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Root<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Root<'a> {
    type Inner = Root<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Root<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Root {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args RootArgs) -> flatbuffers::WIPOffset<Root<'bldr>> {
      let mut builder = RootBuilder::new(_fbb);
      if let Some(x) = args.message { builder.add_message(x); }
      builder.add_message_type(args.message_type);
      builder.finish()
    }

    pub const VT_MESSAGE_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_MESSAGE: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn message_type(&self) -> Message {
    self._tab.get::<Message>(Root::VT_MESSAGE_TYPE, Some(Message::NONE)).unwrap()
  }
  #[inline]
  pub fn message(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Root::VT_MESSAGE, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_circuit(&self) -> Option<Circuit<'a>> {
    if self.message_type() == Message::Circuit {
      self.message().map(|u| Circuit::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_constraint_system(&self) -> Option<ConstraintSystem<'a>> {
    if self.message_type() == Message::ConstraintSystem {
      self.message().map(|u| ConstraintSystem::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_witness(&self) -> Option<Witness<'a>> {
    if self.message_type() == Message::Witness {
      self.message().map(|u| Witness::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn message_as_command(&self) -> Option<Command<'a>> {
    if self.message_type() == Message::Command {
      self.message().map(|u| Command::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct RootArgs {
    pub message_type: Message,
    pub message: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for RootArgs {
    #[inline]
    fn default() -> Self {
        RootArgs {
            message_type: Message::NONE,
            message: None,
        }
    }
}
pub struct RootBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> RootBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_type(&mut self, message_type: Message) {
    self.fbb_.push_slot::<Message>(Root::VT_MESSAGE_TYPE, message_type, Message::NONE);
  }
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Root::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> RootBuilder<'a, 'b> {
    let start = _fbb.start_table();
    RootBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Root<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_root<'a>(buf: &'a [u8]) -> Root<'a> {
  flatbuffers::get_root::<Root<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_root<'a>(buf: &'a [u8]) -> Root<'a> {
  flatbuffers::get_size_prefixed_root::<Root<'a>>(buf)
}

pub const ROOT_IDENTIFIER: &'static str = "zkif";

#[inline]
pub fn root_buffer_has_identifier(buf: &[u8]) -> bool {
  return flatbuffers::buffer_has_identifier(buf, ROOT_IDENTIFIER, false);
}

#[inline]
pub fn root_size_prefixed_buffer_has_identifier(buf: &[u8]) -> bool {
  return flatbuffers::buffer_has_identifier(buf, ROOT_IDENTIFIER, true);
}

pub const ROOT_EXTENSION: &'static str = "zkif";

#[inline]
pub fn finish_root_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Root<'a>>) {
  fbb.finish(root, Some(ROOT_IDENTIFIER));
}

#[inline]
pub fn finish_size_prefixed_root_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Root<'a>>) {
  fbb.finish_size_prefixed(root, Some(ROOT_IDENTIFIER));
}
}  // pub mod zkinterface

