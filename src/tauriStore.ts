export const tauriStoreDeviceKey = "tauriStoreDevice";
export interface TauriStoreDevice {
    input: string | null,
    output: string | null,
    monitor: string | null,
}

export const tauriStoreModelPathKey = "tauriStoreModelPath";

export const tauriStorSlidersKey = "tauriStorSliders";
export interface TauriStoreSliders {
    pitch: number,
    inputGain: number,
    outputGain: number,
    formantShift: number,
}