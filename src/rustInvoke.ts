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

    static startVoiceChanger = async (inputDeviceName: string | null, outputDeviceName: string | null): Promise<void> => {
        await invoke<void>("cpal_start_voice_changer",
            { inputDeviceName: inputDeviceName, outputDeviceName: outputDeviceName }
        );
    }
}

class Beatrice {
    static loadModel = async (modelPath: string) => {
        try {
            await invoke<void>("beatrice_load_model", { modelPath: modelPath })
        } catch (err) {
            console.log(err)
        }
    }

    static setPitch = async (pitch: number) => {
        await invoke<void>("beatrice_set_pitch", { pitch: pitch })
    }

    static setFormantShift = async (formant: number) => {
        await invoke<void>("beatrice_set_formant_shift", { formant: formant })
    }

    static setAverageSourcePitch = async (average_source_pitch: number) => {
        await invoke<void>("beatrice_set_average_source_pitch", {
            average_source_pitch: average_source_pitch
        })
    }

    static setIntonationIntensity = async (intonation_intensity: number) => {
        await invoke<void>("beatrice_set_intonation_intensity", {
            intonation_intensity: intonation_intensity
        })

    }

    static setPitchCorrection = async (pitch_correction: number) => {
        await invoke<void>("beatrice_set_pitch_correction", {
            pitch_correction: pitch_correction
        })
    }

    static setPitchCorrectionType = async (pitch_correction_type: number) => {
        await invoke<void>("beatrice_set_pitch_correction_type", {
            pitch_correction_type: pitch_correction_type
        })
    }
}

export class RustInvoke {
    static Cpal = Cpal
    static Beatrice = Beatrice
}
