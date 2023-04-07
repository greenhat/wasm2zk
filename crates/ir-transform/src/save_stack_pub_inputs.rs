//! In Miden VM public inputs are stored on the stack. This pass saves the public inputs from the stack
//! and stores them in the memory

use std::collections::HashMap;

use c2zk_ir::ir::ext::MidenExt;
use c2zk_ir::ir::Func;
use c2zk_ir::ir::FuncType;
use c2zk_ir::ir::GlobalIndex;
use c2zk_ir::ir::Inst;
use c2zk_ir::ir::Module;
use c2zk_ir::ir::Ty;
use c2zk_ir::pass::IrPass;

// TODO: we should convert Inst::PubInputRead and Inst::PubOutputWrite to memory access in this pass as well.
// TODO: since it's Miden specific we should move it to Miden crate or a miden module in this crate

#[derive(Default)]
pub struct SaveStackPubInputsPass;

pub const SAVE_PUB_INPUTS_FUNC_NAME: &str = "save_pub_inputs";
pub const GET_NEXT_PUB_INPUT_FUNC_NAME: &str = "omni_miden_pub_input";
pub const STORE_PUB_OUTPUT_FUNC_NAME: &str = "omni_miden_pub_output";
pub const LOAD_PUB_OUTPUTS_ON_STACK_FUNC_NAME: &str = "load_pub_outputs_on_stack";

impl IrPass for SaveStackPubInputsPass {
    fn run_mod_pass(&self, module: &mut Module) {
        for func in module.functions_mut().iter_mut() {
            self.run_func_pass(func);
        }
    }

    fn run_func_pass(&self, func: &mut c2zk_ir::ir::Func) {
        todo!()
    }
}

// TODO: move memory address by type size (8 byts for i64)

fn save_pub_inputs_func(pub_inputs_addr_idx: GlobalIndex) -> Func {
    // TODO: can be re-written with GlobalGet/GlobalSet moved outside the loop
    let ins = vec![
        MidenExt::SDepth.into(), // to enter the while.true loop
        MidenExt::While.into(),
        MidenExt::SDepth.into(),
        Inst::GlobalGet {
            global_idx: pub_inputs_addr_idx,
        }, // get the address
        Inst::Dup { idx: 0 },         // duplicate the address
        Inst::Swap { idx: 3 },        // put value on top
        Inst::I32Store { offset: 0 }, // store the stack value
        Inst::I32Const { value: -1 },
        Inst::I32Add, // decrement the address
        Inst::GlobalSet {
            global_idx: pub_inputs_addr_idx,
        }, // set the new address
        Inst::I32Const { value: -1 },
        Inst::I32Add, // decrement the stack depth counter (brought by SDepth)
        MidenExt::End.into(),
    ];
    Func::new(
        SAVE_PUB_INPUTS_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}

fn get_next_pub_input_func(pub_inputs_addr_idx: GlobalIndex) -> Func {
    let ins = vec![
        Inst::GlobalGet {
            global_idx: pub_inputs_addr_idx,
        }, // get the address
        Inst::Dup { idx: 0 },        // duplicate the address
        Inst::I32Load { offset: 0 }, // load the previously saved public input
        Inst::I32Const { value: 1 },
        Inst::I32Add, // increment the address
        Inst::GlobalSet {
            global_idx: pub_inputs_addr_idx,
        },
    ];
    Func::new(
        GET_NEXT_PUB_INPUT_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![Ty::I32],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}

fn store_pub_output_func(pub_outputs_addr_idx: GlobalIndex) -> Func {
    let ins = vec![
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_idx,
        }, // get the address
        Inst::Dup { idx: 0 },         // duplicate the address
        Inst::Swap { idx: 3 },        // put value on top
        Inst::I32Store { offset: 0 }, // store the stack value to public outputs memory region
        Inst::I32Const { value: -1 },
        Inst::I32Add, // decrement the address
        Inst::GlobalSet {
            global_idx: pub_outputs_addr_idx,
        },
    ];
    Func::new(
        STORE_PUB_OUTPUT_FUNC_NAME.to_string(),
        FuncType {
            params: vec![Ty::I32],
            results: vec![],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}

fn load_pub_outputs_on_stack_func(
    pub_outputs_addr_orig_idx: GlobalIndex,
    pub_outputs_addr_idx: GlobalIndex,
) -> Func {
    // TODO: can be re-written with GlobalGet/GlobalSet moved outside the loop
    // and residual addresses on the stack removed
    let ins = vec![
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_idx,
        }, // get the address
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_orig_idx,
        }, // get the original(start) address
        Inst::I32Sub, // get the number of public outputs * type size
        MidenExt::While.into(),
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_idx,
        }, // get the address
        Inst::Dup { idx: 0 },        // duplicate the address
        Inst::I32Load { offset: 0 }, // load the public output on the stack
        Inst::I32Const { value: 1 },
        Inst::I32Add,         // increment the address
        Inst::Dup { idx: 0 }, // duplicate the address
        Inst::GlobalSet {
            global_idx: pub_outputs_addr_idx,
        }, // set the address
        Inst::GlobalGet {
            global_idx: pub_outputs_addr_orig_idx,
        }, // get the original(start) address
        Inst::I32Sub, // get the number of public outputs * type size for while to continue (if > 0)
    ];
    Func::new(
        LOAD_PUB_OUTPUTS_ON_STACK_FUNC_NAME.to_string(),
        FuncType {
            params: vec![],
            results: vec![],
        },
        vec![],
        ins,
        HashMap::new(),
    )
}
