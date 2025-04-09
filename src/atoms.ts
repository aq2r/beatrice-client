import { Store as TauriStore } from "@tauri-apps/plugin-store";
import { atom } from "jotai";
import { BeatriceModelInfo } from "./rustInvoke";

export const jotaiAtoms = {
    tauriStore: atom<TauriStore | null>(null),

    allModelInfo: atom<BeatriceModelInfo[]>([]),
    selectModel: atom<BeatriceModelInfo | null>(null),
    selectSpeakerIdx: atom<number>(0),

    pitch: atom<number>(0.0),
    formantShift: atom<number>(0.0),
    inputGain: atom<number>(1.0),
    outputGain: atom<number>(1.0),

    inputDevices: atom<string[]>([]),
    outputDevices: atom<string[]>([]),
    monitorDevices: atom<string[]>([]),

    selectInputDevice: atom<string | null>(null),
    selectOutputDevice: atom<string | null>(null),
    selectMonitorDevice: atom<string | null>(null),
}