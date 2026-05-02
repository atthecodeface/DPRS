import init from "../pkg/dprs_wasm.js";
import { Log } from "./log.js";
import { MainBase } from "./base.js";
class Main extends MainBase {
    constructor(logger, _) {
        super(logger, _);
        console.log("Bedload 2d child class");
    }
    get_presets() {
        return [
            ["0", "User"],
            ["1", "A"],
            ["2", "B"],
            ["3", "C"],
            ["4", "D"],
            ["5", "E"],
        ];
    }
    enact_preset(preset) {
        var p = this.get_default_parameters();
        switch (preset) {
            case 0:
                p.preset = 0;
                return;
            case 1:
                p.preset = 1;
                p.probabilities.p_1 = 0.61487;
                p.probabilities.p_2 = 0.9;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 5;
                p.settings.n_iterations = 2000;
                break;
            case 2:
                p.preset = 2;
                p.probabilities.p_1 = 0.8135;
                p.probabilities.p_2 = 0.5;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 31;
                p.settings.n_iterations = 2000;
                break;
            case 3:
                p.preset = 3;
                p.probabilities.p_1 = 0.8945;
                p.probabilities.p_2 = 0.3;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 6;
                p.settings.n_iterations = 2000;
                break;
            case 4:
                p.preset = 4;
                p.probabilities.p_1 = 0.96693;
                p.probabilities.p_2 = 0.1;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 4;
                p.settings.n_iterations = 2000;
                break;
            case 5:
                p.preset = 5;
                p.probabilities.p_1 = 0.99677;
                p.probabilities.p_2 = 0.01;
                p.probabilities.p_conj = 1e-5;
                p.settings.random_seed = 1;
                p.settings.n_iterations = 2000;
                break;
            default:
                break;
        }
        this.simulation_controls.parameters = p;
        this.simulation_controls.populate_values();
    }
}
window.main = null;
function complete_init() {
    const window_log = new Log("Log");
    const main = new Main(window_log, window.location.search);
    window.log = window_log;
    window.main = main;
}
window.addEventListener("load", (e) => {
    init().then(() => {
        complete_init();
    });
});
