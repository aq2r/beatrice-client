export const tauriStoreDeviceKey = "tauriStoreDevice";
export interface TauriStoreDevice {
    input: string | null,
    output: string | null,
    monitor: string | null,
}

export const tauriStoreModelPathKey = "tauriStoreModelPath";

export const tauriStorSlidersKey = "tauriStoreSliders";
export interface TauriStoreSliders {
    pitch: number | null,
    inputGain: number | null,
    outputGain: number | null,
    formantShift: number | null,
}