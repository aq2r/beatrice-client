import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { Slider } from "@/components/ui/slider";
import { useEffect, useState } from "react";
import { Progress } from "@/components/ui/progress";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Label } from "@/components/ui/label";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Card } from "@/components/ui/card";
import { useAtom } from "jotai";
import { jotaiAtoms } from "@/jotaiAtoms";
import * as tauriEvent from "@tauri-apps/api/event";
import * as tauriCore from "@tauri-apps/api/core";

function QuestionTooltip({ description }: { description: string }) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span className="flex items-center justify-center w-4 h-4 text-sm text-white bg-neutral-400 rounded-full select-none relative top-0">
          ?
        </span>
      </TooltipTrigger>
      <TooltipContent
        side="top"
        sideOffset={6}
        className=" text-white text-xs p-2 rounded shadow-lg"
      >
        {description}
      </TooltipContent>
    </Tooltip>
  );
}

function SelectDeviceChild({
  label,
  devices,
  value,
  onValueChange,
  description,
  placeholder,
}: {
  label: string;
  devices: string[];
  value: string | null;
  onValueChange: (v: string) => void;
  description: string;
  placeholder: string;
}) {
  return (
    <div className="flex flex-col gap-3 w-[33%]">
      <div className="flex gap-2">
        <Label>{label}</Label>
        <QuestionTooltip description={description} />
      </div>
      <Select
        value={value ?? undefined}
        onValueChange={onValueChange}
        defaultValue={undefined}
      >
        <SelectTrigger className="w-[90%] max-w-[90%]">
          <SelectValue placeholder={placeholder} />
        </SelectTrigger>

        <SelectContent>
          {["None", ...devices].map((i) => (
            <SelectItem value={i}>{i}</SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}

function SelectDevice() {
  const [inputDevices] = useAtom(jotaiAtoms.inputDevices);
  const [outputDevices] = useAtom(jotaiAtoms.outputDevices);
  const [deviceSetting, setDeviceSetting] = useAtom(jotaiAtoms.deviceSetting);

  return (
    <div className="flex justify-between">
      <SelectDeviceChild
        label="Input"
        devices={inputDevices}
        value={deviceSetting.input}
        onValueChange={(v) =>
          setDeviceSetting((prev) => ({
            ...prev,
            input: v,
          }))
        }
        description="ボイスチェンジャーに使用するマイクを選択します。"
        placeholder="Select Input"
      />
      <SelectDeviceChild
        label="Output"
        devices={outputDevices}
        value={deviceSetting.output}
        onValueChange={(v) =>
          setDeviceSetting((prev) => ({
            ...prev,
            output: v,
          }))
        }
        description="変換後の出力先のデバイスを選択します。"
        placeholder="Select Output"
      />
      <SelectDeviceChild
        label="Monitor"
        devices={outputDevices}
        value={deviceSetting.monitor}
        onValueChange={(v) =>
          setDeviceSetting((prev) => ({
            ...prev,
            monitor: v,
          }))
        }
        description="自分の声がどう変換されているか確認するためのデバイスを選択します。"
        placeholder="Select Monitor"
      />
    </div>
  );
}

function MicLevelMeter() {
  const [level, setLevel] = useState(0);

  useEffect(() => {
    let unlisten: (() => void) | null = null;

    const promise = async () => {
      unlisten = await tauriEvent.listen<number>("mic-level", (event) => {
        setLevel(event.payload);
      });
    };
    promise();

    return () => {
      if (unlisten !== null) {
        unlisten();
      }
    };
  });

  const progressValue = Math.min(Math.max(level * 100, 0), 100);

  return (
    <div className="w-full space-y-1">
      <div className="flex justify-between text-xs text-neutral-400">
        <span>Mic Level</span>
        <span>{level.toFixed(2)}</span>
      </div>

      <Progress value={progressValue} className="h-2 bg-neutral-700" />
    </div>
  );
}

export function SliderOption({
  label,
  description,
  value,
  setValue,
  min = 0,
  max = 100,
  step = 1,
}: {
  label: string;
  description: string;
  value: number;
  setValue: (v: number) => void;
  min?: number;
  max?: number;
  step?: number;
}) {
  return (
    <div className="flex flex-col gap-1 w-full">
      <div className="flex justify-between items-center">
        <div className="flex gap-2">
          <span className="text-sm font-medium text-white">{label}</span>
          <QuestionTooltip description={description} />
        </div>

        <span className="text-sm text-indigo-300">{value}</span>
      </div>

      <Slider
        value={[value]}
        onValueChange={(val) => setValue(val[0])}
        min={min}
        max={max}
        step={step}
        className="w-full"
      ></Slider>
    </div>
  );
}

function VoiceAccordion() {
  const [voiceSetting, setVoiceSetting] = useAtom(jotaiAtoms.voiceSetting);
  const [selectModel] = useAtom(jotaiAtoms.selectModel);

  const isRcVersion = selectModel?.version === "2.0.0-rc.0";

  return (
    <AccordionItem value="voiceSetting">
      <AccordionTrigger
        className="
        flex justify-between items-center
        bg-neutral-800 text-white
        px-4 py-3
        font-medium
        rounded-lg
        hover:bg-neutral-900
        active:bg-neutral-950
        transition-colors
      "
      >
        Voice Settings
      </AccordionTrigger>

      <AccordionContent
        className="
        bg-neutral-600 text-neutral-100
        px-4 py-3
        border-t border-neutral-500
        rounded-b-lg
        transition-all
        flex flex-col gap-3
      "
      >
        <SliderOption
          label="Pitch"
          description="声の高さを調節します。"
          value={voiceSetting.pitch}
          setValue={(v) => setVoiceSetting((prev) => ({ ...prev, pitch: v }))}
          min={-24}
          max={24}
          step={0.125}
        />
        <SliderOption
          label="Formant"
          description="フォルマントを調節します。"
          value={voiceSetting.formant}
          setValue={(v) => setVoiceSetting((prev) => ({ ...prev, formant: v }))}
          min={-2}
          max={2}
          step={0.5}
        />
        <SliderOption
          label="IntonationIntensity"
          description="抑揚の大きさを調節します。"
          value={voiceSetting.intonationIntensity}
          setValue={(v) =>
            setVoiceSetting((prev) => ({ ...prev, intonationIntensity: v }))
          }
          min={-1}
          max={3}
          step={0.1}
        />
        {isRcVersion ? (
          <SliderOption
            label="MinSourcePitch"
            description="認識する声の高さの最低値を調節します。"
            value={voiceSetting.minSourcePitch}
            setValue={(v) =>
              setVoiceSetting((prev) => ({ ...prev, minSourcePitch: v }))
            }
            min={33.125}
            max={88.875}
            step={0.125}
          />
        ) : (
          <></>
        )}
        {isRcVersion ? (
          <SliderOption
            label="MaxSourcePitch"
            description="認識する声の高さの最高値を調節します。"
            value={voiceSetting.maxSourcePitch}
            setValue={(v) =>
              setVoiceSetting((prev) => ({ ...prev, maxSourcePitch: v }))
            }
            min={33.125}
            max={88.875}
            step={0.125}
          />
        ) : (
          <></>
        )}
        {isRcVersion ? (
          <SliderOption
            label="VQNeighborCount"
            description="kNN-VC 的な処理の k の値。有効化すると話者類似性が向上するが、やや滑舌が悪化します。"
            value={voiceSetting.vqNeighborCount}
            setValue={(v) =>
              setVoiceSetting((prev) => ({ ...prev, vqNeighborCount: v }))
            }
            min={0}
            max={8}
            step={1}
          />
        ) : (
          <></>
        )}
      </AccordionContent>
    </AccordionItem>
  );
}

function OutputSettingAccordion() {
  const [outputSetting, setOutputSetting] = useAtom(jotaiAtoms.outputSetting);

  return (
    <AccordionItem value="outputSetting">
      <AccordionTrigger
        className="
        flex justify-between items-center
        bg-neutral-800 text-white
        px-4 py-3
        font-medium
        rounded-lg
        hover:bg-neutral-900
        active:bg-neutral-950
        transition-colors
      "
      >
        Output Settings
      </AccordionTrigger>

      <AccordionContent
        className="
        bg-neutral-600 text-neutral-100
        px-4 py-3
        border-t border-neutral-500
        rounded-b-lg
        transition-all
        flex flex-col gap-3
      "
      >
        <SliderOption
          label="InputGain"
          description="ボイスチェンジャーに入力する音声の大きさを調節します。"
          value={outputSetting.inputGain}
          setValue={(v) =>
            setOutputSetting((prev) => ({ ...prev, inputGain: v }))
          }
          min={0}
          max={10}
          step={0.1}
        />
        <SliderOption
          label="OutputGain"
          description="ボイスチェンジャーから出力する音声の大きさを調節します。"
          value={outputSetting.outputGain}
          setValue={(v) =>
            setOutputSetting((prev) => ({ ...prev, outputGain: v }))
          }
          min={0}
          max={10}
          step={0.1}
        />
        <SliderOption
          label="MonitorGain"
          description="モニターデバイスに出力する音声の大きさを調節します。"
          value={outputSetting.monitorGain}
          setValue={(v) =>
            setOutputSetting((prev) => ({ ...prev, monitorGain: v }))
          }
          min={0}
          max={10}
          step={0.1}
        />
        <SliderOption
          label="InputThreshold"
          description="一定以上音声が小さい場合、音声を変換しないしきい値を設定します。"
          value={outputSetting.inputThreshold}
          setValue={(v) =>
            setOutputSetting((prev) => ({ ...prev, inputThreshold: v }))
          }
          min={0}
          max={1}
          step={0.01}
        />
        <MicLevelMeter />
        <div className="" />
        <SelectDevice />
      </AccordionContent>
    </AccordionItem>
  );
}

function SliderSettings() {
  return (
    <div className="flex flex-col">
      <Accordion
        type="multiple"
        className="
          w-full mx-auto
          bg-neutral-700 rounded-xl
          shadow-lg border border-neutral-600
        "
      >
        <VoiceAccordion />
        <OutputSettingAccordion />
      </Accordion>
    </div>
  );
}

function ModelInfo() {
  const [showImgDescription, setShowImgDescription] = useState(false);
  const [selectModel] = useAtom(jotaiAtoms.selectModel);
  const [selectSpeakerIdx, setSelectSpeakerIdx] = useAtom(
    jotaiAtoms.selectSpeakerIdx,
  );

  let fixedImgSrc: string | null = null;
  if (
    selectModel !== null &&
    selectModel.voices[selectSpeakerIdx].portrait_path !== null
  ) {
    fixedImgSrc = tauriCore.convertFileSrc(
      `${selectModel.model_path}/${selectModel.voices[selectSpeakerIdx].portrait_path}`,
    );
  }

  return (
    <div className="flex min-h-70 w-full">
      <div className="flex flex-col w-60">
        <Select
          onValueChange={(v) => {
            setSelectSpeakerIdx(parseInt(v));
          }}
          value={selectSpeakerIdx.toString()}
        >
          <SelectTrigger className="w-[90%] border-neutral-500 text-neutral-200">
            <SelectValue placeholder={"Select Speaker"}></SelectValue>
          </SelectTrigger>
          <SelectContent>
            {selectModel?.voices.map((i, idx) => (
              <SelectItem value={`${idx}`}>{i.name}</SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Card
          onMouseEnter={() => setShowImgDescription(true)}
          onMouseLeave={() => setShowImgDescription(false)}
          className="w-48 h-48 m-3 mt-6 bg-neutral-600 border-neutral-500 hover:brightness-85"
        >
          {fixedImgSrc && (
            <img
              src={fixedImgSrc}
              alt="Speaker"
              className="rounded-xl w-full h-full object-cover"
            />
          )}
        </Card>
      </div>

      <Card className="max-h-70 overflow-y-scroll whitespace-pre-wrap  text-neutral-300 text-sm no-scrollbar p-5 bg-neutral-600 flex-1 h-full border-neutral-500">
        {showImgDescription
          ? `${selectModel?.voices[selectSpeakerIdx].portrait_description ?? ""}`
          : `${selectModel?.voices[selectSpeakerIdx].description ?? ""}`}
      </Card>
    </div>
  );
}

export function VoiceSettings() {
  return (
    <TooltipProvider>
      <div className="flex flex-col gap-5 flex-1 bg-neutral-800 p-5 m-5 ml-0 rounded-xl overflow-y-scroll no-scrollbar">
        <SliderSettings />
        <ModelInfo />
      </div>
    </TooltipProvider>
  );
}
