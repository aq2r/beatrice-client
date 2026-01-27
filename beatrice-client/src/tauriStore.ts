export const tauriStoreKey = "tauriStoreKey";
export interface TauriStoreInterface {
  modelFolderPaths: string[];

  inputDevice: string | null;
  outputDevice: string | null;
  monitorDevice: string | null;

  pitch: number | null;
  formantShift: number | null;
  intonationIntensity: number | null;
  minSourcePitch: number | null;
  maxSourcePitch: number | null;
  vqNumNeighbors: number | null;

  inputGain: number | null;
  outputGain: number | null;
  monitorGain: number | null;
  inputThreshold: number | null;
}
