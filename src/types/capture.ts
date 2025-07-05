export interface CaptureSource {
  name: string;
  width: number;
  height: number;
  source_type: CaptureSourceType;
  handle: number; // HWND or HMONITOR
  left: number;
  top: number;
}

export enum CaptureSourceType {
  Monitor = 'monitor',
  Window = 'window',
}

export interface MonitorInfo {
  id: number;
  hmonitor: number;
  name: string;
  width: number;
  height: number;
  left: number;
  top: number;
}

export interface WindowInfo {
  id: number;
  hwnd: number;
  title: string;
  width: number;
  height: number;
}

export interface Webcam {
  name: string;
  id: string;
  width: number;
  height: number;
}
