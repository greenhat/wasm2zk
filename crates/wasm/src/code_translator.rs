use c2zk_frontend::FuncBuilder;
use wasmparser::{FuncValidator, Operator, WasmModuleResources};

use crate::error::WasmResult;

/// Translates wasm operators into c2zk IR instructions.
#[allow(unused_variables)]
pub fn translate_operator(
    validator: &mut FuncValidator<impl WasmModuleResources>,
    op: &Operator,
    builder: &mut FuncBuilder,
) -> WasmResult<()> {
    match op {
        Operator::Unreachable => (),
        Operator::Nop => todo!(),
        Operator::Block { blockty } => todo!(),
        Operator::Loop { blockty } => todo!(),
        Operator::If { blockty } => todo!(),
        Operator::Else => todo!(),
        Operator::Try { blockty } => todo!(),
        Operator::Catch { tag_index } => todo!(),
        Operator::Throw { tag_index } => todo!(),
        Operator::Rethrow { relative_depth } => todo!(),
        Operator::End => builder.ins().end(),
        Operator::Br { relative_depth } => todo!(),
        Operator::BrIf { relative_depth } => todo!(),
        Operator::BrTable { targets } => todo!(),
        Operator::Return => builder.ins().ret(),
        Operator::Call { function_index } => todo!(),
        Operator::CallIndirect {
            type_index,
            table_index,
            table_byte,
        } => todo!(),
        Operator::ReturnCall { function_index } => todo!(),
        Operator::ReturnCallIndirect {
            type_index,
            table_index,
        } => todo!(),
        Operator::Delegate { relative_depth } => todo!(),
        Operator::CatchAll => todo!(),
        Operator::Drop => todo!(),
        Operator::Select => todo!(),
        Operator::TypedSelect { ty } => todo!(),
        Operator::LocalGet { local_index } => todo!(),
        Operator::LocalSet { local_index } => todo!(),
        Operator::LocalTee { local_index } => todo!(),
        Operator::GlobalGet { global_index } => todo!(),
        Operator::GlobalSet { global_index } => todo!(),
        Operator::I32Load { memarg } => todo!(),
        Operator::I64Load { memarg } => todo!(),
        Operator::F32Load { memarg } => todo!(),
        Operator::F64Load { memarg } => todo!(),
        Operator::I32Load8S { memarg } => todo!(),
        Operator::I32Load8U { memarg } => todo!(),
        Operator::I32Load16S { memarg } => todo!(),
        Operator::I32Load16U { memarg } => todo!(),
        Operator::I64Load8S { memarg } => todo!(),
        Operator::I64Load8U { memarg } => todo!(),
        Operator::I64Load16S { memarg } => todo!(),
        Operator::I64Load16U { memarg } => todo!(),
        Operator::I64Load32S { memarg } => todo!(),
        Operator::I64Load32U { memarg } => todo!(),
        Operator::I32Store { memarg } => todo!(),
        Operator::I64Store { memarg } => todo!(),
        Operator::F32Store { memarg } => todo!(),
        Operator::F64Store { memarg } => todo!(),
        Operator::I32Store8 { memarg } => todo!(),
        Operator::I32Store16 { memarg } => todo!(),
        Operator::I64Store8 { memarg } => todo!(),
        Operator::I64Store16 { memarg } => todo!(),
        Operator::I64Store32 { memarg } => todo!(),
        Operator::MemorySize { mem, mem_byte } => todo!(),
        Operator::MemoryGrow { mem, mem_byte } => todo!(),
        Operator::I32Const { value } => builder.ins().i32const(*value),
        Operator::I64Const { value } => todo!(),
        Operator::F32Const { value } => todo!(),
        Operator::F64Const { value } => todo!(),
        Operator::RefNull { ty } => todo!(),
        Operator::RefIsNull => todo!(),
        Operator::RefFunc { function_index } => todo!(),
        Operator::I32Eqz => todo!(),
        Operator::I32Eq => todo!(),
        Operator::I32Ne => todo!(),
        Operator::I32LtS => todo!(),
        Operator::I32LtU => todo!(),
        Operator::I32GtS => todo!(),
        Operator::I32GtU => todo!(),
        Operator::I32LeS => todo!(),
        Operator::I32LeU => todo!(),
        Operator::I32GeS => todo!(),
        Operator::I32GeU => todo!(),
        Operator::I64Eqz => todo!(),
        Operator::I64Eq => todo!(),
        Operator::I64Ne => todo!(),
        Operator::I64LtS => todo!(),
        Operator::I64LtU => todo!(),
        Operator::I64GtS => todo!(),
        Operator::I64GtU => todo!(),
        Operator::I64LeS => todo!(),
        Operator::I64LeU => todo!(),
        Operator::I64GeS => todo!(),
        Operator::I64GeU => todo!(),
        Operator::F32Eq => todo!(),
        Operator::F32Ne => todo!(),
        Operator::F32Lt => todo!(),
        Operator::F32Gt => todo!(),
        Operator::F32Le => todo!(),
        Operator::F32Ge => todo!(),
        Operator::F64Eq => todo!(),
        Operator::F64Ne => todo!(),
        Operator::F64Lt => todo!(),
        Operator::F64Gt => todo!(),
        Operator::F64Le => todo!(),
        Operator::F64Ge => todo!(),
        Operator::I32Clz => todo!(),
        Operator::I32Ctz => todo!(),
        Operator::I32Popcnt => todo!(),
        Operator::I32Add => todo!(),
        Operator::I32Sub => todo!(),
        Operator::I32Mul => todo!(),
        Operator::I32DivS => todo!(),
        Operator::I32DivU => todo!(),
        Operator::I32RemS => todo!(),
        Operator::I32RemU => todo!(),
        Operator::I32And => todo!(),
        Operator::I32Or => todo!(),
        Operator::I32Xor => todo!(),
        Operator::I32Shl => todo!(),
        Operator::I32ShrS => todo!(),
        Operator::I32ShrU => todo!(),
        Operator::I32Rotl => todo!(),
        Operator::I32Rotr => todo!(),
        Operator::I64Clz => todo!(),
        Operator::I64Ctz => todo!(),
        Operator::I64Popcnt => todo!(),
        Operator::I64Add => todo!(),
        Operator::I64Sub => todo!(),
        Operator::I64Mul => todo!(),
        Operator::I64DivS => todo!(),
        Operator::I64DivU => todo!(),
        Operator::I64RemS => todo!(),
        Operator::I64RemU => todo!(),
        Operator::I64And => todo!(),
        Operator::I64Or => todo!(),
        Operator::I64Xor => todo!(),
        Operator::I64Shl => todo!(),
        Operator::I64ShrS => todo!(),
        Operator::I64ShrU => todo!(),
        Operator::I64Rotl => todo!(),
        Operator::I64Rotr => todo!(),
        Operator::F32Abs => todo!(),
        Operator::F32Neg => todo!(),
        Operator::F32Ceil => todo!(),
        Operator::F32Floor => todo!(),
        Operator::F32Trunc => todo!(),
        Operator::F32Nearest => todo!(),
        Operator::F32Sqrt => todo!(),
        Operator::F32Add => todo!(),
        Operator::F32Sub => todo!(),
        Operator::F32Mul => todo!(),
        Operator::F32Div => todo!(),
        Operator::F32Min => todo!(),
        Operator::F32Max => todo!(),
        Operator::F32Copysign => todo!(),
        Operator::F64Abs => todo!(),
        Operator::F64Neg => todo!(),
        Operator::F64Ceil => todo!(),
        Operator::F64Floor => todo!(),
        Operator::F64Trunc => todo!(),
        Operator::F64Nearest => todo!(),
        Operator::F64Sqrt => todo!(),
        Operator::F64Add => todo!(),
        Operator::F64Sub => todo!(),
        Operator::F64Mul => todo!(),
        Operator::F64Div => todo!(),
        Operator::F64Min => todo!(),
        Operator::F64Max => todo!(),
        Operator::F64Copysign => todo!(),
        Operator::I32WrapI64 => todo!(),
        Operator::I32TruncF32S => todo!(),
        Operator::I32TruncF32U => todo!(),
        Operator::I32TruncF64S => todo!(),
        Operator::I32TruncF64U => todo!(),
        Operator::I64ExtendI32S => todo!(),
        Operator::I64ExtendI32U => todo!(),
        Operator::I64TruncF32S => todo!(),
        Operator::I64TruncF32U => todo!(),
        Operator::I64TruncF64S => todo!(),
        Operator::I64TruncF64U => todo!(),
        Operator::F32ConvertI32S => todo!(),
        Operator::F32ConvertI32U => todo!(),
        Operator::F32ConvertI64S => todo!(),
        Operator::F32ConvertI64U => todo!(),
        Operator::F32DemoteF64 => todo!(),
        Operator::F64ConvertI32S => todo!(),
        Operator::F64ConvertI32U => todo!(),
        Operator::F64ConvertI64S => todo!(),
        Operator::F64ConvertI64U => todo!(),
        Operator::F64PromoteF32 => todo!(),
        Operator::I32ReinterpretF32 => todo!(),
        Operator::I64ReinterpretF64 => todo!(),
        Operator::F32ReinterpretI32 => todo!(),
        Operator::F64ReinterpretI64 => todo!(),
        Operator::I32Extend8S => todo!(),
        Operator::I32Extend16S => todo!(),
        Operator::I64Extend8S => todo!(),
        Operator::I64Extend16S => todo!(),
        Operator::I64Extend32S => todo!(),
        Operator::I32TruncSatF32S => todo!(),
        Operator::I32TruncSatF32U => todo!(),
        Operator::I32TruncSatF64S => todo!(),
        Operator::I32TruncSatF64U => todo!(),
        Operator::I64TruncSatF32S => todo!(),
        Operator::I64TruncSatF32U => todo!(),
        Operator::I64TruncSatF64S => todo!(),
        Operator::I64TruncSatF64U => todo!(),
        Operator::MemoryInit { data_index, mem } => todo!(),
        Operator::DataDrop { data_index } => todo!(),
        Operator::MemoryCopy { dst_mem, src_mem } => todo!(),
        Operator::MemoryFill { mem } => todo!(),
        Operator::TableInit { elem_index, table } => todo!(),
        Operator::ElemDrop { elem_index } => todo!(),
        Operator::TableCopy {
            dst_table,
            src_table,
        } => todo!(),
        Operator::TableFill { table } => todo!(),
        Operator::TableGet { table } => todo!(),
        Operator::TableSet { table } => todo!(),
        Operator::TableGrow { table } => todo!(),
        Operator::TableSize { table } => todo!(),
        Operator::MemoryAtomicNotify { memarg } => todo!(),
        Operator::MemoryAtomicWait32 { memarg } => todo!(),
        Operator::MemoryAtomicWait64 { memarg } => todo!(),
        Operator::AtomicFence => todo!(),
        Operator::I32AtomicLoad { memarg } => todo!(),
        Operator::I64AtomicLoad { memarg } => todo!(),
        Operator::I32AtomicLoad8U { memarg } => todo!(),
        Operator::I32AtomicLoad16U { memarg } => todo!(),
        Operator::I64AtomicLoad8U { memarg } => todo!(),
        Operator::I64AtomicLoad16U { memarg } => todo!(),
        Operator::I64AtomicLoad32U { memarg } => todo!(),
        Operator::I32AtomicStore { memarg } => todo!(),
        Operator::I64AtomicStore { memarg } => todo!(),
        Operator::I32AtomicStore8 { memarg } => todo!(),
        Operator::I32AtomicStore16 { memarg } => todo!(),
        Operator::I64AtomicStore8 { memarg } => todo!(),
        Operator::I64AtomicStore16 { memarg } => todo!(),
        Operator::I64AtomicStore32 { memarg } => todo!(),
        Operator::I32AtomicRmwAdd { memarg } => todo!(),
        Operator::I64AtomicRmwAdd { memarg } => todo!(),
        Operator::I32AtomicRmw8AddU { memarg } => todo!(),
        Operator::I32AtomicRmw16AddU { memarg } => todo!(),
        Operator::I64AtomicRmw8AddU { memarg } => todo!(),
        Operator::I64AtomicRmw16AddU { memarg } => todo!(),
        Operator::I64AtomicRmw32AddU { memarg } => todo!(),
        Operator::I32AtomicRmwSub { memarg } => todo!(),
        Operator::I64AtomicRmwSub { memarg } => todo!(),
        Operator::I32AtomicRmw8SubU { memarg } => todo!(),
        Operator::I32AtomicRmw16SubU { memarg } => todo!(),
        Operator::I64AtomicRmw8SubU { memarg } => todo!(),
        Operator::I64AtomicRmw16SubU { memarg } => todo!(),
        Operator::I64AtomicRmw32SubU { memarg } => todo!(),
        Operator::I32AtomicRmwAnd { memarg } => todo!(),
        Operator::I64AtomicRmwAnd { memarg } => todo!(),
        Operator::I32AtomicRmw8AndU { memarg } => todo!(),
        Operator::I32AtomicRmw16AndU { memarg } => todo!(),
        Operator::I64AtomicRmw8AndU { memarg } => todo!(),
        Operator::I64AtomicRmw16AndU { memarg } => todo!(),
        Operator::I64AtomicRmw32AndU { memarg } => todo!(),
        Operator::I32AtomicRmwOr { memarg } => todo!(),
        Operator::I64AtomicRmwOr { memarg } => todo!(),
        Operator::I32AtomicRmw8OrU { memarg } => todo!(),
        Operator::I32AtomicRmw16OrU { memarg } => todo!(),
        Operator::I64AtomicRmw8OrU { memarg } => todo!(),
        Operator::I64AtomicRmw16OrU { memarg } => todo!(),
        Operator::I64AtomicRmw32OrU { memarg } => todo!(),
        Operator::I32AtomicRmwXor { memarg } => todo!(),
        Operator::I64AtomicRmwXor { memarg } => todo!(),
        Operator::I32AtomicRmw8XorU { memarg } => todo!(),
        Operator::I32AtomicRmw16XorU { memarg } => todo!(),
        Operator::I64AtomicRmw8XorU { memarg } => todo!(),
        Operator::I64AtomicRmw16XorU { memarg } => todo!(),
        Operator::I64AtomicRmw32XorU { memarg } => todo!(),
        Operator::I32AtomicRmwXchg { memarg } => todo!(),
        Operator::I64AtomicRmwXchg { memarg } => todo!(),
        Operator::I32AtomicRmw8XchgU { memarg } => todo!(),
        Operator::I32AtomicRmw16XchgU { memarg } => todo!(),
        Operator::I64AtomicRmw8XchgU { memarg } => todo!(),
        Operator::I64AtomicRmw16XchgU { memarg } => todo!(),
        Operator::I64AtomicRmw32XchgU { memarg } => todo!(),
        Operator::I32AtomicRmwCmpxchg { memarg } => todo!(),
        Operator::I64AtomicRmwCmpxchg { memarg } => todo!(),
        Operator::I32AtomicRmw8CmpxchgU { memarg } => todo!(),
        Operator::I32AtomicRmw16CmpxchgU { memarg } => todo!(),
        Operator::I64AtomicRmw8CmpxchgU { memarg } => todo!(),
        Operator::I64AtomicRmw16CmpxchgU { memarg } => todo!(),
        Operator::I64AtomicRmw32CmpxchgU { memarg } => todo!(),
        Operator::V128Load { memarg } => todo!(),
        Operator::V128Load8x8S { memarg } => todo!(),
        Operator::V128Load8x8U { memarg } => todo!(),
        Operator::V128Load16x4S { memarg } => todo!(),
        Operator::V128Load16x4U { memarg } => todo!(),
        Operator::V128Load32x2S { memarg } => todo!(),
        Operator::V128Load32x2U { memarg } => todo!(),
        Operator::V128Load8Splat { memarg } => todo!(),
        Operator::V128Load16Splat { memarg } => todo!(),
        Operator::V128Load32Splat { memarg } => todo!(),
        Operator::V128Load64Splat { memarg } => todo!(),
        Operator::V128Load32Zero { memarg } => todo!(),
        Operator::V128Load64Zero { memarg } => todo!(),
        Operator::V128Store { memarg } => todo!(),
        Operator::V128Load8Lane { memarg, lane } => todo!(),
        Operator::V128Load16Lane { memarg, lane } => todo!(),
        Operator::V128Load32Lane { memarg, lane } => todo!(),
        Operator::V128Load64Lane { memarg, lane } => todo!(),
        Operator::V128Store8Lane { memarg, lane } => todo!(),
        Operator::V128Store16Lane { memarg, lane } => todo!(),
        Operator::V128Store32Lane { memarg, lane } => todo!(),
        Operator::V128Store64Lane { memarg, lane } => todo!(),
        Operator::V128Const { value } => todo!(),
        Operator::I8x16Shuffle { lanes } => todo!(),
        Operator::I8x16ExtractLaneS { lane } => todo!(),
        Operator::I8x16ExtractLaneU { lane } => todo!(),
        Operator::I8x16ReplaceLane { lane } => todo!(),
        Operator::I16x8ExtractLaneS { lane } => todo!(),
        Operator::I16x8ExtractLaneU { lane } => todo!(),
        Operator::I16x8ReplaceLane { lane } => todo!(),
        Operator::I32x4ExtractLane { lane } => todo!(),
        Operator::I32x4ReplaceLane { lane } => todo!(),
        Operator::I64x2ExtractLane { lane } => todo!(),
        Operator::I64x2ReplaceLane { lane } => todo!(),
        Operator::F32x4ExtractLane { lane } => todo!(),
        Operator::F32x4ReplaceLane { lane } => todo!(),
        Operator::F64x2ExtractLane { lane } => todo!(),
        Operator::F64x2ReplaceLane { lane } => todo!(),
        Operator::I8x16Swizzle => todo!(),
        Operator::I8x16Splat => todo!(),
        Operator::I16x8Splat => todo!(),
        Operator::I32x4Splat => todo!(),
        Operator::I64x2Splat => todo!(),
        Operator::F32x4Splat => todo!(),
        Operator::F64x2Splat => todo!(),
        Operator::I8x16Eq => todo!(),
        Operator::I8x16Ne => todo!(),
        Operator::I8x16LtS => todo!(),
        Operator::I8x16LtU => todo!(),
        Operator::I8x16GtS => todo!(),
        Operator::I8x16GtU => todo!(),
        Operator::I8x16LeS => todo!(),
        Operator::I8x16LeU => todo!(),
        Operator::I8x16GeS => todo!(),
        Operator::I8x16GeU => todo!(),
        Operator::I16x8Eq => todo!(),
        Operator::I16x8Ne => todo!(),
        Operator::I16x8LtS => todo!(),
        Operator::I16x8LtU => todo!(),
        Operator::I16x8GtS => todo!(),
        Operator::I16x8GtU => todo!(),
        Operator::I16x8LeS => todo!(),
        Operator::I16x8LeU => todo!(),
        Operator::I16x8GeS => todo!(),
        Operator::I16x8GeU => todo!(),
        Operator::I32x4Eq => todo!(),
        Operator::I32x4Ne => todo!(),
        Operator::I32x4LtS => todo!(),
        Operator::I32x4LtU => todo!(),
        Operator::I32x4GtS => todo!(),
        Operator::I32x4GtU => todo!(),
        Operator::I32x4LeS => todo!(),
        Operator::I32x4LeU => todo!(),
        Operator::I32x4GeS => todo!(),
        Operator::I32x4GeU => todo!(),
        Operator::I64x2Eq => todo!(),
        Operator::I64x2Ne => todo!(),
        Operator::I64x2LtS => todo!(),
        Operator::I64x2GtS => todo!(),
        Operator::I64x2LeS => todo!(),
        Operator::I64x2GeS => todo!(),
        Operator::F32x4Eq => todo!(),
        Operator::F32x4Ne => todo!(),
        Operator::F32x4Lt => todo!(),
        Operator::F32x4Gt => todo!(),
        Operator::F32x4Le => todo!(),
        Operator::F32x4Ge => todo!(),
        Operator::F64x2Eq => todo!(),
        Operator::F64x2Ne => todo!(),
        Operator::F64x2Lt => todo!(),
        Operator::F64x2Gt => todo!(),
        Operator::F64x2Le => todo!(),
        Operator::F64x2Ge => todo!(),
        Operator::V128Not => todo!(),
        Operator::V128And => todo!(),
        Operator::V128AndNot => todo!(),
        Operator::V128Or => todo!(),
        Operator::V128Xor => todo!(),
        Operator::V128Bitselect => todo!(),
        Operator::V128AnyTrue => todo!(),
        Operator::I8x16Abs => todo!(),
        Operator::I8x16Neg => todo!(),
        Operator::I8x16Popcnt => todo!(),
        Operator::I8x16AllTrue => todo!(),
        Operator::I8x16Bitmask => todo!(),
        Operator::I8x16NarrowI16x8S => todo!(),
        Operator::I8x16NarrowI16x8U => todo!(),
        Operator::I8x16Shl => todo!(),
        Operator::I8x16ShrS => todo!(),
        Operator::I8x16ShrU => todo!(),
        Operator::I8x16Add => todo!(),
        Operator::I8x16AddSatS => todo!(),
        Operator::I8x16AddSatU => todo!(),
        Operator::I8x16Sub => todo!(),
        Operator::I8x16SubSatS => todo!(),
        Operator::I8x16SubSatU => todo!(),
        Operator::I8x16MinS => todo!(),
        Operator::I8x16MinU => todo!(),
        Operator::I8x16MaxS => todo!(),
        Operator::I8x16MaxU => todo!(),
        Operator::I8x16AvgrU => todo!(),
        Operator::I16x8ExtAddPairwiseI8x16S => todo!(),
        Operator::I16x8ExtAddPairwiseI8x16U => todo!(),
        Operator::I16x8Abs => todo!(),
        Operator::I16x8Neg => todo!(),
        Operator::I16x8Q15MulrSatS => todo!(),
        Operator::I16x8AllTrue => todo!(),
        Operator::I16x8Bitmask => todo!(),
        Operator::I16x8NarrowI32x4S => todo!(),
        Operator::I16x8NarrowI32x4U => todo!(),
        Operator::I16x8ExtendLowI8x16S => todo!(),
        Operator::I16x8ExtendHighI8x16S => todo!(),
        Operator::I16x8ExtendLowI8x16U => todo!(),
        Operator::I16x8ExtendHighI8x16U => todo!(),
        Operator::I16x8Shl => todo!(),
        Operator::I16x8ShrS => todo!(),
        Operator::I16x8ShrU => todo!(),
        Operator::I16x8Add => todo!(),
        Operator::I16x8AddSatS => todo!(),
        Operator::I16x8AddSatU => todo!(),
        Operator::I16x8Sub => todo!(),
        Operator::I16x8SubSatS => todo!(),
        Operator::I16x8SubSatU => todo!(),
        Operator::I16x8Mul => todo!(),
        Operator::I16x8MinS => todo!(),
        Operator::I16x8MinU => todo!(),
        Operator::I16x8MaxS => todo!(),
        Operator::I16x8MaxU => todo!(),
        Operator::I16x8AvgrU => todo!(),
        Operator::I16x8ExtMulLowI8x16S => todo!(),
        Operator::I16x8ExtMulHighI8x16S => todo!(),
        Operator::I16x8ExtMulLowI8x16U => todo!(),
        Operator::I16x8ExtMulHighI8x16U => todo!(),
        Operator::I32x4ExtAddPairwiseI16x8S => todo!(),
        Operator::I32x4ExtAddPairwiseI16x8U => todo!(),
        Operator::I32x4Abs => todo!(),
        Operator::I32x4Neg => todo!(),
        Operator::I32x4AllTrue => todo!(),
        Operator::I32x4Bitmask => todo!(),
        Operator::I32x4ExtendLowI16x8S => todo!(),
        Operator::I32x4ExtendHighI16x8S => todo!(),
        Operator::I32x4ExtendLowI16x8U => todo!(),
        Operator::I32x4ExtendHighI16x8U => todo!(),
        Operator::I32x4Shl => todo!(),
        Operator::I32x4ShrS => todo!(),
        Operator::I32x4ShrU => todo!(),
        Operator::I32x4Add => todo!(),
        Operator::I32x4Sub => todo!(),
        Operator::I32x4Mul => todo!(),
        Operator::I32x4MinS => todo!(),
        Operator::I32x4MinU => todo!(),
        Operator::I32x4MaxS => todo!(),
        Operator::I32x4MaxU => todo!(),
        Operator::I32x4DotI16x8S => todo!(),
        Operator::I32x4ExtMulLowI16x8S => todo!(),
        Operator::I32x4ExtMulHighI16x8S => todo!(),
        Operator::I32x4ExtMulLowI16x8U => todo!(),
        Operator::I32x4ExtMulHighI16x8U => todo!(),
        Operator::I64x2Abs => todo!(),
        Operator::I64x2Neg => todo!(),
        Operator::I64x2AllTrue => todo!(),
        Operator::I64x2Bitmask => todo!(),
        Operator::I64x2ExtendLowI32x4S => todo!(),
        Operator::I64x2ExtendHighI32x4S => todo!(),
        Operator::I64x2ExtendLowI32x4U => todo!(),
        Operator::I64x2ExtendHighI32x4U => todo!(),
        Operator::I64x2Shl => todo!(),
        Operator::I64x2ShrS => todo!(),
        Operator::I64x2ShrU => todo!(),
        Operator::I64x2Add => todo!(),
        Operator::I64x2Sub => todo!(),
        Operator::I64x2Mul => todo!(),
        Operator::I64x2ExtMulLowI32x4S => todo!(),
        Operator::I64x2ExtMulHighI32x4S => todo!(),
        Operator::I64x2ExtMulLowI32x4U => todo!(),
        Operator::I64x2ExtMulHighI32x4U => todo!(),
        Operator::F32x4Ceil => todo!(),
        Operator::F32x4Floor => todo!(),
        Operator::F32x4Trunc => todo!(),
        Operator::F32x4Nearest => todo!(),
        Operator::F32x4Abs => todo!(),
        Operator::F32x4Neg => todo!(),
        Operator::F32x4Sqrt => todo!(),
        Operator::F32x4Add => todo!(),
        Operator::F32x4Sub => todo!(),
        Operator::F32x4Mul => todo!(),
        Operator::F32x4Div => todo!(),
        Operator::F32x4Min => todo!(),
        Operator::F32x4Max => todo!(),
        Operator::F32x4PMin => todo!(),
        Operator::F32x4PMax => todo!(),
        Operator::F64x2Ceil => todo!(),
        Operator::F64x2Floor => todo!(),
        Operator::F64x2Trunc => todo!(),
        Operator::F64x2Nearest => todo!(),
        Operator::F64x2Abs => todo!(),
        Operator::F64x2Neg => todo!(),
        Operator::F64x2Sqrt => todo!(),
        Operator::F64x2Add => todo!(),
        Operator::F64x2Sub => todo!(),
        Operator::F64x2Mul => todo!(),
        Operator::F64x2Div => todo!(),
        Operator::F64x2Min => todo!(),
        Operator::F64x2Max => todo!(),
        Operator::F64x2PMin => todo!(),
        Operator::F64x2PMax => todo!(),
        Operator::I32x4TruncSatF32x4S => todo!(),
        Operator::I32x4TruncSatF32x4U => todo!(),
        Operator::F32x4ConvertI32x4S => todo!(),
        Operator::F32x4ConvertI32x4U => todo!(),
        Operator::I32x4TruncSatF64x2SZero => todo!(),
        Operator::I32x4TruncSatF64x2UZero => todo!(),
        Operator::F64x2ConvertLowI32x4S => todo!(),
        Operator::F64x2ConvertLowI32x4U => todo!(),
        Operator::F32x4DemoteF64x2Zero => todo!(),
        Operator::F64x2PromoteLowF32x4 => todo!(),
        Operator::I8x16RelaxedSwizzle => todo!(),
        Operator::I32x4RelaxedTruncSatF32x4S => todo!(),
        Operator::I32x4RelaxedTruncSatF32x4U => todo!(),
        Operator::I32x4RelaxedTruncSatF64x2SZero => todo!(),
        Operator::I32x4RelaxedTruncSatF64x2UZero => todo!(),
        Operator::F32x4RelaxedFma => todo!(),
        Operator::F32x4RelaxedFnma => todo!(),
        Operator::F64x2RelaxedFma => todo!(),
        Operator::F64x2RelaxedFnma => todo!(),
        Operator::I8x16RelaxedLaneselect => todo!(),
        Operator::I16x8RelaxedLaneselect => todo!(),
        Operator::I32x4RelaxedLaneselect => todo!(),
        Operator::I64x2RelaxedLaneselect => todo!(),
        Operator::F32x4RelaxedMin => todo!(),
        Operator::F32x4RelaxedMax => todo!(),
        Operator::F64x2RelaxedMin => todo!(),
        Operator::F64x2RelaxedMax => todo!(),
        Operator::I16x8RelaxedQ15mulrS => todo!(),
        Operator::I16x8DotI8x16I7x16S => todo!(),
        Operator::I32x4DotI8x16I7x16AddS => todo!(),
        Operator::F32x4RelaxedDotBf16x8AddF32x4 => todo!(),
    };
    Ok(())
}