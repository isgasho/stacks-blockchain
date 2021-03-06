use std::convert::TryInto;
use vm::{Value, apply, eval_all};
use vm::representations::{SymbolicExpression};
use vm::errors::{InterpreterResult as Result};
use vm::callables::CallableType;
use vm::contexts::{Environment, LocalContext, ContractContext, GlobalContext};
use vm::ast::ContractAST;
use vm::types::QualifiedContractIdentifier;

#[derive(Serialize, Deserialize)]
pub struct Contract {
    pub contract_context: ContractContext,
}

// AARON: this is an increasingly useless wrapper around a ContractContext struct.
//          will probably be removed soon.
impl Contract {
    pub fn initialize_from_ast (contract_identifier: QualifiedContractIdentifier, contract: &ContractAST, global_context: &mut GlobalContext) -> Result<Contract> {
        let mut contract_context = ContractContext::new(contract_identifier);

        eval_all(&contract.expressions, &mut contract_context, global_context)?;

        Ok(Contract { contract_context: contract_context })
    }

}
