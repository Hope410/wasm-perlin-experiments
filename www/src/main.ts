import { FlowView, FlowModel, NoiseModel } from "wasm-pkg";
import _ from "lodash";

const CANVAS_WIDTH = 768;
const CANVAS_HEIGHT = 480;

function render(flowView: FlowView) {
  flowView.render_vector_field();
}

(function main() {
  const canvas: HTMLCanvasElement | null = document.querySelector("#canvas");
  if (!canvas) {
    throw new Error("Can't get canvas element");
  }

  canvas.width = CANVAS_WIDTH;
  canvas.height = CANVAS_HEIGHT;

  const ctx = canvas.getContext("2d");

  if (!ctx) {
    throw new Error("Can't get canvas 2D context");
  }

  const frequency = 5;
  const lacunarity = 2;
  const gain = 0.1;
  const octaves = 1;

  const vectorSize = 16;

  const noiseModel = NoiseModel.new(frequency, lacunarity, gain, octaves);
  const flowModel = FlowModel.new(noiseModel, vectorSize, CANVAS_WIDTH, CANVAS_HEIGHT);
  const flowView = FlowView.new(ctx, flowModel);

  render(flowView);
})();
