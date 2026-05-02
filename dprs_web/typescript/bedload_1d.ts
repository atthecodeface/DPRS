import init from "../pkg/dprs_wasm.js";
import { Log, Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { SimulationControls } from "./simulation_controls.js";

class Main {
  log: Logger;
  simulation: JsSimulation;
  visualize: Visualize;
  visualize_controls: VisualizeControls;
  simulation_controls: SimulationControls;

  constructor(logger: Log, _: string) {
    const dim = 1;
    this.log = new Logger(logger, `bedload_${dim}d`);
    this.log.push_reason("init");
    this.log.info("Starting");

    this.simulation = new JsSimulation(logger);
    this.visualize = new Visualize(logger, this.simulation, "Visualize");
    this.visualize_controls = new VisualizeControls(
      logger,
      this.visualize,
      this.visualize,
      "VisualizationControls",
    );
    this.visualize.do_rough_background = false;

    this.simulation_controls = new SimulationControls(
      `${dim}d_sc_`,
      `${dim}d_sim_controls`,
      dim,
      this.get_presets(),
    );
    this.simulation_controls.parameters = this.get_default_parameters();
    this.simulation_controls.populate_values();
    this.simulation_controls.set_bedload()

    this.log.info("HTML built, running initial simulation");
    this.run_simulation(1);

    this.log.info("Initialization complete");
    this.log.pop_reason();
  }

  run_simulation(dim: number) {
    this.log.push_reason("sim");
    this.log.info(`Running simulation of dimension ${dim}`);

    this.simulation_controls.populate_parameters();
    this.simulation_controls.parameters.dimensions.n_y = 1;
    this.simulation_controls.parameters.dimensions.n_z = 1;

    const sim_parameters = this.simulation_controls.parameters;
    this.simulation.run(sim_parameters);
    this.log.info(
      `Simulation complete with ${this.simulation.n_results()} results`,
    );

    this.visualize_controls.populate_values(this.simulation);
    this.visualize.set_redraw(this.simulation_controls);
    this.visualize.redraw();

    this.log.pop_reason();
  }

  get_default_parameters(): JsParameters {
    const p = new JsParameters();

    p.probabilities.p_1 = 0.64;
    p.probabilities.p_2 = 0.64;
    p.probabilities.p_conj = 0.0;
    p.probabilities.p_nbr = 0.0;
    p.probabilities.p_diag = 0.0;
    p.probabilities.u_x = 0.0;

    p.probabilities.p_initial = 0.5;

    p.simsettings.n_iterations = 300;
    p.simsettings.sample_period = 1;
    p.simsettings.random_seed = 1;

    p.dimensions.n_x = 350;
    p.dimensions.n_y = 1;
    p.dimensions.n_z = 1;

    p.simsettings.seed_kind = "random";
    p.simsettings.simulation_kind = "bedload";
    return p;
  }

  get_presets(): any | null {
    return null;
  }

  enact_preset(preset: number): void {
  }
}


(window as any).main = null;
function complete_init() {
  const window_log = new Log("Log");
  const main = new Main(window_log, window.location.search);
  (window as any).log = window_log;
  (window as any).main = main;
}

window.addEventListener("load", (e) => {
  init().then(() => {
    complete_init();
  });
});
