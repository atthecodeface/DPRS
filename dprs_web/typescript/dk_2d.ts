import init from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";
import { JsParameters } from "./js_parameters.js";
import { MainSim, MainBase } from "./base.js";

class Main implements MainSim {
  preset_labels = [];
  model_name = "DomanyKinzel";
  dim = 2;
  zoom = null;
  do_rough_background = null;
  default_preset = 0;
  select_preset = null;

  main: MainBase;
  constructor(logger: Log) {
    this.main = new MainBase(this, logger);
    // console.log(`${model} ${dim}d child class`);
  }

  get_default_parameters(): JsParameters {
    const p = new JsParameters();

    p.dimensions.n_x = 350;
    p.dimensions.n_y = 200;
    p.dimensions.n_z = 1;

    p.settings.n_iterations = 500;
    p.settings.sample_period = 1;
    p.settings.random_seed = 6;
    p.settings.initial_seeding = "center";
    p.settings.growth_model = "DomanyKinzel";
    p.settings.growth_scheme = "Staggered";

    p.probabilities.p_1 = 0.38; //0.70548515
    p.probabilities.p_2 = 0.38;
    p.probabilities.p_conj = 0.0;
    p.probabilities.p_nbr = 0.0;
    p.probabilities.p_diag = 0.0;
    p.probabilities.u_x = 0.0;
    p.probabilities.p_initial = 0.5;

    p.preset = 0;

    return p;
  }
}

(window as any).main = null;
function complete_init() {
  const window_log = new Log("Log");
  const main = new Main(window_log); // , window.location.search
  (window as any).log = window_log;
  (window as any).main = main;
}

window.addEventListener("load", (e) => {
  init().then(() => {
    complete_init();
  });
});
