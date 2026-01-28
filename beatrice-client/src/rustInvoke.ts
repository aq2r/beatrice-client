import * as tauri from "@tauri-apps/api/core";

const cpal = {
  getInputs: async () => {
    return await tauri.invoke<string[]>("cpal_get_inputs");
  },
  getOutputs: async () => {
    return await tauri.invoke<string[]>("cpal_get_outputs");
  },

  setInputGain: async (gain: number) => {
    await tauri.invoke<void>("cpal_set_input_gain", { gain: gain });
  },
  setOutputGain: async (gain: number) => {
    await tauri.invoke<void>("cpal_set_output_gain", { gain: gain });
  },
  setMonitorGain: async (gain: number) => {
    await tauri.invoke<void>("cpal_set_monitor_gain", { gain: gain });
  },
  setInputThreshold: async (threshold: number) => {
    await tauri.invoke<void>("cpal_set_input_threshold", {
      threshold: threshold,
    });
  },

  startVoiceChanger: async (
    modelPath: string,
    inputDeviceName: string | null,
    outputDeviceName: string | null,
    monitorDeviceName: string | null,
  ) => {
    await tauri.invoke<void>("cpal_start_voice_changer", {
      modelPath: modelPath,
      inputDeviceName: inputDeviceName,
      outputDeviceName: outputDeviceName,
      monitorDeviceName: monitorDeviceName,
    });
  },
};

export interface BeatriceVoiceInfo {
  name: string;
  description: string;
  average_pitch: number;
  portrait_path: string | null;
  portrait_description: string | null;
}

export interface BeatriceModelInfo {
  model_path: string;
  version: string;
  name: string;
  description: string;

  voices: BeatriceVoiceInfo[];
}

const beatrice = {
  getModelFromPath: async (modelFolder: string) => {
    return await tauri.invoke<BeatriceModelInfo | null>(
      "beatrice_get_model_from_path",
      { modelFolder: modelFolder },
    );
  },

  getNSpeaker: async () => {
    return await tauri.invoke<number | null>("beatrice_get_nspeaker");
  },

  setTargetSpeaker: async (target: number) => {
    return await tauri.invoke<null>("beatrice_set_target_speaker", {
      target: target,
    });
  },

  getVersion: async () => {
    return await tauri.invoke<string | null>("beatrice_get_version");
  },

  setPitch: async (pitch: number) => {
    await tauri.invoke<null>("beatrice_set_pitch", { pitch: pitch });
  },

  setFormantShift: async (formant: number) => {
    await tauri.invoke<null>("beatrice_set_formant_shift", {
      formant: formant,
    });
  },

  setIntonationIntensity: async (intonationIntensity: number) => {
    await tauri.invoke<null>("beatrice_set_intonation_intensity", {
      intonationIntensity: intonationIntensity,
    });
  },

  setAverageSourcePitch: async (averageSourcePitch: number) => {
    await tauri.invoke<null>("beatrice_set_average_source_pitch", {
      averageSourcePitch: averageSourcePitch,
    });
  },

  setMinSourcePitch: async (minSourcePitch: number) => {
    await tauri.invoke<null>("beatrice_set_min_source_pitch", {
      minSourcePitch: minSourcePitch,
    });
  },

  setMaxSourcePitch: async (maxSourcePitch: number) => {
    await tauri.invoke<null>("beatrice_set_max_source_pitch", {
      maxSourcePitch: maxSourcePitch,
    });
  },

  setVqNumNeighbors: async (vqNumNeighbors: number) => {
    await tauri.invoke<null>("beatrice_set_vq_num_neighbors", {
      vqNumNeighbors: vqNumNeighbors,
    });
  },
};

export const rustInvoke = {
  cpal: cpal,
  beatrice: beatrice,
};
