use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract("mxsc:output/world-debt-solution.mxsc.json", world_debt_solution::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/world_debt_solution.scen.json");
}
