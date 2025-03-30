import { invoke } from "@tauri-apps/api/core";

export class RustInvoke {
    static cpalGetInputs = async (): Promise<string[]> => {
        return await invoke<string[]>("cpal_get_inputs");
    }

    static cpalGetOutputs = async (): Promise<string[]> => {
        return await invoke<string[]>("cpal_get_outputs");
    }

    static cpalStartVoiceChanger = async (inputDeviceName: string | null, outputDeviceName: string | null): Promise<void> => {
        await invoke<void>("cpal_start_voice_changer",
            { inputDeviceName: inputDeviceName, outputDeviceName: outputDeviceName }
        );
    }

    static beatriceLoadModel = async (modelPath: string) => {
        try {
            await invoke<void>("beatrice_load_model", { modelPath: modelPath })
        } catch (err) {
            console.log(err)
        }
    }

    static beatriceSetPitch = async (pitch: number) => {
        await invoke<void>("beatrice_set_pitch", { pitch: pitch })
    }
}

