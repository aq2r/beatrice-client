import { atom } from "jotai";
import { BeatriceModelInfo } from "./rustInvoke";

interface VoiceSetting {
  pitch: number;
  formant: number;
  intonationIntensity: number;
  minSourcePitch: number;
  maxSourcePitch: number;
  vqNeighborCount: number;
}

interface OutputSetting {
  inputGain: number;
  outputGain: number;
  monitorGain: number;
  inputThreshold: number;
}

interface DeviceSetting {
  input: string | null;
  output: string | null;
  monitor: string | null;
}

export const jotaiAtoms = {
  loadedModels: atom<BeatriceModelInfo[]>([]),
  selectModel: atom<BeatriceModelInfo | null>(null),
  selectSpeakerIdx: atom<number>(0),

  inputDevices: atom<string[]>([]),
  outputDevices: atom<string[]>([]),

  voiceSetting: atom<VoiceSetting>({
    pitch: 0.0,
    formant: 0.0,
    intonationIntensity: 1.0,
    minSourcePitch: 33.125,
    maxSourcePitch: 88.875,
    vqNeighborCount: 0,
  }),

  outputSetting: atom<OutputSetting>({
    inputGain: 1.0,
    outputGain: 1.0,
    monitorGain: 1.0,
    inputThreshold: 0.0,
  }),

  deviceSetting: atom<DeviceSetting>({
    input: null,
    output: null,
    monitor: null,
  }),
};
