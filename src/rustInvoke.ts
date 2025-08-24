import { invoke } from "@tauri-apps/api/core";


class Cpal {
    static getInputs = async () => {
        return await invoke<string[]>("cpal_get_inputs");
    }

    static getOutputs = async () => {
        return await invoke<string[]>("cpal_get_outputs");
    }

    static setInputGain = async (gain: number) => {
        await invoke<void>("cpal_set_input_gain", { gain: gain });
    }

    static setOutputGain = async (gain: number) => {
        await invoke<void>("cpal_set_output_gain", { gain: gain });
    }

    static startVoiceChanger = async (
        inputDeviceName: string | null,
        outputDeviceName: string | null,
        monitorDeviceName: string | null,
    ): Promise<void> => {
        await invoke<void>("cpal_start_voice_changer",
            {
                inputDeviceName: inputDeviceName,
                outputDeviceName: outputDeviceName,
                monitorDeviceName: monitorDeviceName
            }
        );
    }
}

export interface BeatriceVoiceInfo {
    name: string,
    description: string,
    average_pitch: number,
    portrait_path: string,
    portrait_description: string,
}

export interface BeatriceModelInfo {
    model_path: string,
    version: string,
    name: string,
    description: string,

    voices: BeatriceVoiceInfo[],
}

class Beatrice {
    static loadModel = async (modelPath: string) => {
        await invoke<void>("beatrice_load_model", { modelPath: modelPath })
    }

    static getModelVersion = async () => {
        return await invoke<string | null>("beatrice_get_version");
    }

    static setSpeaker = async (speakerIdx: number) => {
        await invoke<void>("beatrice_set_target_speaker", { target: speakerIdx })
    }

    static setPitch = async (pitch: number) => {
        await invoke<void>("beatrice_set_pitch", { pitch: pitch })
    }

    static setFormantShift = async (formant: number) => {
        await invoke<void>("beatrice_set_formant_shift", { formant: formant })
    }

    static setAverageSourcePitch = async (averageSourcePitch: number) => {
        await invoke<void>("beatrice_set_average_source_pitch", {
            averageSourcePitch: averageSourcePitch
        })
    }

    static setIntonationIntensity = async (intonationIntensity: number) => {
        await invoke<void>("beatrice_set_intonation_intensity", {
            intonationIntensity: intonationIntensity
        })
    }

    static setPitchCorrection = async (pitchCorrection: number) => {
        await invoke<void>("beatrice_set_pitch_correction", {
            pitchCorrection: pitchCorrection
        })
    }

    static setPitchCorrectionType = async (pitchCorrectionType: number) => {
        await invoke<void>("beatrice_set_pitch_correction_type", {
            pitchCorrectionType: pitchCorrectionType
        })
    }

    static setMinSourcePitch = async (minSourcePitch: number) => {
        await invoke<void>("beatrice_set_min_source_pitch", {
            minSourcePitch: minSourcePitch
        })
    }

    static setMaxSourcePitch = async (maxSourcePitch: number) => {
        await invoke<void>("beatrice_set_max_source_pitch", {
            maxSourcePitch: maxSourcePitch
        })
    }

    static setVqNumNeighbors = async (vqNumNeighbors: number) => {
        await invoke<void>("beatrice_set_vq_num_neighbors", {
            vqNumNeighbors: vqNumNeighbors
        })
    }

    static searchModel = async (modelPath?: string) => {
        if (modelPath) {
            return await invoke<BeatriceModelInfo[]>("beatrice_search_model", { inputFolderPath: modelPath });
        } else {
            return await invoke<BeatriceModelInfo[]>("beatrice_search_model");
        }
    }
}

export class RustInvoke {
    static Cpal = Cpal
    static Beatrice = Beatrice

    static otherReadImage = async (path: string) => {
        return await invoke<string>("other_read_image", { path: path });
    }
}
