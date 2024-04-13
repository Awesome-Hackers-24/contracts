use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("mxsc:output/brain-nfts.mxsc.json", brain_nfts::ContractBuilder);
    blockchain
}

#[test]
fn adder_rs() {
    world().run("scenarios/brain_nfts.scen.json");
}

#[test]
fn interactor_trace_rs() {
    world().run("scenarios/interactor_trace.scen.json");
}
