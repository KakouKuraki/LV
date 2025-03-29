export type Tracks = {
  tracks: Track[];
  frameWidth: number;
  isMaxZoom: boolean;
  labelWidth: number;
};

export type Track = {
  id: number;
  items: Item[];
  frameWidth: number;
  isMaxZoom: boolean;
  labelWidth: number;
};

export type Label = {
  label_str: string;
  labelWidth: number;
};

export type Item = {
  start: number;
  end: number;
  vstart: number;
  vend: number;
  label: string;
  frameWidth: number;
  labelWidth: number;
};

