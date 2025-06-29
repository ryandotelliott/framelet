export interface CaptureSource {
  handle: number;
  name: string;
  width: number;
  height: number;
  source_type: string;
}

export interface Region {
  x: number;
  y: number;
  width: number;
  height: number;
}
