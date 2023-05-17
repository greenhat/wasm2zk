use pliron::context::Context;
use pliron::context::Ptr;
use pliron::dialects::builtin::attributes::IntegerAttr;
use pliron::dialects::builtin::ops::ConstantOp;
use pliron::dialects::builtin::types::IntegerType;
use pliron::dialects::builtin::types::Signedness;
use pliron::op::Op;
use pliron::r#type::TypeObj;

use crate::func_builder::FuncBuilder;

pub struct InstBuilder<'a> {
    ctx: &'a mut Context,
    fbuilder: &'a mut FuncBuilder<'a>,
    i32_type: Ptr<TypeObj>,
}

impl<'a> InstBuilder<'a> {
    fn i32_type(ctx: &mut Context) -> Ptr<TypeObj> {
        IntegerType::get(ctx, 32, Signedness::Signed)
    }

    pub fn new(ctx: &mut Context, fbuilder: &mut FuncBuilder) -> InstBuilder<'a> {
        InstBuilder {
            fbuilder,
            ctx,
            i32_type: Self::i32_type(ctx),
        }
    }

    pub fn i32const(&mut self, value: i32) {
        let op =
            ConstantOp::new_unlinked(self.ctx, IntegerAttr::create(self.i32_type, value.into()));
        op.get_operation()
            .insert_at_back(self.fbuilder.get_entry_block(), self.ctx);
    }

    pub fn i64const(&mut self, value: i64) {
        self.fbuilder.push(Inst::I64Const { value });
    }

    pub fn ret(&mut self) {
        self.fbuilder.push(Inst::Return);
    }

    pub fn end(&mut self) {
        self.fbuilder.push(Inst::End);
    }

    pub fn local_get(&mut self, local_index: u32) {
        self.fbuilder.push(Inst::LocalGet {
            local_idx: local_index,
        });
    }

    pub fn local_tee(&mut self, local_index: u32) {
        self.fbuilder.push(Inst::LocalTee {
            local_idx: local_index,
        });
    }

    pub fn local_set(&mut self, local_index: u32) {
        self.fbuilder.push(Inst::LocalSet {
            local_idx: local_index,
        });
    }

    pub fn i32add(&mut self) {
        self.fbuilder.push(Inst::I32Add);
    }

    pub fn i32eqz(&mut self) {
        self.fbuilder.push(Inst::I32Eqz);
    }

    pub fn i32wrapi64(&mut self) {
        self.fbuilder.push(Inst::I32WrapI64);
    }

    pub fn i32and(&mut self) {
        self.fbuilder.push(Inst::I32And);
    }

    pub fn i32geu(&mut self) {
        self.fbuilder.push(Inst::I32GeU);
    }

    pub fn i64add(&mut self) {
        self.fbuilder.push(Inst::I32Add);
    }

    pub fn i64eqz(&mut self) {
        self.fbuilder.push(Inst::I64Eqz);
    }

    pub fn i64eq(&mut self) {
        self.fbuilder.push(Inst::I64Eq);
    }

    pub fn i64and(&mut self) {
        self.fbuilder.push(Inst::I64And);
    }

    pub fn i64geu(&mut self) {
        self.fbuilder.push(Inst::I64GeU);
    }

    pub fn i64ne(&mut self) {
        self.fbuilder.push(Inst::I64Ne);
    }

    pub fn i64extendi32u(&mut self) {
        self.fbuilder.push(Inst::I64ExtendI32U);
    }

    pub fn call(&mut self, func_index: u32) {
        self.fbuilder.push(Inst::Call {
            func_idx: func_index.into(),
        });
    }

    pub fn nop(&mut self) {
        self.fbuilder.push(Inst::Nop);
    }

    pub fn unreachable(&mut self) {
        self.fbuilder.push(Inst::Unreachable);
    }

    pub fn bloop(&mut self, block_type: BlockType) {
        self.fbuilder.push(Inst::Loop { block_type });
    }

    pub fn block(&mut self, blockty: BlockType) {
        self.fbuilder.push(Inst::Block { blockty });
    }

    pub fn br_if(&mut self, relative_depth: u32) {
        self.fbuilder.push(Inst::BrIf { relative_depth });
    }

    pub fn br(&mut self, relative_depth: u32) {
        self.fbuilder.push(Inst::Br { relative_depth });
    }
}