//! derived from Cranelift/wasm-types
//!
//! Internal dependency of Wasmtime and c2zk that defines types for
//! WebAssembly.

use derive_more::From;
pub use wasmparser;
use wasmparser::ValType;

use std::convert::TryFrom;

use crate::error::{WasmError, WasmResult};

// /// WebAssembly function type -- equivalent of `wasmparser`'s FuncType.
// #[derive(Debug, Clone, Eq, PartialEq, Hash)]
// pub struct WasmFuncType {
//     params: Box<[ValType]>,
//     returns: Box<[ValType]>,
// }

// impl WasmFuncType {
//     #[inline]
//     pub fn new(params: Box<[ValType]>, returns: Box<[ValType]>) -> Self {
//         WasmFuncType { params, returns }
//     }

//     /// Function params types.
//     #[allow(dead_code)]
//     #[inline]
//     pub fn params(&self) -> &[ValType] {
//         &self.params
//     }

//     /// Returns params types.
//     #[allow(dead_code)]
//     #[inline]
//     pub fn returns(&self) -> &[ValType] {
//         &self.returns
//     }
// }

// impl TryFrom<wasmparser::FuncType> for WasmFuncType {
//     type Error = WasmError;
//     fn try_from(ty: wasmparser::FuncType) -> Result<Self, Self::Error> {
//         let params = Box::new(ty.params());
//         let returns = Box::new(ty.results());
//         Ok(Self::new(params, returns))
//     }
// }

/// Index type of a function (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct FuncIndex(u32);

/// Index type of a defined function inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct DefinedFuncIndex(u32);

/// Index type of a defined table inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct DefinedTableIndex(u32);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct DefinedMemoryIndex(u32);

/// Index type of a defined memory inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct OwnedMemoryIndex(u32);

/// Index type of a defined global inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct DefinedGlobalIndex(u32);

/// Index type of a table (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct TableIndex(u32);

/// Index type of a global variable (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct GlobalIndex(u32);

/// Index type of a linear memory (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct MemoryIndex(u32);

/// Index type of a signature (imported or defined) inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct SignatureIndex(u32);

/// Index type of a passive data segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct DataIndex(u32);

/// Index type of a passive element segment inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct ElemIndex(u32);

/// Index type of a type inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct TypeIndex(u32);

/// Index type of an event inside the WebAssembly module.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, From)]
pub struct TagIndex(u32);

/// An index of an entity.
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum EntityIndex {
    /// Function index.
    Function(FuncIndex),
    /// Table index.
    Table(TableIndex),
    /// Memory index.
    Memory(MemoryIndex),
    /// Global index.
    Global(GlobalIndex),
}

impl From<FuncIndex> for EntityIndex {
    fn from(idx: FuncIndex) -> EntityIndex {
        EntityIndex::Function(idx)
    }
}

impl From<TableIndex> for EntityIndex {
    fn from(idx: TableIndex) -> EntityIndex {
        EntityIndex::Table(idx)
    }
}

impl From<MemoryIndex> for EntityIndex {
    fn from(idx: MemoryIndex) -> EntityIndex {
        EntityIndex::Memory(idx)
    }
}

impl From<GlobalIndex> for EntityIndex {
    fn from(idx: GlobalIndex) -> EntityIndex {
        EntityIndex::Global(idx)
    }
}

/// A type of an item in a wasm module where an item is typically something that
/// can be exported.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum EntityType {
    /// A global variable with the specified content type
    Global(Global),
    /// A linear memory with the specified limits
    Memory(Memory),
    /// An event definition.
    Tag(Tag),
    /// A table with the specified element type and limits
    Table(Table),
    /// A function type where the index points to the type section and records a
    /// function signature.
    Function(SignatureIndex),
}

/// A WebAssembly global.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Global {
    /// The Wasm type of the value stored in the global.
    pub wasm_ty: ValType,
    /// A flag indicating whether the value may change at runtime.
    pub mutability: bool,
    /// The source of the initial value.
    pub initializer: GlobalInit,
}

/// Globals are initialized via the `const` operators or by referring to another import.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum GlobalInit {
    /// An `i32.const`.
    I32Const(i32),
    /// An `i64.const`.
    I64Const(i64),
    /// An `f32.const`.
    F32Const(u32),
    /// An `f64.const`.
    F64Const(u64),
    /// A `vconst`.
    V128Const(u128),
    /// A `global.get` of another global.
    GetGlobal(GlobalIndex),
    /// A `ref.null`.
    RefNullConst,
    /// A `ref.func <index>`.
    RefFunc(FuncIndex),
    ///< The global is imported from, and thus initialized by, a different module.
    Import,
}

impl Global {
    /// Creates a new `Global` type from wasmparser's representation.
    #[allow(dead_code)]
    pub fn new(ty: wasmparser::GlobalType, initializer: GlobalInit) -> WasmResult<Global> {
        Ok(Global {
            wasm_ty: ty.content_type,
            mutability: ty.mutable,
            initializer,
        })
    }
}

/// WebAssembly table.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Table {
    /// The table elements' Wasm type.
    pub wasm_ty: ValType,
    /// The minimum number of elements in the table.
    pub minimum: u32,
    /// The maximum number of elements in the table.
    pub maximum: Option<u32>,
}

impl TryFrom<wasmparser::TableType> for Table {
    type Error = WasmError;

    fn try_from(ty: wasmparser::TableType) -> WasmResult<Table> {
        Ok(Table {
            wasm_ty: ty.element_type,
            minimum: ty.initial,
            maximum: ty.maximum,
        })
    }
}

/// WebAssembly linear memory.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Memory {
    /// The minimum number of pages in the memory.
    pub minimum: u64,
    /// The maximum number of pages in the memory.
    pub maximum: Option<u64>,
    /// Whether the memory may be shared between multiple threads.
    pub shared: bool,
    /// Whether or not this is a 64-bit memory
    pub memory64: bool,
}

impl From<wasmparser::MemoryType> for Memory {
    fn from(ty: wasmparser::MemoryType) -> Memory {
        Memory {
            minimum: ty.initial,
            maximum: ty.maximum,
            shared: ty.shared,
            memory64: ty.memory64,
        }
    }
}

/// WebAssembly event.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Tag {
    /// The event signature type.
    pub ty: TypeIndex,
}

impl From<wasmparser::TagType> for Tag {
    fn from(ty: wasmparser::TagType) -> Tag {
        match ty.kind {
            wasmparser::TagKind::Exception => Tag {
                ty: TypeIndex(ty.func_type_idx),
            },
        }
    }
}
