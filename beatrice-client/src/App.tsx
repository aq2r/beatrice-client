import { Dispatch, SetStateAction, useEffect, useState } from "react";
import "./App.css";
import { SelectModel } from "./components/mycomponent/modelSelect";
import { VoiceSettings } from "./components/mycomponent/voiceSettings";
import { useAtom } from "jotai";
import { jotaiAtoms } from "./jotaiAtoms";
import { rustInvoke } from "./rustInvoke";
import * as tauriStore from "@tauri-apps/plugin-store";
import { TauriStoreInterface, tauriStoreKey } from "./tauriStore";

function App() {
  const [isLoadStore, setIsLoadStore] = useState(false);

  useEffect(() => {
    const handler = (e: MouseEvent) => {
      e.preventDefault();
    };

    document.addEventListener("contextmenu", handler);

    return () => {
      document.removeEventListener("contextmenu", handler);
    };
  }, []);

  return (
    <div className="select-none flex w-screen h-screen bg-neutral-900 overflow-hidden">
      <SelectModel />
      <VoiceSettings />

      <InitGrobal />
      <SaveTauriStore isLoadStore={isLoadStore} />
      <LoadTauriStore setIsLoadStore={setIsLoadStore} />
      <SyncBeatrice />
    </div>
  );
}

export default App;

function InitGrobal() {
  // 使用できる入力・出力デバイスを取得
  const [, setInputDevices] = useAtom(jotaiAtoms.inputDevices);
  const [, setOutputDevices] = useAtom(jotaiAtoms.outputDevices);
  useEffect(() => {
    const promise = async () => {
      setInputDevices(await rustInvoke.cpal.getInputs());
      setOutputDevices(await rustInvoke.cpal.getOutputs());
    };
    promise();
  }, []);

  return <></>;
}

function SaveTauriStore({ isLoadStore }: { isLoadStore: boolean }) {
  const [loadedModels] = useAtom(jotaiAtoms.loadedModels);
  const [voiceSetting] = useAtom(jotaiAtoms.voiceSetting);
  const [outputSetting] = useAtom(jotaiAtoms.outputSetting);
  const [deviceSetting] = useAtom(jotaiAtoms.deviceSetting);

  useEffect(() => {
    if (isLoadStore) {
      const promise = async () => {
        const store = await tauriStore.load("store.json");

        const storeValue: TauriStoreInterface = {
          modelFolderPaths: loadedModels.map((i) => i.model_path),

          inputDevice: deviceSetting.input,
          outputDevice: deviceSetting.output,
          monitorDevice: deviceSetting.monitor,

          pitch: voiceSetting.pitch,
          formantShift: voiceSetting.formant,
          intonationIntensity: voiceSetting.intonationIntensity,
          minSourcePitch: voiceSetting.minSourcePitch,
          maxSourcePitch: voiceSetting.maxSourcePitch,
          vqNumNeighbors: voiceSetting.vqNeighborCount,

          inputGain: outputSetting.inputGain,
          outputGain: outputSetting.outputGain,
          monitorGain: outputSetting.monitorGain,
          inputThreshold: outputSetting.inputThreshold,
        };

        await store.set(tauriStoreKey, storeValue);
        await store.save();
      };
      promise();
    }
  }, [loadedModels, voiceSetting, outputSetting, deviceSetting, isLoadStore]);

  return <></>;
}

function LoadTauriStore({
  setIsLoadStore,
}: {
  setIsLoadStore: Dispatch<SetStateAction<boolean>>;
}) {
  const [, setLoadedModels] = useAtom(jotaiAtoms.loadedModels);
  const [, setVoiceSetting] = useAtom(jotaiAtoms.voiceSetting);
  const [, setOutputSetting] = useAtom(jotaiAtoms.outputSetting);
  const [, setDeviceSetting] = useAtom(jotaiAtoms.deviceSetting);

  useEffect(() => {
    const promise = async () => {
      const store = await tauriStore.load("store.json");
      const storeValue = await store.get<TauriStoreInterface>(tauriStoreKey);

      const models = await Promise.all(
        storeValue?.modelFolderPaths.map((path) =>
          rustInvoke.beatrice.getModelFromPath(path),
        ) ?? [],
      );

      const nonNullModels = models.filter((m) => m !== null);
      setLoadedModels(nonNullModels);

      setVoiceSetting({
        pitch: storeValue?.pitch ?? 0.0,
        formant: storeValue?.formantShift ?? 0.0,
        intonationIntensity: storeValue?.intonationIntensity ?? 1.0,
        minSourcePitch: storeValue?.minSourcePitch ?? 33.125,
        maxSourcePitch: storeValue?.maxSourcePitch ?? 88.875,
        vqNeighborCount: storeValue?.vqNumNeighbors ?? 0,
      });

      setOutputSetting({
        inputGain: storeValue?.inputGain ?? 1.0,
        outputGain: storeValue?.outputGain ?? 1.0,
        monitorGain: storeValue?.monitorGain ?? 1.0,
        inputThreshold: storeValue?.inputThreshold ?? 0.0,
      });

      setDeviceSetting({
        input: storeValue?.inputDevice ?? null,
        output: storeValue?.outputDevice ?? null,
        monitor: storeValue?.monitorDevice ?? null,
      });

      setIsLoadStore(true);
    };
    promise();
  }, []);

  return <></>;
}

