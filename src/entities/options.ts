export type Options = {
  resizeMin: number;
  resizeMax: number;
  rotateMaxDeg: number;
  brightnessRange: number;
  contrastRange: number;
  noiseSigma: number;
  jpegQuality: number;
  webpQuality: number;
  stripExif: boolean;
};

export const defaultOptions: Options = {
  resizeMin: 0.9,
  resizeMax: 1.1,
  rotateMaxDeg: 2,
  brightnessRange: 5,
  contrastRange: 5,
  noiseSigma: 1,
  jpegQuality: 90,
  webpQuality: 90,
  stripExif: true,
};
