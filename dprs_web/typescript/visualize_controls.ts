import * as html from "./html.js";
import { Visualize } from "./visualize.js";
import { JsSimulation } from "./js_simulation.js";
import { Log, Logger } from "./log.js";

export interface VisualizationControlClient {
  /** Play or pause/stop an animated simulation display
   *
   * @param {number} fps The frames-per-second to set replay to; if this is 0,
   *                     then stop; if this is -ve then a backwards animation is desired
   */
  animation_stop(): void;
  animation_start(time: number): void;
  playback_simulation(fps: number): void;
  set_zoom(zoom: number): void;
  set_slice(slice: number): void;
  // Functions replacing slow back/forward playback
  decrement_slice(): void;
  increment_slice(): void;
  // Allows toggling of playback
  get_animation_state(): boolean;
  // redraw(): void;
}

export class VisualizeControls {
  /**
   * Parent of this widget
   */
  parent: VisualizationControlClient;

  /**
   * Logger to report progress to (as a source of 'sim')
   */
  log: Logger;

  /**
   * Parent of this widget
   */
  visualize: Visualize;

  /**
   * The HtmlElement containing the HTMLDivElement that this populates
   */
  div: html.HtmlElement;
  // Used in populate
  td_slice?: html.HtmlElement;
  td_playback?: html.HtmlElement;

  constructor(
    logger: Log,
    parent: VisualizationControlClient,
    visualize: Visualize,
    div_id: string,
  ) {
    this.parent = parent;
    this.log = new Logger(logger, "vis_control");
    this.visualize = visualize;

    const div = document.getElementById(div_id);
    if (!div) {
      throw new Error(
        `Failed to find ${div_id} to build VisualizationControls`,
      );
    }
    this.div = new html.HtmlElement(div);

    this.build_html();
  }

  build_html() {
    this.div.clear();

    const table = this.div.add_ele("table");
    const zoom_table = table
      .add_ele("tr")
      .add_ele("td")
      .add_ele("table", { classes: "zoom" });
    const playback_table = table
      .add_ele("tr")
      .add_ele("td")
      .add_ele("table", { classes: "playback" });

    const tr_zoom = zoom_table.add_ele("tr", { classes: "zoom_slice" });
    tr_zoom
      .add_ele("td", { classes: "label" })
      .add_label("zoom")
      .set_content("Zoom");
    tr_zoom.add_ele("td").add_input_range(
      "zoom",
      { min: 1, max: 5, step: 0.1 },
      (_e: Event, value) => {
        this.parent.set_zoom(value);
      },
      { id: "zoom" },
    );

    this.td_playback = playback_table;

    const tr_slice = zoom_table.add_ele("tr", { classes: "zoom_slice" });
    this.td_slice = tr_slice;

    tr_slice
      .add_ele("td", { classes: "label" })
      .add_label("slice")
      .set_content("Time slice");
    tr_slice.add_ele("td").add_input_range(
      "slice",
      { min: 0, max: 1, step: 1 },
      (_e: Event, value) => {
        this.parent.set_slice(value);
      },
      { id: "slice" },
    );

    const fps = 120;

    const tr_playback = playback_table.add_ele("tr", {
      classes: "playback",
    });
    // ⏮ ⏪⏸⏩⏭ (Add #fe0e to make them plain)⏸️#fe0e #fe0e  ⏯#fe0e
    // Turning off by hand because I can't turn it off in CSS
    tr_playback.add_ele("td").add_input_button(
      "⏪︎",
      () => {
        this.parent.playback_simulation(-fps);
      },
      { classes: "controls playback reverse" },
    );
    tr_playback.add_ele("td").add_input_button(
      // "⏸︎",
      "⏸︎",
      () => {
        this.parent.playback_simulation(0);
      },
      { classes: "controls playback pause" },
    );
    tr_playback.add_ele("td").add_input_button(
      "⏹︎",
      () => {
        this.parent.playback_simulation(fps);
      },
      { classes: "controls playback play" },
    );
    tr_playback.add_ele("td").add_input_button(
      // "⏵︎",
      "⏯︎",
      () => {
        if (this.parent.get_animation_state()) {
          // this.parent.playback_simulation(0);
          this.parent.animation_stop();
        } else {
          // this.parent.playback_simulation(fps);
          this.parent.animation_start(0);
        }
      },
      { classes: "controls playback pauseplay" },
    );
    tr_playback.add_ele("td").add_input_button(
      // "⏴︎",
      "➖",
      () => {
        // Step backward by one iteration: replaces slow reverse playback
        // this.parent.playback_simulation(-10);
        this.parent.decrement_slice();
      },
      { classes: "controls playback decrement" },
    );
    tr_playback.add_ele("td").add_input_button(
      // "⏵",
      "➕",
      () => {
        // Step forward by one iteration: replaces slow forward playback
        // this.parent.playback_simulation(10);
        this.parent.increment_slice();
      },
      { classes: "controls playback increment" },
    );
  }

  populate_values(simulation: JsSimulation) {
    if (simulation.dim < 2) {
      this.td_slice!.set_style("display", "none");
      this.td_playback!.set_style("display", "none");
    } else {
      this.td_slice!.set_style("display");
      this.td_playback!.set_style("display");
    }
    this.visualize.scale = html.get_input_float("zoom", 1, 5);
    html.set_input_range("slice", 0, simulation.n_results() - 1);
    this.visualize.slice = html.get_input_int(
      "slice",
      simulation.n_results() * 0,
      simulation.n_results() - 1,
    );
    // CPS mod: I want to, perhaps, start with a non-zero time slice
    //          so the user can actually see the demo is *doing* something.
    //          Again, not ideal, but ^shrug^.
    // html.set_input_value("zoom", 2);
    // html.set_input_value("slice", simulation.n_results() / 2);
    // this.visualize.redraw();
  }
}