// Beatrice側を更新する
function SyncBeatrice() {
  // デバイス・モデル
  const [selectModel] = useAtom(jotaiAtoms.selectModel);
  const [selectSpeakerIdx, setSelectSpeakerIdx] = useAtom(
    jotaiAtoms.selectSpeakerIdx,
  );
  const [deviceSetting] = useAtom(jotaiAtoms.deviceSetting);
  const [voiceSetting] = useAtom(jotaiAtoms.voiceSetting);
  const [outputSetting] = useAtom(jotaiAtoms.outputSetting);

  useEffect(() => {
    const promise = async () => {
      if (selectModel !== null) {
        await rustInvoke.cpal.startVoiceChanger(
          selectModel.model_path,
          deviceSetting.input,
          deviceSetting.output,
          deviceSetting.monitor,
        );

        await new Promise((resolve) => setTimeout(resolve, 100));

        setSelectSpeakerIdx(0);
        rustInvoke.beatrice.setTargetSpeaker(0);
        rustInvoke.beatrice.setPitch(voiceSetting.pitch);
        rustInvoke.beatrice.setFormantShift(voiceSetting.formant);
        rustInvoke.beatrice.setIntonationIntensity(
          voiceSetting.intonationIntensity,
        );
        rustInvoke.beatrice.setMinSourcePitch(voiceSetting.minSourcePitch);
        rustInvoke.beatrice.setMaxSourcePitch(voiceSetting.maxSourcePitch);
        rustInvoke.beatrice.setVqNumNeighbors(voiceSetting.vqNeighborCount);
      }
    };

    promise();
  }, [selectModel, deviceSetting]);

  // output設定
  useEffect(() => {
    rustInvoke.cpal.setInputGain(outputSetting.inputGain);
    rustInvoke.cpal.setOutputGain(outputSetting.outputGain);
    rustInvoke.cpal.setMonitorGain(outputSetting.monitorGain);
    rustInvoke.cpal.setInputThreshold(outputSetting.inputThreshold);

    rustInvoke.beatrice.setPitch(voiceSetting.pitch);
    rustInvoke.beatrice.setFormantShift(voiceSetting.formant);
    rustInvoke.beatrice.setIntonationIntensity(
      voiceSetting.intonationIntensity,
    );
    rustInvoke.beatrice.setMinSourcePitch(voiceSetting.minSourcePitch);
    rustInvoke.beatrice.setMaxSourcePitch(voiceSetting.maxSourcePitch);
    rustInvoke.beatrice.setVqNumNeighbors(voiceSetting.vqNeighborCount);
  }, [outputSetting]);

  // ピッチなど
  useEffect(() => {
    rustInvoke.beatrice.setPitch(voiceSetting.pitch);
    rustInvoke.beatrice.setFormantShift(voiceSetting.formant);
    rustInvoke.beatrice.setIntonationIntensity(
      voiceSetting.intonationIntensity,
    );
    rustInvoke.beatrice.setMinSourcePitch(voiceSetting.minSourcePitch);
    rustInvoke.beatrice.setMaxSourcePitch(voiceSetting.maxSourcePitch);
    rustInvoke.beatrice.setVqNumNeighbors(voiceSetting.vqNeighborCount);
  }, [voiceSetting]);

  // スピーカー変更時
  useEffect(() => {
    rustInvoke.beatrice.setTargetSpeaker(selectSpeakerIdx);
  }, [selectSpeakerIdx]);

  return <></>;
}
