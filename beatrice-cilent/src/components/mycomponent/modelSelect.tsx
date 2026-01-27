import { Button } from "@/components/ui/button";
import { jotaiAtoms } from "@/jotaiAtoms";
import { BeatriceModelInfo, rustInvoke } from "@/rustInvoke";
import * as tauriCore from "@tauri-apps/api/core";
import * as tauriDialog from "@tauri-apps/plugin-dialog";
import { useAtom } from "jotai";
import { Tooltip, TooltipContent, TooltipTrigger } from "../ui/tooltip";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";

function ModelCard({ model }: { model: BeatriceModelInfo }) {
  const [loadedModels, setLoadedModels] = useAtom(jotaiAtoms.loadedModels);
  const [selectModel, setSelectModel] = useAtom(jotaiAtoms.selectModel);
  const [, selectSpeakerIdx] = useAtom(jotaiAtoms.selectSpeakerIdx);

  const imgSrc = `${model.model_path}/${model.voices[0].portrait_path}`;
  const fixedImgSrc = tauriCore.convertFileSrc(imgSrc);

  let borderCss;
  if (model.model_path === selectModel?.model_path) {
    borderCss = "border-3";
  } else {
    borderCss = "";
  }

  return (
    <ContextMenu>
      <Tooltip>
        <TooltipTrigger asChild>
          <ContextMenuTrigger>
            <Button
              onClick={() => {
                setSelectModel(model);
                selectSpeakerIdx(0);
              }}
              className="w-17 h-17 bg-neutral-700 group"
            >
              {fixedImgSrc && (
                <img
                  className={`w-full h-full rounded-md object-contain
                  group-hover:brightness-80 group-active:brightness-60
                  ${borderCss}`}
                  src={fixedImgSrc}
                ></img>
              )}
            </Button>
          </ContextMenuTrigger>
        </TooltipTrigger>

        <TooltipContent
          className="text-sm whitespace-pre-wrap max-w-150"
          side="right"
          align="center"
        >
          {`モデル名: ${model.name}\nフォルダ名: ${model.model_path.split("\\").pop()}\n\n${model.description}`}
        </TooltipContent>

        <ContextMenuContent>
          <ContextMenuItem
            onSelect={() => {
              setLoadedModels((prev) =>
                prev.filter((m) => m.model_path !== model.model_path),
              );

              if (selectModel?.model_path === model.model_path) {
                setSelectModel(null);
              }
            }}
          >
            Delete
          </ContextMenuItem>
        </ContextMenuContent>
      </Tooltip>
    </ContextMenu>
  );
}

function ModelAddCard() {
  const [loadedModels, setLoadedModels] = useAtom(jotaiAtoms.loadedModels);

  return (
    <Button
      onClick={() => {
        const promise = async () => {
          const path = await tauriDialog.open({
            multiple: false,
            directory: true,
          });
          if (path === null) return;

          const modelinfo = await rustInvoke.beatrice.getModelFromPath(path);
          if (modelinfo === null) return;

          const loadedModelPaths = loadedModels.map((i) => i.model_path);
          if (loadedModelPaths.includes(modelinfo.model_path)) return;

          setLoadedModels((prev) => [...prev, modelinfo]);
        };
        promise();
      }}
      className="w-17 h-17 bg-neutral-700 text-3xl hover:brightness-80 text-neutral-300"
    >
      +
    </Button>
  );
}

export function SelectModel() {
  const [loadedModels] = useAtom(jotaiAtoms.loadedModels);

  return (
    <div className="grid grid-cols-2 auto-rows-min gap-3 p-4 m-5 w-45 h-[92%] bg-neutral-800 rounded-xl overflow-auto no-scrollbar shadow-2xl">
      <ModelAddCard key="ModelAddCard" />

      {loadedModels.map((i) => (
        <ModelCard key={i.model_path} model={i} />
      ))}
    </div>
  );
}
