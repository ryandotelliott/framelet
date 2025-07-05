export interface Region {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface RecordingConfig {
  outputPath: string;
  region?: Region;
}

export interface ScreenRecorder {
  isRecording: boolean;
  startTime?: Date;
  duration?: number;
}
