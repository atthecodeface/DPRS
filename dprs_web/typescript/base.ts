import { Log, Logger } from "./log.js";
import { Visualize } from "./visualize.js";
import { VisualizeControls } from "./visualize_controls.js";
import { JsSimulation } from "./js_simulation.js";
import { JsParameters } from "./js_parameters.js";
import { Presettable, SimulationControls } from "./simulation_controls.js";

export class MainBase implements Presettable {
  log: Logger;
  simulation: JsSimulation;
  visualize: Visualize;
  visualize_controls: VisualizeControls;
  simulation_controls: SimulationControls;
  presets: [string, string][] = [];
  default_preset_value = null;

  constructor(
    logger: Log,
    model: string,
    dim: number,
    zoom: number | null = null,
    do_rough_background: boolean | null = null,
  ) {
    this.log = new Logger(logger, `${model}_${dim}d`);
    this.log.push_reason("init");
    this.log.info("Starting");

    this.simulation = new JsSimulation(logger);
    this.simulation_controls = new SimulationControls(
      `${dim}d_sc_`,
      `${dim}d_sim_controls`,
      dim,
      this,
    );
    this.simulation_controls.parameters = this.get_default_parameters();
    this.simulation_controls.populate_webpage_entries();

    this.visualize = new Visualize(logger, this.simulation, "Visualize");
    this.visualize_controls = new VisualizeControls(
      logger,
      this.visualize,
      this.visualize,
      "VisualizationControls",
    );
    if (do_rough_background != null) {
      this.visualize.do_rough_background = do_rough_background!;
    }
    if (zoom != null) {
      this.visualize.set_zoom(zoom!);
    }

    this.log.info("HTML built, running initial simulation");
    this.run_simulation(dim);

    this.log.info("Initialization complete");
    this.log.pop_reason();
  }

  run_simulation(dim: number) {
    this.log.push_reason("sim");
    this.log.info(`Running simulation of dimension ${dim}`);

    this.simulation_controls.set_parameters_from_webpage_entries();
    if (dim <= 1) {
      this.simulation_controls.parameters.dimensions.n_y = 1;
    }
    if (dim <= 2) {
      this.simulation_controls.parameters.dimensions.n_z = 1;
    }

    const sim_parameters = this.simulation_controls.parameters;
    this.simulation.run(sim_parameters);
    this.log.info(
      `Simulation complete with ${this.simulation.n_results()} results`,
    );

    this.visualize_controls.set_parameters_from_webpage_entries(
      this.simulation,
    );
    this.visualize.set_redraw(this.simulation_controls);
    this.visualize.redraw();

    this.log.pop_reason();
  }

  get_default_parameters(): JsParameters {
    const p = new JsParameters();
    return p;
  }

  select_preset(preset: string): void {
    var p = this.get_default_parameters();
    this.simulation_controls.parameters = p;
    this.simulation_controls.populate_webpage_entries();
  }
}
