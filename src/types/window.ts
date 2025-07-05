export interface WindowConfig {
  title: string;
  width: number;
  height: number;
  resizable: boolean;
  alwaysOnTop: boolean;
}

export interface RegionSelectorConfig extends WindowConfig {
  monitorHandle: number;
}
