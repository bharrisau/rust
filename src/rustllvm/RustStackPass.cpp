#include "rustllvm.h"

using namespace llvm;

void RustSafeStack::getAnalysisUsage(AnalysisUsage &AU) const {
  AU.setPreservesCFG();
  AU.addPreserved<MachineLoopInfo>();
  AU.addPreserved<MachineDominatorTree>();

  // Get pass ID for PEI
  StringRef SR("prologepilog");
  const void *pei_id = PassRegistry::getPassRegistry()->getPassInfo(SR)->getTypeInfo();
  AU.addRequiredID(pei_id);

  MachineFunctionPass::getAnalysisUsage(AU);
}

bool RustSafeStack::runOnMachineFunction(MachineFunction &Fn) {
  errs() << "Rust stack safety running\n";
  return false;
}

char RustSafeStack::ID = 0;
//static RegisterPass<RustSafeStack> X("rustsafestack", "Rust Stack Safety Pass", false, false);
