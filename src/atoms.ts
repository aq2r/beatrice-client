import { Store as TauriStore } from "@tauri-apps/plugin-store";
import { atom } from "jotai";
import { BeatriceModelInfo } from "./rustInvoke";

export const jotaiAtoms = {
    tauriStore: atom<TauriStore | null>(null),

    allModelInfo: atom<BeatriceModelInfo[]>([]),
    selectModel: atom<BeatriceModelInfo | null>(null),
    selectSpeakerIdx: atom<number>(0),

    loadedModelVersion: atom<string | null>(null),

    pitch: atom<number>(0.0),
    formantShift: atom<number>(0.0),
    inputGain: atom<number>(1.0),
    outputGain: atom<number>(1.0),

    isDisplayAdvancedSettings: atom<boolean>(false),
    averageSourcePitch:  atom<number>(52.0),
    intonationIntensity:  atom<number>(1.0),
    pitchCorrection:  atom<number>(0.0),
    pitchCorrectionType:  atom<number>(0),
    minSourcePitch:  atom<number>(33.125),
    maxSourcePitch:  atom<number>(80.875),
    vqNumNeighbors:  atom<number>(0),

    inputDevices: atom<string[]>([]),
    outputDevices: atom<string[]>([]),
    monitorDevices: atom<string[]>([]),

    selectInputDevice: atom<string | null>(null),
    selectOutputDevice: atom<string | null>(null),
    selectMonitorDevice: atom<string | null>(null),
}